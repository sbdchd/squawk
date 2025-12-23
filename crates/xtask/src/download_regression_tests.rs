use crate::path::project_root;
use anyhow::{Result, bail};
use camino::Utf8PathBuf;
use regex::Regex;
use std::fs::{File, create_dir_all, remove_dir_all};
use std::io::{BufRead, Write};
use std::process::Command;

const PROCESSED_OUTPUT_DIR: &str = "crates/squawk_parser/tests/data/regression_suite";

const START_END_MARKERS: &[(&str, &str)] = &[
    (
        "MERGE INTO target t RANDOMWORD",
        "\tUPDATE SET balance = 0;",
    ),
    (
        "-- incorrectly specifying INTO target",
        "\tINSERT INTO target DEFAULT VALUES;",
    ),
    ("-- Multiple VALUES clause", "\tINSERT VALUES (1,1), (2,2);"),
    ("-- SELECT query for INSERT", "\tINSERT SELECT (1, 1);"),
    ("-- UPDATE tablename", "\tUPDATE target SET balance = 0;"),
];

const IGNORED_LINES: &[&str] = &[
    r#"SELECT rank() OVER (PARTITION BY four, ORDER BY ten) FROM tenk1;"#,
    r#"SELECT q.* FROM (SELECT * FROM test_tablesample) as q TABLESAMPLE BERNOULLI (5);"#,
    r#"CREATE SEQUENCE tableam_seq_heap2 USING heap2;"#,
    "CREATE VIEW tableam_view_heap2 USING heap2 AS SELECT * FROM tableam_tbl_heap2;",
    "SELECT INTO tableam_tblselectinto_heap2 USING heap2 FROM tableam_tbl_heap2;",
    "INSERT INTO foo DEFAULT VALUES RETURNING WITH (nonsuch AS something) *;",
    "SELECT 0.0e;",
    "SELECT 0.0e+a;",
    "SELECT 0b;",
    "SELECT 0o;",
    "SELECT 0x;",
    "SELECT _1_000.5;",
    "EXPLAIN (COSTS OFF) :qry;",
    ":qry;",
    "create table foo (with baz);",
    "create table foo (with ordinality);",
    ":show_data;",
    "alter trigger a on only grandparent rename to b;	-- ONLY not supported",
    "CREATE SUBSCRIPTION regress_testsub CONNECTION 'foo';",
    "CREATE SUBSCRIPTION regress_testsub PUBLICATION foo;",
    "SELECT U&'wrong: +0061' UESCAPE +;",
    "CREATE STATISTICS tst;",
    "CREATE STATISTICS tst ON a, b;",
    "CREATE STATISTICS tst ON a FROM (VALUES (x)) AS foo;",
    "CREATE STATISTICS tst ON a FROM foo NATURAL JOIN bar;",
    "CREATE STATISTICS tst ON a FROM (SELECT * FROM ext_stats_test) AS foo;",
    "CREATE STATISTICS tst ON a FROM ext_stats_test s TABLESAMPLE system (x);",
    "CREATE STATISTICS tst ON a FROM XMLTABLE('foo' PASSING 'bar' COLUMNS a text);",
    "CREATE STATISTICS tst ON a FROM JSON_TABLE(jsonb '123', '$' COLUMNS (item int));",
    "CREATE STATISTICS alt_stat2 ON a FROM tftest(1);",
    "ALTER STATISTICS IF EXISTS ab1_a_b_stats SET STATISTICS 0;",
    "CHECKPOINT (WRONG);",
    "CHECKPOINT (MODE WRONG);",
    "CHECKPOINT (MODE FAST, FLUSH_UNLOGGED FALSE);",
    "CHECKPOINT (FLUSH_UNLOGGED);",
    "ALTER PUBLICATION testpub1_forschema ADD TABLES IN SCHEMA foo (a, b);",
    "CREATE SCHEMA IF NOT EXISTS test_ns_schema_renamed -- fail, disallowed",
    "insert into insertconflicttest values (1) on conflict (key int4_ops (fillfactor=10)) do nothing;",
    "insert into insertconflicttest values (1) on conflict (key asc) do nothing;",
    "insert into insertconflicttest values (1) on conflict (key nulls last) do nothing;",
    "ALTER USER MAPPING FOR user SERVER ss4 OPTIONS (gotcha 'true'); -- ERROR",
    "ALTER FOREIGN DATA WRAPPER foo;                             -- ERROR",
    "ALTER SERVER s0;                                            -- ERROR",
    "ALTER USER MAPPING FOR user SERVER ss4 OPTIONS (gotcha 'true'); -- ERROR",
    "alter table atacc1 SET WITH OIDS;",
    "alter table atacc1 drop xmin;",
    "create view myview as select * from atacc1;",
    "CREATE INDEX IF NOT EXISTS ON onek USING btree(unique1 int4_ops);",
    "SELECT 10 !=-;",
    "CREATE TABLE withoid() WITH OIDS;",
    "update dposintatable set (f1[2])[1] = array[98];",
    "CREATE FOREIGN TABLE ft1 ();                                    -- ERROR",
    r#"select 'a\\bcd' as f1, 'a\\b\'cd' as f2, 'a\\b\'''cd' as f3, 'abcd\\'   as f4, 'ab\\\'cd' as f5, '\\\\' as f6;"#,
    r#"select 'a\\bcd' as f1, 'a\\b\'cd' as f2, 'a\\b\'''cd' as f3, 'abcd\\'   as f4, 'ab\\\'cd' as f5, '\\\\' as f6;"#,
    "copy (select * from test1) (t,id) to stdout;",
];

