use smol_str::SmolStr;
use squawk_syntax::ast::{self, AstNode};
use std::borrow::Borrow;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct Name(pub(crate) SmolStr);

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Schema(pub(crate) Name);

impl Schema {
    pub(crate) fn new(name: impl Into<SmolStr>) -> Self {
        Schema(Name::from_string(name))
    }
}

impl fmt::Display for Schema {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.0)
    }
}

impl Name {
    // TODO: we should get rid of this and update the ast methods to return
    // normalized idents.
    pub(crate) fn from_string(text: impl Into<SmolStr>) -> Self {
        let text = text.into();
        let text = text
            .strip_prefix('"')
            .and_then(|t| t.strip_suffix('"'))
            .map(|x| x.replace(r#""""#, "\""))
            .unwrap_or(text.to_ascii_lowercase());
        Name(text.into())
    }
    pub(crate) fn from_node(node: &impl ast::NameLike) -> Self {
        let text = node.text();
        Name(text.into())
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn debug_assert_normalized(text: &str) {
    debug_assert!(
        !text.bytes().any(|b| b.is_ascii_uppercase()),
        "un-normalized str used as a Name: {text:?}"
    );
}

// Allows us to use either Name or &str
pub(crate) trait AsName {
    fn as_name(&self) -> &str;
}

impl AsName for Name {
    fn as_name(&self) -> &str {
        &self.0
    }
}

impl AsName for str {
    fn as_name(&self) -> &str {
        debug_assert_normalized(self);
        self
    }
}

impl Borrow<str> for Name {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl PartialEq<str> for Name {
    fn eq(&self, other: &str) -> bool {
        debug_assert_normalized(other);
        self.0 == other
    }
}

impl PartialEq<Name> for str {
    fn eq(&self, other: &Name) -> bool {
        other == self
    }
}

impl PartialEq<&str> for Name {
    fn eq(&self, other: &&str) -> bool {
        self == *other
    }
}

impl PartialEq<Name> for &str {
    fn eq(&self, other: &Name) -> bool {
        other == *self
    }
}

pub(crate) fn schema_and_name_path(path: &ast::PathRef) -> Option<(Option<Schema>, Name)> {
    Some((schema_name(path), table_name(path)?))
}

pub(crate) fn schema_and_name_definition(path: &ast::Path) -> Option<(Option<Schema>, Name)> {
    Some((schema_definition_name(path), table_definition_name(path)?))
}

pub(crate) fn schema_and_table_name(
    name_ref: &impl ast::NameLike,
) -> Option<(Option<Schema>, Name)> {
    if let Some(path) = name_ref.syntax().ancestors().find_map(ast::PathRef::cast) {
        return schema_and_name_path(&path);
    }

    Some((None, Name::from_node(name_ref)))
}

pub(crate) fn schema_and_name(name_ref: &impl ast::NameLike) -> (Option<Schema>, Name) {
    let table_name = Name::from_node(name_ref);
    let schema = if let Some(parent) = name_ref.syntax().parent()
        && let Some(base) = ast::FieldExpr::cast(parent).and_then(|x| x.base())
        && let Some(schema_name_ref) = ast::NameRef::cast(base.syntax().clone())
    {
        Some(Schema(Name::from_node(&schema_name_ref)))
    } else {
        None
    };

    (schema, table_name)
}

pub(crate) fn schema_and_func_name(call_expr: &ast::CallExpr) -> Option<(Option<Schema>, Name)> {
    match call_expr.expr()? {
        ast::Expr::NameRef(name_ref) => Some((None, Name::from_node(&name_ref))),
        ast::Expr::FieldExpr(field_expr) => {
            let function_name = Name::from_node(&field_expr.field()?);
            let ast::Expr::NameRef(schema_name_ref) = field_expr.base()? else {
                return None;
            };
            let schema = Schema(Name::from_node(&schema_name_ref));
            Some((Some(schema), function_name))
        }
        _ => None,
    }
}

pub(crate) fn table_name(path: &ast::PathRef) -> Option<Name> {
    Some(Name::from_node(&path.segment()?))
}

pub(crate) fn table_definition_name(path: &ast::Path) -> Option<Name> {
    Some(Name::from_node(&path.segment()?))
}

pub(crate) fn schema_name(path: &ast::PathRef) -> Option<Schema> {
    schema_name_from_qualifier(path.qualifier())
}

pub(crate) fn schema_definition_name(path: &ast::Path) -> Option<Schema> {
    schema_name_from_qualifier(path.qualifier())
}

fn schema_name_from_qualifier(qualifier: Option<ast::PathRef>) -> Option<Schema> {
    qualifier
        .and_then(|q| q.segment())
        .map(|name_ref| Schema(Name::from_node(&name_ref)))
}

// TODO: doesn't handle CTEs/subqueries/aliases
pub(crate) fn schema_and_table_from_from_item(
    from_item: &ast::FromItem,
) -> Option<(Option<Schema>, Name)> {
    let ast::FromItem::RelationFromItem(relation) = from_item else {
        return None;
    };
    schema_and_name_path(&relation.path_ref()?)
}

pub(crate) fn schema_and_table_from_field_expr(
    field_expr: &ast::FieldExpr,
) -> Option<(Option<Schema>, Name)> {
    match field_expr.base()? {
        ast::Expr::NameRef(name_ref) => Some((None, Name::from_node(&name_ref))),
        ast::Expr::FieldExpr(field_expr) => {
            let field = field_expr.field()?;
            let ast::Expr::NameRef(schema) = field_expr.base()? else {
                return None;
            };
            Some((
                Some(Schema(Name::from_node(&schema))),
                Name::from_node(&field),
            ))
        }
        _ => None,
    }
}

pub(crate) fn schema_and_type_name(ty: &ast::Type) -> Option<(Option<Schema>, Name)> {
    match ty {
        ast::Type::ArrayType(array_type) => {
            let inner = array_type.ty()?;
            schema_and_type_name(&inner)
        }
        ast::Type::BitType(bit_type) => {
            let name = if bit_type.varying_token().is_some() {
                "varbit"
            } else {
                "bit"
            };
            Some((None, Name::from_string(name)))
        }
        ast::Type::IntervalType(_) => Some((None, Name::from_string("interval"))),
        ast::Type::PathType(path_type) => {
            let path = path_type.path_ref()?;
            schema_and_name_path(&path)
        }
        ast::Type::ExprType(expr_type) => {
            if let ast::Expr::FieldExpr(field_expr) = expr_type.expr()?
                && let Some(field) = field_expr.field()
                && let Some(ast::Expr::NameRef(schema_name_ref)) = field_expr.base()
            {
                let type_name = Name::from_node(&field);
                let schema = Some(Schema(Name::from_node(&schema_name_ref)));
                Some((schema, type_name))
            } else {
                None
            }
        }
        ast::Type::CharType(char_type) => {
            let name = if char_type.varchar_token().is_some() || char_type.varying_token().is_some()
            {
                "varchar"
            } else {
                "bpchar"
            };
            Some((None, Name::from_string(name)))
        }
        ast::Type::DoubleType(_) => Some((None, Name::from_string("float8"))),
        ast::Type::TimeType(time_type) => {
            let mut name = if time_type.timestamp_token().is_some() {
                "timestamp".to_string()
            } else {
                "time".to_string()
            };
            if let Some(ast::Timezone::WithTimezone(_)) = time_type.timezone() {
                name.push_str("tz");
            }
            Some((None, Name::from_string(name)))
        }
        ast::Type::PercentType(_) => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn name_case_insensitive_compare() {
        assert_eq!(Name::from_string("foo"), Name::from_string(r#""foo""#));
    }

    #[test]
    fn name_quote_comparing() {
        assert_eq!(Name::from_string(r#""foo""#), Name::from_string("foo"));
    }

    #[test]
    fn name_str_comparing() {
        assert_eq!(Name::from_string("FOO"), "foo");
        assert_eq!("foo", Name::from_string(r#""foo""#));
        assert_ne!(Name::from_string(r#""FOO""#), "foo");
    }
}
