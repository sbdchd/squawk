use rowan::{TextRange, TextSize};
use salsa::Database as Db;
use squawk_line_index::{NewlineWithTrailingNewline, find_newline};
use squawk_linter::Edit;
use squawk_syntax::{
    SyntaxToken,
    ast::{self, AstNode, AstToken},
};

use crate::{
    comments::{
        block_comment_content, line_comment_content, line_comment_group, strip_block_decoration,
    },
    db::parse,
    file::InFile,
};

use super::{ActionKind, CodeAction};

pub(super) fn convert_comment(
    db: &dyn Db,
    position: InFile<TextSize>,
    actions: &mut Vec<CodeAction>,
) -> Option<()> {
    let file = position.file_id;
    let comment = parse(db, file)
        .tree()
        .syntax()
        .token_at_offset(position.value)
        .find_map(ast::Comment::cast)?;

    let action = if comment.kind().is_block() {
        if has_trailing_content(&comment) {
            return None;
        }
        CodeAction {
            title: "Convert to line comment".to_owned(),
            edits: vec![block_to_lines(&comment)?],
            kind: ActionKind::RefactorRewrite,
        }
    } else {
        CodeAction {
            title: "Convert to block comment".to_owned(),
            edits: vec![lines_to_block(&comment)?],
            kind: ActionKind::RefactorRewrite,
        }
    };

    actions.push(action);

    Some(())
}

fn has_trailing_content(comment: &ast::Comment) -> bool {
    let Some(next) = comment.syntax().next_token() else {
        return false;
    };
    let Some(whitespace) = ast::Whitespace::cast(next) else {
        return true;
    };

    find_newline(whitespace.text()).is_none() && whitespace.syntax().next_token().is_some()
}

fn block_to_lines(comment: &ast::Comment) -> Option<Edit> {
    let lines = NewlineWithTrailingNewline::from(block_comment_content(comment.text())?)
        .map(|line| {
            let content = strip_block_decoration(line.as_str());
            (line, content)
        })
        .collect::<Vec<_>>();
    // Trim leading `/*` and trailing `*/` lines.
    let lines = trim_empty_edges(&lines, |(_, content)| content.is_empty());

    let text = match lines {
        [] => "--".to_owned(),
        [(_, content)] => line_comment(content),
        lines => {
            let indent = indent_of(comment.syntax());
            let mut text = String::with_capacity(comment.text().len());
            for (i, (line, content)) in lines.iter().enumerate() {
                if i > 0 {
                    text.push_str(&indent);
                }
                text.push_str("--");
                if !content.is_empty() {
                    text.push(' ');
                    text.push_str(content);
                }
                if i + 1 < lines.len()
                    && let Some(line_ending) = line.line_ending()
                {
                    text.push_str(line_ending.as_str());
                }
            }
            text
        }
    };

    Some(Edit::replace(comment.syntax().text_range(), text))
}

fn lines_to_block(comment: &ast::Comment) -> Option<Edit> {
    let comments = line_comment_group(comment);

    let mut contents = Vec::with_capacity(comments.len());
    for comment in &comments {
        let content = line_comment_content(comment.text())?;
        if content.contains("/*") || content.contains("*/") {
            return None;
        }
        contents.push(content);
    }

    let contents = trim_empty_edges(&contents, |content| content.is_empty());

    let range = TextRange::new(
        comments.first()?.syntax().text_range().start(),
        comments.last()?.syntax().text_range().end(),
    );

    let text = if let [content] = contents {
        if content.is_empty() {
            "/**/".to_owned()
        } else {
            format!("/* {content} */")
        }
    } else {
        let indent = indent_of(comments.first()?.syntax());
        let line_ending = group_line_ending(&comments)?;
        let mut text = String::with_capacity(usize::from(range.len()));
        text.push_str("/*");
        for content in contents {
            text.push_str(line_ending);
            text.push_str(&indent);
            text.push_str(" *");
            if !content.is_empty() {
                text.push(' ');
                text.push_str(content);
            }
        }
        text.push_str(line_ending);
        text.push_str(&indent);
        text.push_str(" */");
        text
    };

    Some(Edit::replace(range, text))
}

fn trim_empty_edges<T>(items: &[T], is_empty: impl Fn(&T) -> bool) -> &[T] {
    let Some(start) = items.iter().position(|item| !is_empty(item)) else {
        return &items[..items.len().min(1)];
    };
    let end = items.iter().rposition(|item| !is_empty(item)).unwrap();
    &items[start..=end]
}

fn group_line_ending(comments: &[ast::Comment]) -> Option<&'static str> {
    let whitespace = comments.first()?.syntax().next_token()?;
    Some(find_newline(whitespace.text())?.1.as_str())
}

fn line_comment(content: &str) -> String {
    if content.is_empty() {
        "--".to_owned()
    } else {
        format!("-- {content}")
    }
}