const VARIABLE_REPLACEMENTS: &[(&str, &str)] = &[
    (":reltoastname", "reltoastname"),
    (":temp_schema_name", "temp_schema_name"),
    (":toastrel", "toastrel"),
    (":newloid", "10101"),
    (r#" :""#, r#" ""#),
];

const GSET_REPLACEMENTS: &[(&str, &str)] = &[
    (
        "\\gset my_io_sum_shared_before_",
        "/* \\gset my_io_sum_shared_before_ */;",
    ),
    (
        "\\gset io_sum_shared_before_",
        "/* \\gset io_sum_shared_before_ */;",
    ),
    (
        "\\gset io_sum_wal_normal_before_",
        "/* \\gset io_sum_wal_normal_before_ */;",
    ),
];

pub(crate) fn download_regression_tests() -> Result<()> {
    let temp_dir = download_regression_suite()?;
    transform_regression_suite(&temp_dir)?;
    Ok(())
}

fn download_regression_suite() -> Result<Utf8PathBuf> {
    let target_dir = Utf8PathBuf::try_from(std::env::temp_dir())
        .map_err(|_| anyhow::anyhow!("temp dir path is not valid UTF-8"))?
        .join("squawk_raw_regression_suite");

    if target_dir.exists() {
        println!("Cleaning temp directory: {target_dir:?}");
        remove_dir_all(&target_dir)?;
    }

    create_dir_all(&target_dir)?;

    let urls = fetch_download_urls()?;
    let total_files = urls.len();

    for (index, url) in urls.iter().enumerate() {
        let filename = url.split('/').next_back().unwrap();
        if filename.contains("psql") {
            continue;
        }
        let filepath = target_dir.join(filename);

        println!(
            "[{}/{}] Downloading {}... ",
            index + 1,
            total_files,
            filename
        );

        let output = Command::new("curl").args(["-s", url]).output()?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            bail!("Failed to download '{}': {}", url, error_msg);
        }

        File::create(&filepath)?.write_all(&output.stdout)?;
    }

    Ok(target_dir)
}

fn transform_regression_suite(input_dir: &Utf8PathBuf) -> Result<()> {
    let output_dir = project_root().join(PROCESSED_OUTPUT_DIR);

    if output_dir.exists() {
        println!("Cleaning target directory: {output_dir:?}");
        remove_dir_all(&output_dir)?;
    }

    create_dir_all(&output_dir)?;

    let mut files: Vec<Utf8PathBuf> = vec![];
    for entry in std::fs::read_dir(input_dir)? {
        let entry = entry?;
        let path = Utf8PathBuf::try_from(entry.path())?;
        if path.extension() == Some("sql") {
            files.push(path);
        }
    }

    files.sort();
    let total_files = files.len();

    for (index, input_path) in files.iter().enumerate() {
        let filename = input_path.file_name().unwrap();
        let output_path = output_dir.join(filename);

        println!("[{}/{}] Processing {}...", index + 1, total_files, filename);

        let input_file = File::open(input_path)?;
        let reader = std::io::BufReader::new(input_file);
        let mut processed_content = vec![];

        if let Err(e) = preprocess_sql(reader, &mut processed_content) {
            eprintln!("Error: Failed to process file: {e}");
            continue;
        }

        let mut dest = File::create(&output_path)?;
        dest.write_all(&processed_content)?;
    }

    Ok(())
}

