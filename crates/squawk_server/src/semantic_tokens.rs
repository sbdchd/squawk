use gen_lsp_types::{SemanticTokenModifiers, SemanticTokenTypes};

pub(crate) const SUPPORTED_TYPES: &[SemanticTokenTypes] = &[
    SemanticTokenTypes::Comment,
    SemanticTokenTypes::Function,
    SemanticTokenTypes::Keyword,
    SemanticTokenTypes::Namespace,
    SemanticTokenTypes::Number,
    SemanticTokenTypes::Operator,
    SemanticTokenTypes::Parameter,
    SemanticTokenTypes::Property,
    SemanticTokenTypes::String,
    SemanticTokenTypes::Struct,
    SemanticTokenTypes::Type,
    SemanticTokenTypes::Variable,
];

pub(crate) const SUPPORTED_MODIFIERS: &[SemanticTokenModifiers] = &[
    SemanticTokenModifiers::Declaration,
    SemanticTokenModifiers::Definition,
    SemanticTokenModifiers::Readonly,
];

pub(crate) fn type_index(ty: SemanticTokenTypes) -> u32 {
    SUPPORTED_TYPES.iter().position(|it| *it == ty).unwrap() as u32
}
