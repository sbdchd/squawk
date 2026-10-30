use rowan::{TextRange, TextSize};
use squawk_syntax::{
    ast::{self, AstNode, NameLike},
    match_ast,
};

use squawk_sema::file::{FileId, FileRange, InFile};
use squawk_sema::ir::{ColumnId, FunctionKind, Item, ParamMode, Resolution};
use squawk_sema::semantic_index::SemanticIndex;

use crate::db::{Db, File, original_file, parse, semantic_index};

pub fn inlay_hints(db: &dyn Db, file_id: FileId) -> Vec<InlayHint> {
    file_inlay_hints(db, original_file(db, file_id))
}

#[salsa::tracked]
fn file_inlay_hints(db: &dyn Db, file: File) -> Vec<InlayHint> {
    let file_id = file.file_id(db);
    inlay_hints_with_index(
        semantic_index(db, file_id),
        InFile::new(file_id, &parse(db, file_id).tree()),
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InlayHintKind {
    Type,
    Parameter,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InlayHint {
    pub position: TextSize,
    pub label: String,
    pub kind: InlayHintKind,
    pub target: Option<FileRange>,
}

pub(crate) fn inlay_hints_with_index(
    index: &SemanticIndex,
    file: InFile<&ast::SourceFile>,
) -> Vec<InlayHint> {
    let hints = Hints {
        index,
        file_id: file.file_id,
    };
    let mut result = vec![];
    for node in file.value.syntax().descendants() {
        match_ast! {
            match node {
                ast::CallExpr(call_expr) => hints.call_expr(&call_expr, &mut result),
                ast::Call(call) => hints.call(&call, &mut result),
                ast::Insert(insert) => hints.insert(&insert, &mut result),
                _ => None,
            }
        };
    }
    result
}

struct Hints<'a> {
    index: &'a SemanticIndex,
    file_id: FileId,
}

impl Hints<'_> {
    fn call_expr(&self, call_expr: &ast::CallExpr, result: &mut Vec<InlayHint>) -> Option<()> {
        let arg_list = call_expr.arg_list()?;
        let name_ref = match call_expr.expr()? {
            ast::Expr::NameRef(name_ref) => name_ref,
            ast::Expr::FieldExpr(field_expr) => field_expr.field()?,
            ast::Expr::ArrayExpr(_)
            | ast::Expr::BetweenExpr(_)
            | ast::Expr::BinExpr(_)
            | ast::Expr::CallExpr(_)
            | ast::Expr::CaseExpr(_)
            | ast::Expr::CastExpr(_)
            | ast::Expr::Collate(_)
            | ast::Expr::IndexExpr(_)
            | ast::Expr::Literal(_)
            | ast::Expr::ParenExpr(_)
            | ast::Expr::PostfixExpr(_)
            | ast::Expr::PrefixExpr(_)
            | ast::Expr::SliceExpr(_)
            | ast::Expr::TupleExpr(_) => return None,
        };
        self.args(name_ref.syntax().text_range(), &arg_list, result)
    }

    fn call(&self, call: &ast::Call, result: &mut Vec<InlayHint>) -> Option<()> {
        let arg_list = call.arg_list()?;
        let segment = call.procedure_name_ref()?.path_ref()?.segment()?;
        self.args(segment.syntax().text_range(), &arg_list, result)
    }

    fn args(
        &self,
        name: TextRange,
        arg_list: &ast::ArgList,
        result: &mut Vec<InlayHint>,
    ) -> Option<()> {
        let Resolution::Function(function) = self.resolution(name)? else {
            return None;
        };
        let function = &self.index.functions[function];
        let params = function.params.into_iter().filter(|&param| {
            let mode = self.index.params[param].mode;
            mode != ParamMode::Out || function.kind == FunctionKind::Procedure
        });
        for (param, arg) in params.zip(arg_list.args()) {
            match arg.func_arg_expr() {
                Some(ast::FuncArgExpr::Expr(_)) => {}
                Some(ast::FuncArgExpr::NamedArg(_)) => break,
                None => continue,
            }
            let param = &self.index.params[param];
            let (Some(name), Some(name_ptr)) = (param.name, param.name_ptr) else {
                continue;
            };
            result.push(InlayHint {
                position: arg.syntax().text_range().start(),
                label: format!("{}: ", self.index.names.resolve(name)),
                kind: InlayHintKind::Parameter,
                target: Some(self.index.range(name_ptr.file_range())),
            });
        }
        Some(())
    }

    fn insert(&self, insert: &ast::Insert, result: &mut Vec<InlayHint>) -> Option<()> {
        let ast::InsertSource::SelectVariant(select) = insert.insert_source()? else {
            return None;
        };
        let mut columns = vec![];
        if let Some(column_list) = insert.column_target_list() {
            for column_target in column_list.column_targets() {
                let Some(name) = column_target.name() else {
                    continue;
                };
                let target = match self.resolution(name.syntax().text_range()) {
                    Some(Resolution::Column { column, .. }) => self.column_target(column),
                    Some(_) | None => None,
                };
                columns.push(ColumnHint {
                    label: name.text(),
                    target,
                });
            }
        } else {
            let segment = insert.relation_name_ref()?.path_ref()?.segment()?;
            let Resolution::Table(table) = self.resolution(segment.syntax().text_range())? else {
                return None;
            };
            let table_columns = self.index.tables[table].columns.range;
            columns.reserve(table_columns.len());
            for column in table_columns {
                let Some(name) = self.index.columns[column].name else {
                    continue;
                };
                columns.push(ColumnHint {
                    label: self.index.names.resolve(name).to_string(),
                    target: self.column_target(column),
                });
            }
        }

        match &select {
            ast::SelectVariant::Values(values) => {
                for row in values.row_list()?.rows() {
                    for (column, expr) in columns.iter().zip(row.exprs()) {
                        result.push(column.hint(expr.syntax().text_range().start()));
                    }
                }
            }
            ast::SelectVariant::Select(_)
            | ast::SelectVariant::SelectInto(_)
            | ast::SelectVariant::ParenSelect(_)
            | ast::SelectVariant::CompoundSelect(_)
            | ast::SelectVariant::Table(_) => {
                let target_list = select.target_list()?;
                for (column, target) in columns.iter().zip(target_list.targets()) {
                    let expr = target.expr()?;
                    result.push(column.hint(expr.syntax().text_range().start()));
                }
            }
        }
        Some(())
    }

    fn resolution(&self, range: TextRange) -> Option<Resolution> {
        let item = self.index.item_at_range(InFile::new(self.file_id, range))?;
        let Item::Reference(reference) = item.value.item else {
            return None;
        };
        Some(self.index.references[reference].resolution)
    }

    fn column_target(&self, column: ColumnId) -> Option<FileRange> {
        let column = self.index.merged_columns(column).into_iter().next()?;
        let ptr = self.index.columns[column].ptr;
        Some(self.index.range(ptr.file_range()))
    }
}

struct ColumnHint {
    label: String,
    target: Option<FileRange>,
}

impl ColumnHint {
    fn hint(&self, position: TextSize) -> InlayHint {
        InlayHint {
            position,
            label: format!("{}: ", self.label),
            kind: InlayHintKind::Parameter,
            target: self.target,
        }
    }
}

#[cfg(test)]
mod tests;
