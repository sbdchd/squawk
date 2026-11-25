use squawk_syntax::{SyntaxKind, SyntaxNode};

pub(crate) fn is_index_element(expr: &SyntaxNode) -> bool {
    expr.parent().is_some_and(|parent| {
        matches!(
            parent.kind(),
            SyntaxKind::PARTITION_ITEM | SyntaxKind::CONFLICT_INDEX_ITEM
        ) || (parent.kind() == SyntaxKind::COLLATE && is_index_element(&parent))
    })
}
