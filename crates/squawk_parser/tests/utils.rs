use std::fmt::Write;

use squawk_parser::{parse, LexedStr};

pub fn parse_text(text: &str) -> (String, bool) {
    let lexed = LexedStr::new(text);
    let input = lexed.to_input();
    let output = parse(&input);

    let mut buf = String::new();
    let mut errors = Vec::new();
    let mut indent = String::new();
    let mut depth = 0;
    let mut len = 0;
    lexed.intersperse_trivia(&output, &mut |step| match step {
        squawk_parser::StrStep::Token { kind, text } => {
            assert!(depth > 0);
            len += text.len();
            writeln!(buf, "{indent}{kind:?} {text:?}").unwrap();
        }
        squawk_parser::StrStep::Enter { kind } => {
            assert!(depth > 0 || len == 0);
            depth += 1;
            writeln!(buf, "{indent}{kind:?}").unwrap();
            indent.push_str("  ");
        }
        squawk_parser::StrStep::Exit => {
            assert!(depth > 0);
            depth -= 1;
            indent.pop();
            indent.pop();
        }
        squawk_parser::StrStep::Error { msg, pos } => {
            assert!(depth > 0);
            let err = "ERROR";
            errors.push(format!("{err}@{pos}: {msg}\n"));
        }
    });
    assert_eq!(
        len,
        text.len(),
        "didn't parse all text.\nParsed:\n{}\n\nAll:\n{}\n",
        &text[..len],
        text
    );

    for (token, msg) in lexed.errors() {
        let pos = lexed.text_start(token);
        let err = "ERROR";
        errors.push(format!("{err}@{pos}: {msg}\n"));
    }

    let has_errors = !errors.is_empty();
    if has_errors {
        buf.push_str("---\n");
        for e in errors {
            buf.push_str(&e);
        }
    }
    (buf, has_errors)
}
