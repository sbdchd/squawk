use anyhow::{Context, Result};
use lsp_types::{
    CodeAction, CodeActionKind, CodeActionOrCommand, CodeActionParams, CodeActionResponse, Command,
    WorkspaceEdit,
};
use rustc_hash::FxHashMap;
use squawk_ide::code_actions::code_actions;
use squawk_ide::db::line_index;

use crate::diagnostic::{AssociatedDiagnosticData, DIAGNOSTIC_NAME};
use crate::lsp_utils;
use crate::system::System;

pub(crate) fn handle_code_action(
    system: &dyn System,
    params: CodeActionParams,
) -> Result<Option<CodeActionResponse>> {
    let uri = params.text_document.uri;

    let mut actions: CodeActionResponse = vec![];

    let db = system.db();
    let file = system.file(&uri).unwrap();
    let line_index = line_index(db, file);
    let offset = lsp_utils::offset(&line_index, params.range.start).unwrap();

    let ide_actions = code_actions(db, file, offset).unwrap_or_default();

    for action in ide_actions {
        let lsp_action = lsp_utils::code_action(&line_index, uri.clone(), action);
        actions.push(CodeActionOrCommand::CodeAction(lsp_action));
    }

    for mut diagnostic in params
        .context
        .diagnostics
        .into_iter()
        .filter(|diagnostic| diagnostic.source.as_deref() == Some(DIAGNOSTIC_NAME))
    {
        let Some(rule_name) = diagnostic.code.as_ref().map(|x| match x {
            lsp_types::NumberOrString::String(s) => s.clone(),
            lsp_types::NumberOrString::Number(n) => n.to_string(),
        }) else {
            continue;
        };
        let Some(data) = diagnostic.data.take() else {
            continue;
        };

        let associated_data: AssociatedDiagnosticData =
            serde_json::from_value(data).context("deserializing diagnostic data")?;

        if let Some(ignore_line_edit) = associated_data.ignore_line_edit {
            let disable_line_action = CodeAction {
                title: format!("Disable {rule_name} for this line"),
                kind: Some(CodeActionKind::QUICKFIX),
                diagnostics: Some(vec![diagnostic.clone()]),
                edit: Some(WorkspaceEdit::new({
                    let mut changes = FxHashMap::default();
                    changes.insert(uri.clone(), vec![ignore_line_edit]);
                    changes.into_iter().collect()
                })),
                ..Default::default()
            };
            actions.push(CodeActionOrCommand::CodeAction(disable_line_action));
        }
        if let Some(ignore_file_edit) = associated_data.ignore_file_edit {
            let disable_file_action = CodeAction {
                title: format!("Disable {rule_name} for the entire file"),
                kind: Some(CodeActionKind::QUICKFIX),
                diagnostics: Some(vec![diagnostic.clone()]),
                edit: Some(WorkspaceEdit::new({
                    let mut changes = FxHashMap::default();
                    changes.insert(uri.clone(), vec![ignore_file_edit]);
                    changes.into_iter().collect()
                })),
                ..Default::default()
            };
            actions.push(CodeActionOrCommand::CodeAction(disable_file_action));
        }

        let title = format!("Show documentation for {rule_name}");
        let documentation_action = CodeAction {
            title: title.clone(),
            kind: Some(CodeActionKind::QUICKFIX),
            diagnostics: Some(vec![diagnostic.clone()]),
            command: Some(Command {
                title,
                command: "vscode.open".to_string(),
                arguments: Some(vec![serde_json::to_value(format!(
                    "https://squawkhq.com/docs/{rule_name}"
                ))?]),
            }),
            ..Default::default()
        };
        actions.push(CodeActionOrCommand::CodeAction(documentation_action));

        if !associated_data.title.is_empty() && !associated_data.edits.is_empty() {
            let fix_action = CodeAction {
                title: associated_data.title,
                kind: Some(CodeActionKind::QUICKFIX),
                diagnostics: Some(vec![diagnostic.clone()]),
                edit: Some(WorkspaceEdit::new({
                    let mut changes = FxHashMap::default();
                    changes.insert(uri.clone(), associated_data.edits);
                    changes.into_iter().collect()
                })),
                is_preferred: Some(true),
                ..Default::default()
            };
            actions.push(CodeActionOrCommand::CodeAction(fix_action));
        }
    }

    Ok(Some(actions))
}