fn fetch_download_urls() -> Result<Vec<String>> {
    println!("Fetching SQL file URLs...");
    let output = Command::new("gh")
        .args([
            "api",
            "-H",
            "Accept: application/vnd.github+json",
            "/repos/postgres/postgres/contents/src/test/regress/sql",
        ])
        .output()?;

    if !output.status.success() {
        bail!(
            "Failed to fetch SQL files: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let json_str = String::from_utf8(output.stdout)?;
    let files: Vec<serde_json::Value> = serde_json::from_str(&json_str)?;

    let urls: Vec<String> = files
        .into_iter()
        .filter(|file| {
            file["name"]
                .as_str()
                .is_some_and(|name| name.ends_with(".sql"))
        })
        .filter_map(|file| file["download_url"].as_str().map(String::from))
        .collect();

    if urls.is_empty() {
        bail!("No SQL files found");
    }

    Ok(urls)
}

// The regression suite from postgres has a mix of valid and invalid sql. We
// don't have a good way to determine what is what, so we munge the data to
// comment out any problematic code.
pub(crate) fn preprocess_sql<R: BufRead, W: Write>(source: R, mut dest: W) -> Result<()> {
    let template_vars_regex = Regex::new(r"^:'([^']+)'|^:([a-zA-Z_][a-zA-Z0-9_]*)").unwrap();
    let mut in_copy_stdin = false;
    let mut in_bogus_cases = false;
    let mut in_copy_select_input = false;
    let mut looking_for_end: Option<&str> = None;

    for line in source.lines() {
        let mut line = line?;
        let mut should_comment = false;

        if line.contains("bogus cases") {
            in_bogus_cases = true;
        } else if line.is_empty() {
            in_bogus_cases = false;
        }

        if line.contains("copy test3 from stdin\\;") {
            in_copy_select_input = true;
        } else if line.contains("select * from test3") {
            in_copy_select_input = false;
        }

        for &(start, end) in START_END_MARKERS {
            if line.contains(start) {
                looking_for_end = Some(end);
            }
        }

        if let Some(end) = looking_for_end {
            should_comment = true;
            if line.contains(end) {
                looking_for_end = None;
            }
        }

        let line_lower = line.to_lowercase();
        if (line_lower.starts_with("copy ") || line_lower.starts_with("\\copy"))
            && (line_lower.contains("from stdin") || line_lower.contains("from stdout"))
        {
            in_copy_stdin = true;
            if line.starts_with("\\copy") {
                should_comment = true;
            }
        } else if in_copy_stdin {
            if line == "\\."
                || line.starts_with("--")
                || ["copy", "begin", "rollback", "select"]
                    .iter()
                    .any(|prefix| line_lower.starts_with(prefix))
            {
                in_copy_stdin = false;
            }
            should_comment = true;
        } else if (line.trim_start().starts_with('\\') && !line.contains("\\gset"))
            || line.starts_with("'show_data'")
            || line.starts_with(':')
        {
            should_comment = true;
        }

        if in_bogus_cases || in_copy_select_input {
            should_comment = true;
        }

        if IGNORED_LINES.iter().any(|&prefix| line.starts_with(prefix)) {
            should_comment = true;
        }

        if line.contains("\\;") || line.starts_with("**") {
            should_comment = true;
        }

        if should_comment {
            line = format!("-- {line}");
        }

        for &(from, to) in GSET_REPLACEMENTS {
            line = line.replace(from, to);
        }

        line = line.replace(
            "FROM generate_series(1, 1100) g(i)",
            "FROM generate_series(1, 1100) g(i);",
        );

        for &(from, to) in VARIABLE_REPLACEMENTS {
            line = line.replace(from, to);
        }

        if line.contains("\\gset") {
            if let Some(start) = line.find("\\gset") {
                let end = line[start..]
                    .find('\n')
                    .map(|i| start + i)
                    .unwrap_or(line.len());
                let gset_cmd = line[start..end].trim_end();
                line = format!("{}/* {} */;{}", &line[..start], gset_cmd, &line[end..]);
            }
        }

        if line.trim_start().starts_with("--") {
            writeln!(dest, "{line}")?;
            continue;
        }

        let processed = replace_template_vars(&line, &template_vars_regex)?;
        writeln!(dest, "{processed}")?;
    }

    Ok(())
}

fn replace_template_vars(line: &str, template_vars_regex: &Regex) -> Result<String> {
    let mut result = String::new();
    let mut char_indices = line.char_indices().peekable();
    let mut in_single_quote = false;
    let mut in_double_quote = false;
    let mut in_array = false;

    while let Some((byte_pos, c)) = char_indices.next() {
        match c {
            '\'' => {
                result.push(c);
                in_single_quote = !in_single_quote;
            }
            '"' => {
                result.push(c);
                in_double_quote = !in_double_quote;
            }
            '[' => {
                result.push(c);
                in_array = true;
            }
            ']' => {
                result.push(c);
                in_array = false;
            }
            ':' if !in_single_quote && !in_double_quote && !in_array => {
                if let Some(&(_, next_c)) = char_indices.peek() {
                    if next_c == ':' {
                        result.push_str("::");
                        char_indices.next();
                        continue;
                    }
                    if next_c == '=' {
                        result.push_str(":=");
                        char_indices.next();
                        continue;
                    }
                }

                let remaining = &line[byte_pos..];
                if let Some(caps) = template_vars_regex.captures(remaining) {
                    let full = caps.get(0).unwrap();
                    let m = caps.get(1).or_else(|| caps.get(2)).unwrap();
                    let matched_var = &remaining[m.start()..m.end()];

                    result.push('\'');
                    result.push_str(matched_var);
                    result.push('\'');

                    let skip_bytes = full.end() - c.len_utf8();
                    let mut skipped = 0;
                    while skipped < skip_bytes {
                        if let Some((_, ch)) = char_indices.next() {
                            skipped += ch.len_utf8();
                        } else {
                            break;
                        }
                    }
                    continue;
                }
                result.push(c);
            }
            _ => result.push(c),
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    fn test_preprocess_sql(sql: &str) -> Result<String> {
        let input = sql.as_bytes();
        let mut output = Vec::new();
        let cursor = Cursor::new(input);
        preprocess_sql(cursor, &mut output)?;
        String::from_utf8(output).map_err(Into::into)
    }

    #[test]
    fn test_replacement() {
        let cases = [
            (
                "SELECT * FROM foo WHERE bar = :'foo' AND baz = :baz;",
                "SELECT * FROM foo WHERE bar = 'foo' AND baz = 'baz';",
            ),
            (
                "select array_dims('{1,2,3}'::dia);",
                "select array_dims('{1,2,3}'::dia);",
            ),
            (
                "SELECT to_char(now(), 'OF') as \"OF\", to_char(now(), 'TZH:TZM') as \"TZH:TZM\";",
                "SELECT to_char(now(), 'OF') as \"OF\", to_char(now(), 'TZH:TZM') as \"TZH:TZM\";",
            ),
            (
                "SELECT ('{{{1},{2},{3}},{{4},{5},{6}}}'::int[])[1][1:NULL][1];",
                "SELECT ('{{{1},{2},{3}},{{4},{5},{6}}}'::int[])[1][1:NULL][1];",
            ),
            ("d := $1::di;", "d := $1::di;"),
            (
                "SELECT JSON_OBJECT('foo': NULL::int FORMAT JSON);",
                "SELECT JSON_OBJECT('foo': NULL::int FORMAT JSON);",
            ),
            (
                r#"ALTER DATABASE :"datname" REFRESH COLLATION VERSION;"#,
                r#"ALTER DATABASE "datname" REFRESH COLLATION VERSION;"#,
            ),
            (
                "-- comment with :placeholder should not be replaced",
                "-- comment with :placeholder should not be replaced",
            ),
            (
                "  -- indented comment with :foo",
                "  -- indented comment with :foo",
            ),
            (
                "SELECT 'ὀδυσσεύς' = 'ὈΔΥΣΣΕΎΣ' COLLATE case_sensitive;",
                "SELECT 'ὀδυσσεύς' = 'ὈΔΥΣΣΕΎΣ' COLLATE case_sensitive;",
            ),
            (
                "SELECT 'ὀδυσσεύς' WHERE name = :greek_name;",
                "SELECT 'ὀδυσσεύς' WHERE name = 'greek_name';",
            ),
        ];

        for (input, expected) in &cases {
            let result = test_preprocess_sql(input).unwrap();
            assert_eq!(result, format!("{}\n", *expected));
        }
    }
}
