use smallvec::SmallVec;
use squawk_syntax::ast::{self, NameLike};

use squawk_syntax::identifier::truncate_identifier;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub(crate) struct Name(String);

impl Name {
    pub(crate) fn from_node(node: &impl NameLike) -> Self {
        Self::from_string(node.text().into_owned())
    }

    pub(crate) fn from_string(mut text: String) -> Self {
        text.truncate(truncate_identifier(&text).len());
        Self(text)
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }

    pub(crate) fn is_public_schema(&self) -> bool {
        self.0 == "public"
    }
}

pub(crate) fn path_name(path: &ast::Path) -> SmallVec<[Name; 2]> {
    let mut name: SmallVec<[Name; 2]> = path
        .qualifier()
        .into_iter()
        .flat_map(|qualifier| qualifier.segments())
        .map(|segment| Name::from_node(&segment))
        .collect();
    if let Some(segment) = path.segment() {
        name.push(Name::from_node(&segment));
    }
    strip_public_schema(&mut name);
    name
}

pub(crate) fn path_ref_name(path: &ast::PathRef) -> SmallVec<[Name; 2]> {
    let mut name = path
        .segments()
        .map(|segment| Name::from_node(&segment))
        .collect();
    strip_public_schema(&mut name);
    name
}

fn strip_public_schema(name: &mut SmallVec<[Name; 2]>) {
    if let [schema, _] = name.as_slice()
        && schema.is_public_schema()
    {
        name.remove(0);
    }
}