fn indent_of(token: &SyntaxToken) -> String {
    let mut tokens = Vec::new();
    let mut prefix_start = 0;
    let mut prefix_len = 0;
    let mut current = token.prev_token();

    while let Some(token) = current {
        let text = token.text();
        if let Some(idx) = text.rfind(['\n', '\r']) {
            prefix_start = idx + 1;
            prefix_len += text.len() - prefix_start;
            tokens.push(token);
            break;
        }
        prefix_len += text.len();
        current = token.prev_token();
        tokens.push(token);
    }

    let mut prefix = String::with_capacity(prefix_len);
    for (idx, token) in tokens.iter().rev().enumerate() {
        let text = token.text();
        prefix.push_str(if idx == 0 {
            &text[prefix_start..]
        } else {
            text
        });
    }

    if prefix.chars().all(char::is_whitespace) {
        prefix
    } else {
        " ".repeat(prefix.chars().count())
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;

    use crate::code_actions::test_utils::{
        apply_code_action, code_action_not_applicable, code_action_not_applicable_with_errors,
    };

    use super::convert_comment;

    #[test]
    fn line_to_block() {
        assert_snapshot!(apply_code_action(
            convert_comment,
            "
-- a com$0ment
select 1;
"
        ), @r"
        /* a comment */
        select 1;
        ");
    }

    #[test]
    fn block_to_line() {
        assert_snapshot!(apply_code_action(
            convert_comment,
            "
/* a com$0ment */
select 1;
"
        ), @r"
        -- a comment
        select 1;
        ");

        assert_snapshot!(apply_code_action(
            convert_comment,
            "
/** foo$0 */
select 2;
"
        ), @"
        -- foo
        select 2;
        ");
    }

    #[test]
    fn block_to_line_preserves_leading_stars() {
        assert_snapshot!(apply_code_action(
            convert_comment,
            "
/* **bo$0ld** */
select 1;
"
        ), @r"
        -- **bold**
        select 1;
        ");
    }

    #[test]
    fn multiline_block_to_lines() {
        assert_snapshot!(apply_code_action(
            convert_comment,
            "
  /* first$0 line
     second line
  */
select 1;
"
        ), @r"
          -- first line
          -- second line
        select 1;
        ");

        assert_snapshot!(apply_code_action(
            convert_comment,
            "
/*
first$0 line
second line
*/
select 1;
"
        ), @"
        -- first line
        -- second line
        select 1;
        ");
        assert_snapshot!(apply_code_action(
            convert_comment,
            "
--
-- first line$0
-- second line
--
select 1;
"
        ), @"
        /*
         * first line
         * second line
         */
        select 1;
        ");

        // Converting the generated block back should not leave empty comments
        // for the opening and closing delimiter lines.
        assert_snapshot!(apply_code_action(
            convert_comment,
            "
/*
 * select_window_$0clause
 * simple
 */
select 1 window w as ();
"
        ), @"
        -- select_window_clause
        -- simple
        select 1 window w as ();
        ");
    }

    #[test]
    fn preserves_crlf_and_cr_line_endings() {
        for line_ending in ["\r\n", "\r"] {
            let line_comments = "
--
-- first line$0
-- second line
--
select 1;
"
            .replace('\n', line_ending);
            let expected_block = "
/*
 * first line
 * second line
 */
select 1;
"
            .replace('\n', line_ending);
            assert_eq!(
                apply_code_action(convert_comment, &line_comments),
                expected_block
            );

            let block_comment = "
/*
first$0 line
second line
*/
select 1;
"
            .replace('\n', line_ending);
            let expected_lines = "
-- first line
-- second line
select 1;
"
            .replace('\n', line_ending);
            assert_eq!(
                apply_code_action(convert_comment, &block_comment),
                expected_lines
            );
        }
    }

    #[test]
    fn block_after_code_can_be_converted() {
        assert_snapshot!(apply_code_action(
            convert_comment,
            "
select 1; /* com$0ment */
select 2;
"
        ), @r"
        select 1; -- comment
        select 2;
        ");
    }

    #[test]
    fn block_before_code_cannot_be_converted() {
        assert!(code_action_not_applicable(
            convert_comment,
            "select /* com$0ment */ 1;"
        ));
        assert!(code_action_not_applicable(
            convert_comment,
            "/* com$0ment */ select 1;"
        ));
    }

    #[test]
    fn line_with_block_comment_delimiter_cannot_be_converted() {
        assert!(code_action_not_applicable(
            convert_comment,
            "
-- com$0ment /*
select 1;
"
        ));
        assert!(code_action_not_applicable_with_errors(
            convert_comment,
            "
-- com$0ment */
select 1;
"
        ));
    }

    #[test]
    fn not_applicable_outside_comment() {
        assert!(code_action_not_applicable(convert_comment, "select $01;"));
    }
}
