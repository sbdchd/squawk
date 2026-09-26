use std::marker::PhantomData;

use rowan::{GreenNode, TextRange};

use crate::{
    Parse, SyntaxNode, ast, ast::AstNode, decoded_text::DecodedText, syntax_error::SyntaxError,
    validation,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Body<T> {
    green: GreenNode,
    errors: Vec<SyntaxError>,
    decoded: DecodedText,
    _ty: PhantomData<fn() -> T>,
}

pub trait BodyLanguage: AstNode {
    const LANGUAGE: &'static str;

    fn parse_text(text: &str) -> (GreenNode, Vec<SyntaxError>);

    fn is_language(language: Option<ast::LanguageName>) -> bool {
        language.is_some_and(|language| language.name == Self::LANGUAGE)
    }
}

impl<T: BodyLanguage> Body<T> {
    pub const LANGUAGE: &'static str = T::LANGUAGE;

    pub(crate) fn parse(decoded: DecodedText) -> Self {
        let (green, errors) = T::parse_text(decoded.text());
        let errors = errors
            .into_iter()
            .map(|error| {
                let range = decoded.source_range(error.range());
                error.with_range(range)
            })
            .collect();

        Self {
            green,
            errors,
            decoded,
            _ty: PhantomData,
        }
    }

    pub(crate) fn from_options(options: ast::FuncOptionList) -> Option<Self> {
        let mut matches = false;
        let mut body = None;

        for option in options.options() {
            match option {
                ast::FuncOption::LanguageFuncOption(option) => {
                    matches = T::is_language(option.language_name());
                }
                ast::FuncOption::AsFuncOption(option) => {
                    if let Some(ast::AsFuncTarget::AsDefinition(definition)) =
                        option.as_func_target()
                    {
                        body = definition.literal();
                    }
                }
                _ => (),
            }
        }

        matches.then_some(())?;
        Some(Self::parse(body?.decoded_value()?))
    }

    pub fn to_parse(&self) -> Parse<T> {
        Parse::new(self.green.clone(), vec![])
    }

    pub fn syntax(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green.clone())
    }

    pub fn tree(&self) -> T {
        T::cast(self.syntax()).expect("root is always the body's node")
    }

    pub fn text(&self) -> &str {
        self.decoded.text()
    }

    pub fn source_range(&self, range: TextRange) -> TextRange {
        self.decoded.source_range(range)
    }

    pub fn body_position(&self, source: rowan::TextSize) -> Option<rowan::TextSize> {
        self.decoded.decoded_pos(source)
    }

    pub fn errors(&self) -> Vec<SyntaxError> {
        let mut validation_errors = vec![];
        validation::validate(&self.syntax(), &mut validation_errors);

        let mut errors = self.errors.clone();
        errors.extend(validation_errors.into_iter().map(|error| {
            let range = self.decoded.source_range(error.range());
            error.with_range(range)
        }));
        errors.sort_by_key(|error| error.range().start());
        errors
    }
}
