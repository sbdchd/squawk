use ::line_index::LineIndex;
use anyhow::Result;
use lsp_server::{Connection, Message, Response};
use lsp_types::{DocumentSymbol, DocumentSymbolParams, SymbolKind};
use squawk_ide::db::line_index;
use squawk_ide::document_symbols::{DocumentSymbolKind, document_symbols};

use crate::system::System;
use crate::lsp_utils;

pub(crate) fn handle_document_symbol(
    connection: &Connection,
    req: lsp_server::Request,
    system: &impl System,
) -> Result<()> {
    let params: DocumentSymbolParams = serde_json::from_value(req.params)?;
    let uri = params.text_document.uri;

    let db = system.db();
    let file = system.file(&uri).unwrap();
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
                DocumentSymbolKind::Schema => SymbolKind::NAMESPACE,
                DocumentSymbolKind::Table => SymbolKind::STRUCT,
                DocumentSymbolKind::View => SymbolKind::STRUCT,
                DocumentSymbolKind::MaterializedView => SymbolKind::STRUCT,
                DocumentSymbolKind::Function => SymbolKind::FUNCTION,
                DocumentSymbolKind::Aggregate => SymbolKind::FUNCTION,
                DocumentSymbolKind::Procedure => SymbolKind::FUNCTION,
                DocumentSymbolKind::Type => SymbolKind::CLASS,
                DocumentSymbolKind::Enum => SymbolKind::ENUM,
                DocumentSymbolKind::Index => SymbolKind::KEY,
                DocumentSymbolKind::Domain => SymbolKind::CLASS,
                DocumentSymbolKind::Sequence => SymbolKind::CONSTANT,
                DocumentSymbolKind::Trigger => SymbolKind::EVENT,
                DocumentSymbolKind::Tablespace => SymbolKind::NAMESPACE,
                DocumentSymbolKind::Database => SymbolKind::MODULE,
                DocumentSymbolKind::Server => SymbolKind::OBJECT,
                DocumentSymbolKind::Extension => SymbolKind::PACKAGE,
                DocumentSymbolKind::Column => SymbolKind::FIELD,
                DocumentSymbolKind::Variant => SymbolKind::ENUM_MEMBER,
                DocumentSymbolKind::Cursor => SymbolKind::VARIABLE,
                DocumentSymbolKind::PreparedStatement => SymbolKind::VARIABLE,
                DocumentSymbolKind::Channel => SymbolKind::EVENT,
                DocumentSymbolKind::EventTrigger => SymbolKind::EVENT,
                DocumentSymbolKind::Role => SymbolKind::CLASS,
                DocumentSymbolKind::Policy => SymbolKind::VARIABLE,
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

    let resp = Response {
        id: req.id,
        result: Some(serde_json::to_value(&lsp_symbols).unwrap()),
        error: None,
    };

    connection.sender.send(Message::Response(resp))?;
    Ok(())
}
