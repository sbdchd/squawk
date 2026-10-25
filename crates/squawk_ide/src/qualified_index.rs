use std::ops::Index;

use la_arena::Idx;

use crate::arena::{IdxRange, LayeredArena, LayeredMultimap};
use crate::interner::{Name, QualifiedName};
use crate::ir::{
    Collation, Conversion, Function, FunctionId, Operator, OperatorClass, OperatorFamily,
    OperatorId, Statistics, TextSearch, Ty,
};

pub(crate) trait Qualified {
    fn name(&self) -> Name;

    fn schema(&self) -> Name;

    fn schema_mut(&mut self) -> &mut Name;
}

#[derive(Debug)]
pub(crate) struct QualifiedIndex<T: 'static> {
    arena: LayeredArena<T>,
    by_name: LayeredMultimap<QualifiedName, Idx<T>>,
    by_schema: LayeredMultimap<Name, Idx<T>>,
}

impl<T> Default for QualifiedIndex<T> {
    fn default() -> Self {
        QualifiedIndex {
            arena: LayeredArena::default(),
            by_name: LayeredMultimap::default(),
            by_schema: LayeredMultimap::default(),
        }
    }
}

impl<T: Qualified> QualifiedIndex<T> {
    pub(crate) fn on_top_of(base: &'static QualifiedIndex<T>) -> Self {
        QualifiedIndex {
            arena: LayeredArena::on_top_of(&base.arena),
            by_name: LayeredMultimap::on_top_of(&base.by_name),
            by_schema: LayeredMultimap::on_top_of(&base.by_schema),
        }
    }

    pub(crate) fn alloc(&mut self, value: T) -> Idx<T> {
        let schema = value.schema();
        let name = value.name();
        let id = self.arena.alloc(value);
        self.by_name.insert(QualifiedName { schema, name }, id);
        self.by_schema.insert(schema, id);
        id
    }

    pub(crate) fn named(&self, schema: Name, name: Name) -> impl Iterator<Item = Idx<T>> + '_ {
        self.by_name.get(QualifiedName { schema, name })
    }

    pub(crate) fn in_schema(&self, schema: Name) -> impl DoubleEndedIterator<Item = Idx<T>> + '_ {
        self.by_schema.get(schema)
    }

    pub(crate) fn next_idx(&self) -> Idx<T> {
        self.arena.next_idx()
    }

    pub(crate) fn range_from(&self, start: Idx<T>) -> IdxRange<T> {
        self.arena.range_from(start)
    }

    pub(crate) fn set_schema(&mut self, id: Idx<T>, schema: Name) -> bool {
        if !self.arena.is_local(id) {
            return false;
        }
        self.move_schema(id, schema);
        true
    }

    pub(crate) fn rename_schema(&mut self, from: Name, to: Name) {
        for id in self.by_schema.take(from) {
            self.move_schema(id, to);
        }
    }

    fn move_schema(&mut self, id: Idx<T>, schema: Name) {
        let value = &mut self.arena[id];
        let name = value.name();
        let previous = std::mem::replace(value.schema_mut(), schema);
        if previous == schema {
            return;
        }
        self.by_name.remove(
            QualifiedName {
                schema: previous,
                name,
            },
            id,
        );
        self.by_name.insert(QualifiedName { schema, name }, id);
        self.by_schema.remove(previous, id);
        self.by_schema.insert(schema, id);
    }

    pub(crate) fn shrink_to_fit(&mut self) {
        self.arena.shrink_to_fit();
        self.by_name.shrink_to_fit();
        self.by_schema.shrink_to_fit();
    }
}

impl<T> Index<Idx<T>> for QualifiedIndex<T> {
    type Output = T;

    fn index(&self, id: Idx<T>) -> &T {
        &self.arena[id]
    }
}

impl QualifiedIndex<Function> {
    pub(crate) fn set_ret(&mut self, id: FunctionId, ret: Option<Ty>) {
        self.arena[id].ret = ret;
    }

    pub(crate) fn set_window(&mut self, id: FunctionId) {
        self.arena[id].window = true;
    }
}

impl QualifiedIndex<Operator> {
    pub(crate) fn set_function(&mut self, id: OperatorId, function: Option<FunctionId>) {
        self.arena[id].function = function;
    }
}

pub(crate) type CollationIndex = QualifiedIndex<Collation>;
pub(crate) type ConversionIndex = QualifiedIndex<Conversion>;
pub(crate) type FunctionIndex = QualifiedIndex<Function>;
pub(crate) type OperatorClassIndex = QualifiedIndex<OperatorClass>;
pub(crate) type OperatorFamilyIndex = QualifiedIndex<OperatorFamily>;
pub(crate) type OperatorIndex = QualifiedIndex<Operator>;
pub(crate) type TextSearchIndex = QualifiedIndex<TextSearch>;

macro_rules! qualified {
    ($($ty:ty),* $(,)?) => {
        $(
            impl Qualified for $ty {
                fn name(&self) -> Name {
                    self.name
                }

                fn schema(&self) -> Name {
                    self.schema
                }

                fn schema_mut(&mut self) -> &mut Name {
                    &mut self.schema
                }
            }
        )*
    };
}

qualified!(
    Collation,
    Conversion,
    Function,
    Operator,
    OperatorClass,
    OperatorFamily,
    Statistics,
    TextSearch,
);
