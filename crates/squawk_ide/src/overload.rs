use rustc_hash::FxHashSet;
use smallvec::{SmallVec, smallvec};

use crate::{
    interner::Name,
    ir::{
        CastContext, CastId, CastTypes, Function, FunctionId, FunctionKind, Operand, Operator,
        OperatorId, Param, ParamMode, Resolution, Ty, TypeCategory as Category, TypeId, TypeKind,
    },
    semantic_index::IndexData,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum ArgType {
    Known(Ty),
    UnknownLiteral,
    #[default]
    Untyped,
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct CallArg {
    pub(crate) name: Option<Name>,
    pub(crate) ty: ArgType,
    pub(crate) variadic: bool,
}

impl CallArg {
    pub(crate) fn positional(ty: ArgType) -> CallArg {
        CallArg {
            ty,
            ..CallArg::default()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Match {
    Exact,
    DomainBase,
    Implicit,
    Polymorphic,
    UnknownLiteral,
    Uncertain,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pseudo {
    Any,
    AnyElement,
    AnyArray,
    AnyNonArray,
    AnyEnum,
    AnyRange,
    AnyMultirange,
    AnyCompatible,
    AnyCompatibleArray,
    AnyCompatibleNonArray,
    AnyCompatibleRange,
    AnyCompatibleMultirange,
    Record,
}

#[derive(Debug, Clone, Copy, Default)]
struct Polymorphic {
    element: Option<Ty>,
    array: Option<Ty>,
    range: Option<Ty>,
    multirange: Option<Ty>,
    compatible: Option<Ty>,
    compatible_range: Option<Ty>,
    compatible_multirange: Option<Ty>,
    uncertain: bool,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct BuiltinTypes {
    anyelement: Option<TypeId>,
    anycompatible: Option<TypeId>,
    text: Option<TypeId>,
}

impl BuiltinTypes {
    pub(crate) fn new(index: &mut IndexData, pg_catalog: Name) -> BuiltinTypes {
        let mut find = |name: &str| {
            let name = index.names.intern(name);
            index.types.named(pg_catalog, name).next()
        };
        BuiltinTypes {
            anyelement: find("anyelement"),
            anycompatible: find("anycompatible"),
            text: find("text"),
        }
    }
}

struct Candidate<Id> {
    id: Id,
    params: SmallVec<[Option<Ty>; 4]>,
    matches: SmallVec<[Match; 4]>,
}

pub(crate) struct Overloads<'a> {
    index: &'a IndexData,
    pg_catalog: Name,
    builtin_types: BuiltinTypes,
    dropped_casts: &'a FxHashSet<CastId>,
}

impl<'a> Overloads<'a> {
    pub(crate) fn new(
        index: &'a IndexData,
        pg_catalog: Name,
        builtin_types: BuiltinTypes,
        dropped_casts: &'a FxHashSet<CastId>,
    ) -> Overloads<'a> {
        Overloads {
            index,
            pg_catalog,
            builtin_types,
            dropped_casts,
        }
    }

    pub(crate) fn select(
        &self,
        candidates: impl Iterator<Item = FunctionId>,
        args: &[CallArg],
    ) -> Resolution {
        let mut viable: SmallVec<[Candidate<FunctionId>; 4]> = SmallVec::new();
        for id in candidates {
            let function = &self.index.functions[id];
            let Some(candidate) = self
                .bind(function, args)
                .and_then(|params| self.candidate(id, params, args))
            else {
                continue;
            };
            if viable
                .iter()
                .any(|other| self.same_signature(&self.index.functions[other.id], function))
            {
                continue;
            }
            viable.push(candidate);
        }
        self.choose(viable, args, Resolution::Function)
    }

    pub(crate) fn select_operator(
        &self,
        candidates: impl Iterator<Item = (OperatorId, &'a Operator)>,
        left: Option<ArgType>,
        right: Option<ArgType>,
    ) -> Resolution {
        let args: SmallVec<[CallArg; 2]> = [left, right]
            .into_iter()
            .flatten()
            .map(|ty| CallArg {
                ty,
                ..CallArg::default()
            })
            .collect();
        let assumed = match (left, right) {
            (Some(ArgType::Known(ty)), Some(ArgType::UnknownLiteral))
            | (Some(ArgType::UnknownLiteral), Some(ArgType::Known(ty))) => Some(ty),
            (Some(_) | None, Some(_) | None) => None,
        };
        let assumed_base = assumed
            .and_then(|ty| self.index.base_type(ty))
            .filter(|base| Some(*base) != assumed);
        let mut exact = None;
        let mut exact_base = None;
        let mut viable: SmallVec<[Candidate<OperatorId>; 4]> = SmallVec::new();
        for (id, operator) in candidates {
            if (operator.left == Operand::None) != left.is_none()
                || (operator.right == Operand::None) != right.is_none()
            {
                continue;
            }
            let both = |ty: Option<Ty>| {
                ty.is_some_and(|ty| {
                    operator.left == Operand::Ty(ty) && operator.right == Operand::Ty(ty)
                })
            };
            if exact.is_none() && both(assumed) {
                exact = Some(id);
            }
            if exact_base.is_none() && both(assumed_base) {
                exact_base = Some(id);
            }
            let params = [operator.left, operator.right]
                .into_iter()
                .filter_map(|operand| match operand {
                    Operand::None => None,
                    Operand::Unknown => Some(None),
                    Operand::Ty(ty) => Some(Some(ty)),
                })
                .collect();
            let Some(candidate) = self.candidate(id, params, &args) else {
                continue;
            };
            if viable.iter().any(|other| {
                let other = &self.index.operators[other.id];
                other.left == operator.left && other.right == operator.right
            }) {
                continue;
            }
            viable.push(candidate);
        }
        if let Some(id) = exact.or(exact_base) {
            return Resolution::Operator(id);
        }
        self.choose(viable, &args, Resolution::Operator)
    }

    fn choose<Id: Copy>(
        &self,
        mut viable: SmallVec<[Candidate<Id>; 4]>,
        args: &[CallArg],
        resolution: impl Fn(Id) -> Resolution,
    ) -> Resolution {
        if viable.is_empty() {
            return Resolution::Unresolved;
        }
        if viable
            .iter()
            .any(|candidate| candidate.matches.iter().all(|m| *m == Match::Exact))
        {
            viable.retain(|candidate| candidate.matches.iter().all(|m| *m == Match::Exact));
        }
        retain_max(&mut viable, |candidate| {
            candidate
                .matches
                .iter()
                .filter(|m| matches!(m, Match::Exact | Match::DomainBase))
                .count()
        });
        retain_max(&mut viable, |candidate| {
            candidate
                .matches
                .iter()
                .zip(&candidate.params)
                .filter(|(m, param)| {
                    **m == Match::Implicit && param.is_some_and(|param| self.is_preferred(param))
                })
                .count()
        });
        self.resolve_unknown_literals(&mut viable, args);
        self.assume_known_type(&mut viable, args);
        match viable.as_slice() {
            [candidate] => resolution(candidate.id),
            _ if viable
                .iter()
                .any(|candidate| candidate.matches.contains(&Match::Uncertain)) =>
            {
                Resolution::Unknown
            }
            _ => Resolution::Ambiguous,
        }
    }

    pub(crate) fn accepts_args(&self, function: &Function, args: &[CallArg]) -> bool {
        self.bind(function, args).is_some_and(|params| {
            args.iter()
                .zip(&params)
                .all(|(arg, param)| self.arg_match(arg.ty, *param).is_some())
                && self.solve(&params, args).is_some()
        })
    }

    pub(crate) fn missing_array_type(&self, function: &Function, args: &[CallArg]) -> Option<Ty> {
        let params = self.bind(function, args)?;
        let solution = self.solve(&params, args)?;
        if solution.uncertain {
            return None;
        }
        let inputs: SmallVec<[&Param; 8]> = self.inputs(function).collect();
        let variadic = inputs
            .last()
            .filter(|param| param.mode == ParamMode::Variadic)
            .and_then(|param| param.ty)
            .and_then(|ty| self.pseudo(ty))
            .filter(|_| {
                args.iter()
                    .enumerate()
                    .any(|(i, arg)| arg.name.is_none() && !arg.variadic && i + 1 >= inputs.len())
            });
        let ret = function.ret.and_then(|ty| self.pseudo(ty));
        [
            (Pseudo::AnyArray, solution.element),
            (Pseudo::AnyCompatibleArray, solution.compatible),
        ]
        .into_iter()
        .filter(|(pseudo, _)| variadic == Some(*pseudo) || ret == Some(*pseudo))
        .find_map(|(_, element)| element.filter(|element| self.is_array(*element)))
    }

    pub(crate) fn return_type(&self, function: &Function, args: &[CallArg]) -> Option<Ty> {
        let ret = function.ret?;
        let Some(pseudo) = self.pseudo(ret) else {
            return Some(ret);
        };
        let params = self.bind(function, args)?;
        let solution = self.solve(&params, args)?;
        if solution.uncertain {
            return None;
        }
        let compatible = || {
            solution
                .compatible
                .or_else(|| self.builtin_types.text.map(Ty::Type))
        };
        match pseudo {
            Pseudo::Any | Pseudo::Record => Some(ret),
            Pseudo::AnyElement | Pseudo::AnyNonArray | Pseudo::AnyEnum => solution.element,
            Pseudo::AnyArray => solution.array.or_else(|| self.array_of(solution.element?)),
            Pseudo::AnyRange => solution.range,
            Pseudo::AnyMultirange => solution
                .multirange
                .or_else(|| self.multirange_of(solution.range?)),
            Pseudo::AnyCompatible | Pseudo::AnyCompatibleNonArray => compatible(),
            Pseudo::AnyCompatibleArray => self.array_of(compatible()?),
            Pseudo::AnyCompatibleRange => solution.compatible_range,
            Pseudo::AnyCompatibleMultirange => solution
                .compatible_multirange
                .or_else(|| self.multirange_of(solution.compatible_range?)),
        }
    }

    fn candidate<Id>(
        &self,
        id: Id,
        params: SmallVec<[Option<Ty>; 4]>,
        args: &[CallArg],
    ) -> Option<Candidate<Id>> {
        let matches = args
            .iter()
            .zip(&params)
            .map(|(arg, param)| self.arg_match(arg.ty, *param))
            .collect::<Option<_>>()?;
        self.solve(&params, args)?;
        Some(Candidate {
            id,
            params,
            matches,
        })
    }

    fn inputs(&self, function: &Function) -> impl Iterator<Item = &'a Param> + use<'a> {
        let params = &self.index.params;
        let procedure = function.kind == FunctionKind::Procedure;
        function
            .params
            .iter()
            .map(move |id| &params[id])
            .filter(move |param| param.mode != ParamMode::Out || procedure)
    }

    fn same_signature(&self, a: &Function, b: &Function) -> bool {
        self.inputs(a)
            .map(|param| param.ty)
            .eq(self.inputs(b).map(|param| param.ty))
    }

    fn bind(&self, function: &Function, args: &[CallArg]) -> Option<SmallVec<[Option<Ty>; 4]>> {
        let inputs: SmallVec<[&Param; 8]> = self.inputs(function).collect();
        let variadic = inputs
            .last()
            .is_some_and(|param| param.mode == ParamMode::Variadic);
        let fixed = inputs.len() - usize::from(variadic);
        let mut assigned: SmallVec<[bool; 8]> = smallvec![false; inputs.len()];
        let mut params = SmallVec::with_capacity(args.len());
        let mut seen_named = false;
        for (i, arg) in args.iter().enumerate() {
            let position = match arg.name {
                Some(name) => {
                    seen_named = true;
                    let position = inputs.iter().position(|param| param.name == Some(name))?;
                    if assigned[position] {
                        return None;
                    }
                    position
                }
                None if seen_named => return None,
                None if i < fixed => i,
                None if variadic => fixed,
                None => return None,
            };
            let param = inputs[position];
            if arg.variadic && (param.mode != ParamMode::Variadic || assigned[position]) {
                return None;
            }
            assigned[position] = true;
            let ty = match (param.mode, arg.name, arg.variadic) {
                (ParamMode::Variadic, None, false) => {
                    param.ty.and_then(|ty| self.variadic_element(ty))
                }
                (ParamMode::In | ParamMode::Out | ParamMode::InOut | ParamMode::Variadic, _, _) => {
                    param.ty
                }
            };
            params.push(ty);
        }
        inputs
            .iter()
            .zip(&assigned)
            .all(|(param, assigned)| *assigned || param.has_default)
            .then_some(params)
    }

    fn variadic_element(&self, ty: Ty) -> Option<Ty> {
        match self.pseudo(ty) {
            Some(Pseudo::AnyArray) => self.builtin_types.anyelement.map(Ty::Type),
            Some(Pseudo::AnyCompatibleArray) => self.builtin_types.anycompatible.map(Ty::Type),
            Some(
                Pseudo::Any
                | Pseudo::AnyElement
                | Pseudo::AnyNonArray
                | Pseudo::AnyEnum
                | Pseudo::AnyRange
                | Pseudo::AnyMultirange
                | Pseudo::AnyCompatible
                | Pseudo::AnyCompatibleNonArray
                | Pseudo::AnyCompatibleRange
                | Pseudo::AnyCompatibleMultirange
                | Pseudo::Record,
            )
            | None => Some(ty.element().unwrap_or(ty)),
        }
    }

    fn arg_match(&self, arg: ArgType, param: Option<Ty>) -> Option<Match> {
        let Some(param) = param else {
            return Some(Match::Uncertain);
        };
        match arg {
            ArgType::Known(arg) => self.coerce(arg, param),
            ArgType::UnknownLiteral => Some(Match::UnknownLiteral),
            ArgType::Untyped => Some(Match::Uncertain),
        }
    }

    fn coerce(&self, arg: Ty, param: Ty) -> Option<Match> {
        if arg == param {
            return Some(Match::Exact);
        }
        if self.pseudo(arg).is_some() {
            return Some(Match::Uncertain);
        }
        let Some(arg_base) = self.index.base_type(arg) else {
            return Some(Match::Uncertain);
        };
        if let Some(pseudo) = self.pseudo(param) {
            let accepts = match pseudo {
                Pseudo::Any | Pseudo::AnyElement | Pseudo::AnyCompatible => true,
                Pseudo::AnyArray | Pseudo::AnyCompatibleArray => {
                    self.index.element_type(arg_base).is_some()
                }
                Pseudo::AnyNonArray | Pseudo::AnyCompatibleNonArray => {
                    self.index.element_type(arg_base).is_none()
                }
                Pseudo::AnyEnum => self.type_kind(arg_base) == Some(TypeKind::Enum),
                Pseudo::AnyRange | Pseudo::AnyCompatibleRange => {
                    self.type_kind(arg_base) == Some(TypeKind::Range)
                }
                Pseudo::AnyMultirange | Pseudo::AnyCompatibleMultirange => {
                    self.type_kind(arg_base) == Some(TypeKind::Multirange)
                }
                Pseudo::Record => self.category(arg_base) == Some(Category::Composite),
            };
            return accepts.then_some(Match::Polymorphic);
        }
        let Some(param_base) = self.index.base_type(param) else {
            return Some(Match::Uncertain);
        };
        if arg_base == param_base {
            return Some(if param == param_base {
                Match::DomainBase
            } else {
                Match::Implicit
            });
        }
        if let Some(cast) = self
            .index
            .casts
            .get(CastTypes {
                source: arg_base,
                target: param_base,
            })
            .find(|cast| !self.dropped_casts.contains(cast))
        {
            return (self.index.casts[cast].context == CastContext::Implicit)
                .then_some(Match::Implicit);
        }
        if matches!(
            self.builtin_name(param_base),
            Some("int2vector" | "oidvector")
        ) {
            return None;
        }
        let arg_element = self.index.element_type(arg_base)?;
        let param_element = self.index.element_type(param_base)?;
        match self.coerce(arg_element, param_element)? {
            Match::Exact | Match::DomainBase | Match::Implicit => Some(Match::Implicit),
            Match::Uncertain => Some(Match::Uncertain),
            Match::Polymorphic | Match::UnknownLiteral => None,
        }
    }

    fn solve(&self, params: &[Option<Ty>], args: &[CallArg]) -> Option<Polymorphic> {
        let mut solution = Polymorphic::default();
        let mut nonarray = false;
        let mut is_enum = false;
        let mut compatible_nonarray = false;
        let mut compatible: SmallVec<[CompatibleType; 4]> = SmallVec::new();
        for (arg, param) in args.iter().zip(params) {
            let Some(pseudo) = param
                .and_then(|param| self.pseudo(param))
                .filter(|pseudo| !matches!(pseudo, Pseudo::Any | Pseudo::Record))
            else {
                continue;
            };
            nonarray |= pseudo == Pseudo::AnyNonArray;
            is_enum |= pseudo == Pseudo::AnyEnum;
            compatible_nonarray |= pseudo == Pseudo::AnyCompatibleNonArray;
            let actual = match arg.ty {
                ArgType::Known(ty) if self.pseudo(ty).is_none() => ty,
                ArgType::UnknownLiteral => continue,
                ArgType::Known(_) | ArgType::Untyped => {
                    solution.uncertain = true;
                    continue;
                }
            };
            let Some(base) = self.index.base_type(actual) else {
                solution.uncertain = true;
                continue;
            };
            match pseudo {
                Pseudo::AnyElement | Pseudo::AnyNonArray | Pseudo::AnyEnum => {
                    unify(&mut solution.element, actual)?;
                }
                Pseudo::AnyArray => unify(&mut solution.array, base)?,
                Pseudo::AnyRange => unify(&mut solution.range, base)?,
                Pseudo::AnyMultirange => unify(&mut solution.multirange, base)?,
                Pseudo::AnyCompatible | Pseudo::AnyCompatibleNonArray => {
                    push_compatible(&mut compatible, actual, base);
                }
                Pseudo::AnyCompatibleArray => {
                    let element = self.index.element_type(base)?;
                    let Some(element_base) = self.index.base_type(element) else {
                        solution.uncertain = true;
                        continue;
                    };
                    push_compatible(&mut compatible, element, element_base);
                }
                Pseudo::AnyCompatibleRange => unify(&mut solution.compatible_range, base)?,
                Pseudo::AnyCompatibleMultirange => {
                    unify(&mut solution.compatible_multirange, base)?;
                }
                Pseudo::Any | Pseudo::Record => {}
            }
        }
        if let Some(multirange) = solution.multirange {
            match self.range_of(multirange) {
                Some(range) => unify(&mut solution.range, range)?,
                None => solution.uncertain = true,
            }
        }
        if let Some(array) = solution.array {
            unify(&mut solution.element, self.index.element_type(array)?)?;
        }
        if let Some(range) = solution.range {
            match self.subtype_of(range) {
                Some(subtype) => unify(&mut solution.element, subtype)?,
                None => solution.uncertain = true,
            }
        }
        if let Some(element) = solution.element
            && ((nonarray && self.is_array(element)) || (is_enum && !self.is_enum(element)))
        {
            return None;
        }
        if let Some(multirange) = solution.compatible_multirange {
            match self.range_of(multirange) {
                Some(range) => unify(&mut solution.compatible_range, range)?,
                None => solution.uncertain = true,
            }
        }
        let target = match solution.compatible_range {
            Some(range) => {
                let subtype = self.subtype_of(range);
                solution.uncertain |= subtype.is_none();
                subtype
            }
            None if compatible.is_empty() => None,
            None => Some(self.common_type(&compatible)?),
        };
        if let Some(target) = target {
            for compatible in &compatible {
                match self.coerce(compatible.actual, target)? {
                    Match::Exact | Match::DomainBase | Match::Implicit => {}
                    Match::Uncertain => solution.uncertain = true,
                    Match::Polymorphic | Match::UnknownLiteral => return None,
                }
            }
            if compatible_nonarray && self.is_array(target) {
                return None;
            }
        }
        solution.compatible = target;
        Some(solution)
    }

    pub(crate) fn select_common_type(&self, left: Ty, right: Ty) -> Option<Ty> {
        let left = CompatibleType {
            actual: left,
            base: self.index.base_type(left)?,
        };
        let right = CompatibleType {
            actual: right,
            base: self.index.base_type(right)?,
        };
        self.common_type(&[left, right])
    }

    fn common_type(&self, types: &[CompatibleType]) -> Option<Ty> {
        let first = *types.first()?;
        if types.iter().all(|ty| ty.actual == first.actual) {
            return Some(first.actual);
        }
        let mut common = first.base;
        let mut category = self.category(common);
        let mut preferred = self.is_preferred(common);
        for &CompatibleType { base, .. } in &types[1..] {
            if base == common {
                continue;
            }
            let base_category = self.category(base);
            if base_category != category {
                return None;
            }
            if !preferred
                && self.coerces_implicitly(common, base)
                && !self.coerces_implicitly(base, common)
            {
                common = base;
                category = base_category;
                preferred = self.is_preferred(base);
            }
        }
        Some(common)
    }

    fn coerces_implicitly(&self, from: Ty, to: Ty) -> bool {
        matches!(
            self.coerce(from, to),
            Some(Match::Exact | Match::DomainBase | Match::Implicit)
        )
    }

    fn is_array(&self, ty: Ty) -> bool {
        self.index
            .base_type(ty)
            .is_some_and(|base| self.index.element_type(base).is_some())
    }

    fn is_enum(&self, ty: Ty) -> bool {
        self.index
            .base_type(ty)
            .is_some_and(|base| self.type_kind(base) == Some(TypeKind::Enum))
    }

    fn array_of(&self, element: Ty) -> Option<Ty> {
        (!self.is_array(element)).then(|| element.array())
    }

    fn multirange_of(&self, range: Ty) -> Option<Ty> {
        let Ty::Type(range) = range else {
            return None;
        };
        self.index
            .types
            .with_range(range)
            .find(|id| self.index.types[*id].kind == TypeKind::Multirange)
            .map(Ty::Type)
    }

    fn subtype_of(&self, range: Ty) -> Option<Ty> {
        match range {
            Ty::Type(id) => self.index.types[id].subtype,
            Ty::Table(_) | Ty::Cte(_) | Ty::Array(_) => None,
        }
    }

    fn range_of(&self, multirange: Ty) -> Option<Ty> {
        match multirange {
            Ty::Type(id) => self.index.types[id].range.map(Ty::Type),
            Ty::Table(_) | Ty::Cte(_) | Ty::Array(_) => None,
        }
    }

    fn resolve_unknown_literals<Id>(
        &self,
        viable: &mut SmallVec<[Candidate<Id>; 4]>,
        args: &[CallArg],
    ) {
        if viable.len() < 2 {
            return;
        }
        for (position, _) in args
            .iter()
            .enumerate()
            .filter(|(_, arg)| arg.ty == ArgType::UnknownLiteral)
        {
            let category_at = |candidate: &Candidate<Id>| {
                candidate.params[position].and_then(|param| self.category(param))
            };
            let mut categories = viable.iter().map(category_at);
            let first = categories.next().flatten();
            let category = if viable
                .iter()
                .any(|candidate| category_at(candidate) == Some(Category::String))
            {
                Category::String
            } else if let Some(first) = first
                && categories.all(|category| category == Some(first))
            {
                first
            } else {
                continue;
            };
            let has_preferred = viable.iter().any(|candidate| {
                category_at(candidate) == Some(category)
                    && candidate.params[position].is_some_and(|param| self.is_preferred(param))
            });
            viable.retain(|candidate| {
                category_at(candidate) == Some(category)
                    && (!has_preferred
                        || candidate.params[position].is_some_and(|param| self.is_preferred(param)))
            });
        }
    }

    fn assume_known_type<Id>(&self, viable: &mut SmallVec<[Candidate<Id>; 4]>, args: &[CallArg]) {
        if viable.len() < 2 || !args.iter().any(|arg| arg.ty == ArgType::UnknownLiteral) {
            return;
        }
        let mut known = None;
        for arg in args {
            let ty = match arg.ty {
                ArgType::UnknownLiteral => continue,
                ArgType::Untyped => return,
                ArgType::Known(ty) => ty,
            };
            let Some(base) = self.index.base_type(ty) else {
                return;
            };
            if known.is_some_and(|known| known != base) {
                return;
            }
            known = Some(base);
        }
        let Some(known) = known else {
            return;
        };
        let assumed: SmallVec<[CallArg; 4]> = args
            .iter()
            .map(|arg| CallArg {
                ty: ArgType::Known(known),
                ..*arg
            })
            .collect();
        let mut coercible = viable.iter().enumerate().filter(|(_, candidate)| {
            assumed
                .iter()
                .zip(&candidate.params)
                .all(|(arg, param)| self.arg_match(arg.ty, *param).is_some())
                && self.solve(&candidate.params, &assumed).is_some()
        });
        if let (Some((index, _)), None) = (coercible.next(), coercible.next()) {
            viable.swap(0, index);
            viable.truncate(1);
        }
    }

    fn type_kind(&self, ty: Ty) -> Option<TypeKind> {
        match ty {
            Ty::Type(id) => Some(self.index.types[id].kind),
            Ty::Table(_) | Ty::Cte(_) | Ty::Array(_) => None,
        }
    }

    fn builtin_name(&self, ty: Ty) -> Option<&'a str> {
        let Ty::Type(id) = ty else {
            return None;
        };
        let ty = &self.index.types[id];
        (ty.schema == self.pg_catalog).then(|| self.index.names.resolve(ty.name))
    }

    fn pseudo(&self, ty: Ty) -> Option<Pseudo> {
        let Ty::Type(id) = ty else {
            return None;
        };
        if self.index.types[id].category != Some(Category::Pseudo) {
            return None;
        }
        Some(match self.builtin_name(ty)? {
            "any" => Pseudo::Any,
            "anyelement" => Pseudo::AnyElement,
            "anyarray" => Pseudo::AnyArray,
            "anynonarray" => Pseudo::AnyNonArray,
            "anyenum" => Pseudo::AnyEnum,
            "anyrange" => Pseudo::AnyRange,
            "anymultirange" => Pseudo::AnyMultirange,
            "anycompatible" => Pseudo::AnyCompatible,
            "anycompatiblearray" => Pseudo::AnyCompatibleArray,
            "anycompatiblenonarray" => Pseudo::AnyCompatibleNonArray,
            "anycompatiblerange" => Pseudo::AnyCompatibleRange,
            "anycompatiblemultirange" => Pseudo::AnyCompatibleMultirange,
            "record" => Pseudo::Record,
            _ => return None,
        })
    }

    fn category(&self, ty: Ty) -> Option<Category> {
        let id = match ty {
            Ty::Array(_) => return Some(Category::Array),
            Ty::Table(_) | Ty::Cte(_) => return Some(Category::Composite),
            Ty::Type(id) => id,
        };
        let ty = &self.index.types[id];
        if let Some(category) = ty.category {
            return Some(category);
        }
        match ty.kind {
            TypeKind::Shell => Some(Category::Pseudo),
            TypeKind::Enum => Some(Category::Enum),
            TypeKind::Range | TypeKind::Multirange => Some(Category::Range),
            TypeKind::Domain => ty.base.and_then(|base| self.category(base)),
            TypeKind::Type if ty.columns.is_some() => Some(Category::Composite),
            TypeKind::Type => Some(Category::User),
        }
    }

    fn is_preferred(&self, ty: Ty) -> bool {
        matches!(ty, Ty::Type(id) if self.index.types[id].preferred)
    }
}

fn retain_max<Id>(
    viable: &mut SmallVec<[Candidate<Id>; 4]>,
    score: impl Fn(&Candidate<Id>) -> usize,
) {
    if viable.len() < 2 {
        return;
    }
    let Some(max) = viable.iter().map(&score).max() else {
        return;
    };
    viable.retain(|candidate| score(candidate) == max);
}

fn unify(slot: &mut Option<Ty>, ty: Ty) -> Option<()> {
    match *slot {
        Some(existing) if existing != ty => None,
        Some(_) | None => {
            *slot = Some(ty);
            Some(())
        }
    }
}

#[derive(Clone, Copy)]
struct CompatibleType {
    actual: Ty,
    base: Ty,
}

fn push_compatible(types: &mut SmallVec<[CompatibleType; 4]>, actual: Ty, base: Ty) {
    if !types.iter().any(|ty| ty.actual == actual) {
        types.push(CompatibleType { actual, base });
    }
}
