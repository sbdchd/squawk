use std::collections::hash_map::Entry;

use rustc_hash::{FxHashMap, FxHashSet};
use smallvec::SmallVec;

use crate::column_exprs::ColumnExprId;
use crate::interner::Name;
use crate::ir::{
    AccessMethodId, CastId, CollationId, ConstraintId, ConversionId, EventTriggerId, ExtensionId,
    ForeignDataWrapperId, FunctionId, IndexId, Item, LanguageId, OperatorClassId, OperatorFamilyId,
    OperatorFamilyMemberId, OperatorId, PolicyId, PublicationId, RuleId, SchemaId, ServerId,
    StatisticsId, TableId, TextSearchId, TransformId, TriggerId, TypeId,
};
use crate::semantic_index::IndexData;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum Object {
    Function(FunctionId),
    ColumnExpr(ColumnExprId),
    OperatorFamilyMember(OperatorFamilyMemberId),
    Table(TableId),
    Type(TypeId),
    Trigger(TriggerId),
    EventTrigger(EventTriggerId),
    Index(IndexId),
    Constraint(ConstraintId),
    Policy(PolicyId),
    Rule(RuleId),
    Cast(CastId),
    Transform(TransformId),
    Operator(OperatorId),
    AccessMethod(AccessMethodId),
    Language(LanguageId),
    ForeignDataWrapper(ForeignDataWrapperId),
    Conversion(ConversionId),
    TextSearch(TextSearchId),
    Collation(CollationId),
    OperatorFamily(OperatorFamilyId),
    OperatorClass(OperatorClassId),
    Statistics(StatisticsId),
    Extension(ExtensionId),
    Schema(SchemaId),
    Server(ServerId),
    Publication(PublicationId),
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Dependency {
    dependent: Object,
    referenced: Object,
}

#[derive(Default)]
pub(crate) struct DependencyGraph {
    edges: FxHashSet<Dependency>,
    by_referenced: FxHashMap<Object, SmallVec<[Object; 2]>>,
    moved_from: FxHashMap<Object, SmallVec<[Object; 1]>>,
    moved_to: FxHashMap<Object, SmallVec<[Object; 1]>>,
    replaced: FxHashMap<FunctionId, SmallVec<[FunctionId; 1]>>,
}

impl DependencyGraph {
    pub(crate) fn add_dependency(&mut self, dependent: Object, referenced: Object) {
        if !self.edges.insert(Dependency {
            dependent,
            referenced,
        }) {
            return;
        }
        self.by_referenced
            .entry(referenced)
            .or_default()
            .push(dependent);
    }

    pub(crate) fn remove_dependency(&mut self, dependent: Object, referenced: Object) -> bool {
        if !self.edges.remove(&Dependency {
            dependent,
            referenced,
        }) {
            return false;
        }
        let Entry::Occupied(mut entry) = self.by_referenced.entry(referenced) else {
            return false;
        };
        let Some(position) = entry
            .get()
            .iter()
            .position(|&existing| existing == dependent)
        else {
            return false;
        };
        entry.get_mut().remove(position);
        if entry.get().is_empty() {
            entry.remove();
        }
        true
    }

    pub(crate) fn add_move(&mut self, from: Object, to: Object) {
        debug_assert!(from < to);
        self.moved_from.entry(to).or_default().push(from);
        self.moved_to.entry(from).or_default().push(to);
    }

    pub(crate) fn add_replacement(&mut self, old: FunctionId, new: FunctionId) {
        self.replaced.entry(new).or_default().push(old);
    }

    pub(crate) fn dependents_of(&self, referenced: Object) -> &[Object] {
        self.by_referenced
            .get(&referenced)
            .map_or(&[], |dependents| dependents.as_slice())
    }

    pub(crate) fn moved_from(&self, to: Object) -> &[Object] {
        self.moved_from.get(&to).map_or(&[], |from| from.as_slice())
    }

    pub(crate) fn moved_to(&self, from: Object) -> &[Object] {
        self.moved_to.get(&from).map_or(&[], |to| to.as_slice())
    }

    pub(crate) fn replaced_by(&self, new: FunctionId) -> &[FunctionId] {
        self.replaced.get(&new).map_or(&[], |old| old.as_slice())
    }
}

impl Object {
    pub(crate) fn item(self) -> Option<Item> {
        match self {
            Object::Function(id) => Some(Item::Function(id)),
            Object::Table(id) => Some(Item::Table(id)),
            Object::Type(id) => Some(Item::Type(id)),
            Object::Trigger(id) => Some(Item::Trigger(id)),
            Object::EventTrigger(id) => Some(Item::EventTrigger(id)),
            Object::Index(id) => Some(Item::Index(id)),
            Object::Constraint(id) => Some(Item::Constraint(id)),
            Object::Policy(id) => Some(Item::Policy(id)),
            Object::Rule(id) => Some(Item::Rule(id)),
            Object::Operator(id) => Some(Item::Operator(id)),
            Object::AccessMethod(id) => Some(Item::AccessMethod(id)),
            Object::Language(id) => Some(Item::Language(id)),
            Object::ForeignDataWrapper(id) => Some(Item::ForeignDataWrapper(id)),
            Object::Conversion(id) => Some(Item::Conversion(id)),
            Object::TextSearch(id) => Some(Item::TextSearch(id)),
            Object::Collation(id) => Some(Item::Collation(id)),
            Object::OperatorFamily(id) => Some(Item::OperatorFamily(id)),
            Object::OperatorClass(id) => Some(Item::OperatorClass(id)),
            Object::Statistics(id) => Some(Item::Statistics(id)),
            Object::Extension(id) => Some(Item::Extension(id)),
            Object::Schema(id) => Some(Item::Schema(id)),
            Object::Server(id) => Some(Item::Server(id)),
            Object::Publication(id) => Some(Item::Publication(id)),
            Object::ColumnExpr(_)
            | Object::OperatorFamilyMember(_)
            | Object::Cast(_)
            | Object::Transform(_) => None,
        }
    }
}

pub(crate) fn set_object_schema(index: &mut IndexData, object: Object, schema: Name) -> bool {
    match object {
        Object::Table(table) => index.tables.set_schema(table, schema),
        Object::Index(idx) => index.indexes.set_schema(idx, schema),
        Object::Type(ty) => index.types.set_schema(ty, schema),
        Object::Function(function) => index.functions.set_schema(function, schema),
        Object::Operator(operator) => index.operators.set_schema(operator, schema),
        Object::Collation(collation) => index.collations.set_schema(collation, schema),
        Object::Conversion(conversion) => index.conversions.set_schema(conversion, schema),
        Object::OperatorFamily(family) => index.operator_families.set_schema(family, schema),
        Object::OperatorClass(class) => index.operator_classes.set_schema(class, schema),
        Object::Statistics(statistics) => index.statistics.set_schema(statistics, schema),
        Object::Extension(extension) => index.extensions.set_schema(extension, schema),
        Object::TextSearch(text_search) => index.text_search.set_schema(text_search, schema),
        Object::Trigger(_)
        | Object::EventTrigger(_)
        | Object::Constraint(_)
        | Object::Policy(_)
        | Object::Rule(_)
        | Object::Cast(_)
        | Object::Transform(_)
        | Object::AccessMethod(_)
        | Object::Language(_)
        | Object::ForeignDataWrapper(_)
        | Object::ColumnExpr(_)
        | Object::OperatorFamilyMember(_)
        | Object::Schema(_)
        | Object::Server(_)
        | Object::Publication(_) => false,
    }
}

pub(crate) fn rename_schema_objects(index: &mut IndexData, from: Name, to: Name) {
    index.tables.rename_schema(from, to);
    index.extensions.rename_schema(from, to);
    index.indexes.rename_schema(from, to);
    index.types.rename_schema(from, to);
    index.functions.rename_schema(from, to);
    index.operators.rename_schema(from, to);
    index.collations.rename_schema(from, to);
    index.conversions.rename_schema(from, to);
    index.operator_families.rename_schema(from, to);
    index.operator_classes.rename_schema(from, to);
    index.statistics.rename_schema(from, to);
    index.text_search.rename_schema(from, to);
}
