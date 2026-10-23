use std::ops::Index;

use squawk_syntax::{AstPtr, ast};

use crate::arena::{IdxRange, LayeredArena, LayeredMultimap};
use crate::file::InIrFile;
use crate::interner::{Name, QualifiedName};
use crate::ir::{Column, ColumnTypeRef, Columns, Subscript, Ty, Type, TypeCategory, TypeId};

type NamePtr = InIrFile<AstPtr<ast::PathSegment>>;

#[derive(Debug, Default)]
pub(crate) struct TypeIndex {
    arena: LayeredArena<Type>,
    by_name: LayeredMultimap<QualifiedName, TypeId>,
    by_schema: LayeredMultimap<Name, TypeId>,
    by_range: LayeredMultimap<TypeId, TypeId>,
    by_column_type: LayeredMultimap<ColumnTypeRef, TypeId>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Keys {
    name: QualifiedName,
    range: Option<TypeId>,
}

impl Keys {
    fn of(ty: &Type) -> Keys {
        Keys {
            name: QualifiedName {
                schema: ty.schema,
                name: ty.name,
            },
            range: ty.range,
        }
    }
}

impl TypeIndex {
    pub(crate) fn on_top_of(base: &'static TypeIndex) -> Self {
        TypeIndex {
            arena: LayeredArena::on_top_of(&base.arena),
            by_name: LayeredMultimap::on_top_of(&base.by_name),
            by_schema: LayeredMultimap::on_top_of(&base.by_schema),
            by_range: LayeredMultimap::on_top_of(&base.by_range),
            by_column_type: LayeredMultimap::on_top_of(&base.by_column_type),
        }
    }

    pub(crate) fn alloc(&mut self, ty: Type, columns: &LayeredArena<Column>) -> TypeId {
        let keys = Keys::of(&ty);
        let type_columns = ty.columns;
        let id = self.arena.alloc(ty);
        self.index_columns(id, type_columns, columns);
        self.by_name.insert(keys.name, id);
        self.by_schema.insert(keys.name.schema, id);
        if let Some(range) = keys.range {
            self.by_range.insert(range, id);
        }
        id
    }

    pub(crate) fn named(&self, schema: Name, name: Name) -> impl Iterator<Item = TypeId> + '_ {
        self.by_name.get(QualifiedName { schema, name })
    }

    pub(crate) fn in_schema(&self, schema: Name) -> impl DoubleEndedIterator<Item = TypeId> + '_ {
        self.by_schema.get(schema)
    }

    pub(crate) fn with_range(&self, range: TypeId) -> impl Iterator<Item = TypeId> + '_ {
        self.by_range.get(range)
    }

    pub(crate) fn with_column_type(
        &self,
        column_type: ColumnTypeRef,
    ) -> impl DoubleEndedIterator<Item = TypeId> + '_ {
        self.by_column_type.get(column_type)
    }

    pub(crate) fn next_idx(&self) -> TypeId {
        self.arena.next_idx()
    }

    pub(crate) fn range_from(&self, start: TypeId) -> IdxRange<Type> {
        self.arena.range_from(start)
    }

    pub(crate) fn is_local(&self, id: TypeId) -> bool {
        self.arena.is_local(id)
    }

    pub(crate) fn replace(&mut self, id: TypeId, ty: Type, columns: &LayeredArena<Column>) -> Type {
        let type_columns = ty.columns;
        let previous = self.rekey(id, |current| std::mem::replace(current, ty));
        self.unindex_columns(id, previous.columns, columns);
        self.index_columns(id, type_columns, columns);
        previous
    }

    pub(crate) fn move_to(&mut self, id: TypeId, schema: Name, name: Name, name_ptr: NamePtr) {
        self.rekey(id, |ty| {
            ty.schema = schema;
            ty.name = name;
            ty.name_ptr = name_ptr;
        });
    }

    pub(crate) fn set_schema(&mut self, id: TypeId, schema: Name) -> bool {
        if !self.arena.is_local(id) {
            return false;
        }
        self.rekey(id, |ty| ty.schema = schema);
        true
    }

    pub(crate) fn rename_schema(&mut self, from: Name, to: Name) {
        for id in self.by_schema.take(from) {
            self.rekey(id, |ty| ty.schema = to);
        }
    }

    pub(crate) fn set_columns(
        &mut self,
        id: TypeId,
        columns: Option<Columns>,
        arena: &LayeredArena<Column>,
    ) -> Option<Columns> {
        let previous = std::mem::replace(&mut self.local_mut(id).columns, columns);
        self.unindex_columns(id, previous, arena);
        self.index_columns(id, columns, arena);
        previous
    }

    fn index_columns(
        &mut self,
        id: TypeId,
        columns: Option<Columns>,
        arena: &LayeredArena<Column>,
    ) {
        if let Some(columns) = columns {
            for column_type in ColumnTypeRef::in_columns(arena, columns.range) {
                self.by_column_type.insert(column_type, id);
            }
        }
    }

    fn unindex_columns(
        &mut self,
        id: TypeId,
        columns: Option<Columns>,
        arena: &LayeredArena<Column>,
    ) {
        if let Some(columns) = columns {
            for column_type in ColumnTypeRef::in_columns(arena, columns.range) {
                self.by_column_type.remove(column_type, id);
            }
        }
    }

    pub(crate) fn set_base(&mut self, id: TypeId, base: Option<Ty>) {
        self.local_mut(id).base = base;
    }

    pub(crate) fn set_element(&mut self, id: TypeId, element: Option<Ty>, subtype: Option<Ty>) {
        let ty = self.local_mut(id);
        ty.element = element;
        ty.subtype = subtype;
    }

    pub(crate) fn set_category(
        &mut self,
        id: TypeId,
        category: Option<TypeCategory>,
        preferred: bool,
    ) {
        let ty = self.local_mut(id);
        ty.category = category;
        ty.preferred = preferred;
    }

    pub(crate) fn set_subscript(
        &mut self,
        id: TypeId,
        subscript: Option<Subscript>,
        result: Option<Ty>,
        slice: Option<Ty>,
    ) {
        let ty = self.local_mut(id);
        ty.subscript = subscript;
        ty.subscript_result = result;
        ty.slice_result = slice;
    }

    fn rekey<R>(&mut self, id: TypeId, f: impl FnOnce(&mut Type) -> R) -> R {
        let ty = self.local_mut(id);
        let previous = Keys::of(ty);
        let result = f(ty);
        let current = Keys::of(ty);
        if previous.name != current.name {
            self.by_name.remove(previous.name, id);
            self.by_name.insert(current.name, id);
        }
        if previous.name.schema != current.name.schema {
            self.by_schema.remove(previous.name.schema, id);
            self.by_schema.insert(current.name.schema, id);
        }
        if previous.range != current.range {
            if let Some(range) = previous.range {
                self.by_range.remove(range, id);
            }
            if let Some(range) = current.range {
                self.by_range.insert(range, id);
            }
        }
        result
    }

    fn local_mut(&mut self, id: TypeId) -> &mut Type {
        self.arena
            .get_local_mut(id)
            .expect("base layer is immutable")
    }

    pub(crate) fn shrink_to_fit(&mut self) {
        self.arena.shrink_to_fit();
        self.by_name.shrink_to_fit();
        self.by_schema.shrink_to_fit();
        self.by_range.shrink_to_fit();
        self.by_column_type.shrink_to_fit();
    }
}

impl Index<TypeId> for TypeIndex {
    type Output = Type;

    fn index(&self, id: TypeId) -> &Type {
        &self.arena[id]
    }
}
