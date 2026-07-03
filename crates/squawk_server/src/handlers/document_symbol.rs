use ::line_index::LineIndex;
use anyhow::Result;
use gen_lsp_types::{DocumentSymbol, DocumentSymbolParams, DocumentSymbolResponse, SymbolKind};
use squawk_ide::db::line_index;
use squawk_ide::document_symbols::{DocumentSymbolKind, document_symbols};

use crate::global_state::Snapshot;
use crate::lsp_utils;

pub(crate) fn handle_document_symbol(
    snapshot: &Snapshot,
    params: DocumentSymbolParams,
) -> Result<Option<DocumentSymbolResponse>> {
    let uri = params.text_document.uri;

    let db = snapshot.db();
    let file = snapshot.file(&uri).unwrap();
    let line_index = line_index(db, file);

    let symbols = document_symbols(db, file);

    fn convert_symbol(
        sym: squawk_ide::document_symbols::DocumentSymbol,
        line_index: &LineIndex,
    ) -> DocumentSymbol {
        let range = lsp_utils::range(line_index, sym.full_range);
        let selection_range = lsp_utils::range(line_index, sym.focus_range);

        let children = sym
            .children
            .into_iter()
            .map(|child| convert_symbol(child, line_index))
            .collect::<Vec<_>>();

        let children = (!children.is_empty()).then_some(children);

        DocumentSymbol {
            name: sym.name,
            detail: sym.detail,
            kind: match sym.kind {
                DocumentSymbolKind::Schema => SymbolKind::Namespace,
                DocumentSymbolKind::Table => SymbolKind::Struct,
                DocumentSymbolKind::View => SymbolKind::Struct,
                DocumentSymbolKind::MaterializedView => SymbolKind::Struct,
                DocumentSymbolKind::Function => SymbolKind::Function,
                DocumentSymbolKind::Aggregate => SymbolKind::Function,
                DocumentSymbolKind::Procedure => SymbolKind::Function,
                DocumentSymbolKind::Type => SymbolKind::Class,
                DocumentSymbolKind::Enum => SymbolKind::Enum,
                DocumentSymbolKind::Index => SymbolKind::Key,
                DocumentSymbolKind::Domain => SymbolKind::Class,
                DocumentSymbolKind::Sequence => SymbolKind::Constant,
                DocumentSymbolKind::Trigger => SymbolKind::Event,
                DocumentSymbolKind::Tablespace => SymbolKind::Namespace,
                DocumentSymbolKind::Database => SymbolKind::Module,
                DocumentSymbolKind::Server => SymbolKind::Object,
                DocumentSymbolKind::Extension => SymbolKind::Package,
                DocumentSymbolKind::Column => SymbolKind::Field,
                DocumentSymbolKind::Variant => SymbolKind::EnumMember,
                DocumentSymbolKind::Cursor => SymbolKind::Variable,
                DocumentSymbolKind::PreparedStatement => SymbolKind::Variable,
                DocumentSymbolKind::Channel => SymbolKind::Event,
                DocumentSymbolKind::EventTrigger => SymbolKind::Event,
                DocumentSymbolKind::Role => SymbolKind::Class,
                DocumentSymbolKind::Rule => SymbolKind::Event,
                DocumentSymbolKind::Policy => SymbolKind::Variable,
                DocumentSymbolKind::PropertyGraph => SymbolKind::Struct,
            },
            tags: None,
            range,
            selection_range,
            children,
            #[allow(deprecated)]
            deprecated: None,
        }
    }

    let lsp_symbols: Vec<DocumentSymbol> = symbols
        .into_iter()
        .map(|sym| convert_symbol(sym, &line_index))
        .collect();

    Ok(Some(DocumentSymbolResponse::DocumentSymbolList(
        lsp_symbols,
    )))
}
