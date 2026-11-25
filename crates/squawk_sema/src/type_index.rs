use std::ops::Index;

use squawk_syntax::SyntaxNodePtr;

use crate::arena::{IdxRange, LayeredArena, LayeredMultimap};
use crate::file::InIrFile;
use crate::interner::{Name, QualifiedName};
use crate::ir::{
    Column, ColumnType, ColumnTypeRef, Columns, Subscript, Ty, Type, TypeCategory, TypeId,
};

use crate::qualified_index::QualifiedIndex;

type NamePtr = InIrFile<SyntaxNodePtr>;

#[derive(Debug, Default)]
pub struct TypeIndex {
    types: QualifiedIndex<Type>,
    by_range: LayeredMultimap<TypeId, TypeId>,
    by_column_type: LayeredMultimap<ColumnTypeRef, TypeId>,
    by_array_name: LayeredMultimap<QualifiedName, TypeId>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Keys {
    range: Option<TypeId>,
    array_name: Option<QualifiedName>,
}

impl Keys {
    fn of(ty: &Type) -> Keys {
        Keys {
            range: ty.range,
            array_name: ty.array_name.map(|name| QualifiedName {
                schema: ty.schema,
                name,
            }),
        }
    }
}

impl TypeIndex {
    pub(crate) fn on_top_of(base: &'static TypeIndex) -> Self {
        TypeIndex {
            types: QualifiedIndex::on_top_of(&base.types),
            by_range: LayeredMultimap::on_top_of(&base.by_range),
            by_column_type: LayeredMultimap::on_top_of(&base.by_column_type),
            by_array_name: LayeredMultimap::on_top_of(&base.by_array_name),
        }
    }

    pub(crate) fn alloc(&mut self, ty: Type, columns: &LayeredArena<Column>) -> TypeId {
        let keys = Keys::of(&ty);
        let type_columns = ty.columns;
        let id = self.types.alloc(ty);
        self.index_columns(id, type_columns, columns);
        if let Some(range) = keys.range {
            self.by_range.insert(range, id);
        }
        if let Some(array_name) = keys.array_name {
            self.by_array_name.insert(array_name, id);
        }
        id
    }

    pub(crate) fn named(&self, schema: Name, name: Name) -> impl Iterator<Item = TypeId> + '_ {
        self.types.named(schema, name)
    }

    pub(crate) fn with_array_name(
        &self,
        schema: Name,
        name: Name,
    ) -> impl Iterator<Item = TypeId> + '_ {
        self.by_array_name.get(QualifiedName { schema, name })
    }

    pub(crate) fn in_schema(&self, schema: Name) -> impl DoubleEndedIterator<Item = TypeId> + '_ {
        self.types.in_schema(schema)
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
        self.types.next_idx()
    }

    pub(crate) fn range_from(&self, start: TypeId) -> IdxRange<Type> {
        self.types.range_from(start)
    }

    pub(crate) fn is_local(&self, id: TypeId) -> bool {
        self.types.is_local(id)
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

    pub(crate) fn set_array_name(&mut self, id: TypeId, array_name: Option<Name>) -> Option<Name> {
        self.rekey(id, |ty| std::mem::replace(&mut ty.array_name, array_name))
    }

    pub(crate) fn set_schema(&mut self, id: TypeId, schema: Name) -> bool {
        if !self.types.is_local(id) {
            return false;
        }
        self.rekey(id, |ty| ty.schema = schema);
        true
    }

    pub(crate) fn rename_schema(&mut self, from: Name, to: Name) {
        for id in self.types.take_schema(from) {
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
            self.by_column_type
                .insert_keys(id, ColumnTypeRef::in_columns(arena, columns.range));
        }
    }

    fn unindex_columns(
        &mut self,
        id: TypeId,
        columns: Option<Columns>,
        arena: &LayeredArena<Column>,
    ) {
        if let Some(columns) = columns {
            self.by_column_type
                .remove_keys(id, ColumnTypeRef::in_columns(arena, columns.range));
        }
    }

    pub(crate) fn set_base(&mut self, id: TypeId, base: ColumnType) {
        let ty = self.local_mut(id);
        ty.base = base.ty;
        ty.typmod = base.typmod;
        ty.collation = base.collation;
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
        collatable: bool,
    ) {
        let ty = self.local_mut(id);
        ty.category = category;
        ty.preferred = preferred;
        ty.collatable = collatable;
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
        let previous = Keys::of(&self.types[id]);
        let result = self.types.rekey(id, f);
        let current = Keys::of(&self.types[id]);
        self.by_range.replace_key(id, previous.range, current.range);
        self.by_array_name
            .replace_key(id, previous.array_name, current.array_name);
        result
    }

    fn local_mut(&mut self, id: TypeId) -> &mut Type {
        self.types.local_mut(id)
    }

    pub(crate) fn shrink_to_fit(&mut self) {
        self.types.shrink_to_fit();
        self.by_range.shrink_to_fit();
        self.by_column_type.shrink_to_fit();
        self.by_array_name.shrink_to_fit();
    }
}

impl Index<TypeId> for TypeIndex {
    type Output = Type;

    fn index(&self, id: TypeId) -> &Type {
        &self.types[id]
    }
}
