use std::ops::Range;

use annotate_snippets::{AnnotationKind, Level, Renderer, Snippet, renderer::DecorStyle};
use rowan::TextSize;
use squawk_sema::builtins::BUILTINS_SQL;
use squawk_sema::extensions::BundledExtension;
use squawk_sema::file::{FileId, FileIndex, FilePosition, FileRange, InFile};
use squawk_sema::semantic_index::SemanticIndex;
use squawk_sema::semantic_index_builder::SemanticIndexBuilder;
use squawk_syntax::SourceFile;

use crate::db::Database;
use crate::hover::hover_with_index as hover;
use crate::offsets::item_at_position;

// TODO: we should probably use something else since `$0` is valid syntax, maybe `%0`?
const MARKER: &str = "$0";

pub(crate) struct Fixture {
    marker_offset: TextSize,
    sql: String,
    db: Database,
    file: FileIndex,
}

pub(crate) struct Marker {
    file: FileIndex,
    offset: TextSize,
    offset_before: TextSize,
    range: Range<usize>,
}

impl Fixture {
    #[track_caller]
    pub(crate) fn new(sql: &str) -> Self {
        let fixture = Self::new_allow_errors(sql);
        assert_eq!(
            crate::db::parse(&fixture.db, fixture.file.into()).errors(),
            vec![]
        );
        fixture
    }

    #[track_caller]
    pub(crate) fn new_allow_errors(sql: &str) -> Self {
        if let Some(pos) = sql.find(MARKER) {
            let sql = sql.replace(MARKER, "");
            let mut db = Database::default();
            let file = db.add_file(sql.as_str().into());
            return Self {
                marker_offset: TextSize::try_from(pos).unwrap(),
                sql,
                db,
                file,
            };
        }
        panic!("No marker found in SQL. Did you forget to add a marker `$0`?");
    }

    pub(crate) fn marker(&self) -> Marker {
        let offset_before = self.offset_before_marker();
        Marker {
            file: self.file,
            offset: self.marker_offset,
            offset_before,
            range: self.char_span_at(offset_before),
        }
    }

    fn offset_before_marker(&self) -> TextSize {
        let marker_offset: usize = self.marker_offset.into();
        if marker_offset == 0 {
            return 0.into();
        }

        TextSize::try_from(
            self.sql[..marker_offset]
                .char_indices()
                .last()
                .map_or(0, |(offset, _)| offset),
        )
        .unwrap()
    }

    fn char_span_at(&self, offset: TextSize) -> Range<usize> {
        let start: usize = offset.into();
        let len = self.sql[start..].chars().next().map_or(0, char::len_utf8);
        start..start + len
    }

    pub(crate) fn db(&self) -> &Database {
        &self.db
    }
}

impl Marker {
    pub(crate) fn offset(&self) -> InFile<TextSize> {
        InFile::new(self.file, self.offset)
    }

    pub(crate) fn offset_before(&self) -> InFile<TextSize> {
        InFile::new(self.file, self.offset_before)
    }

    pub(crate) fn range(&self) -> Range<usize> {
        self.range.clone()
    }
}

#[track_caller]
pub(crate) fn render_goto<T: IntoIterator<Item = FileRange>>(
    files: &[&str],
    title: &str,
    goto: impl Fn(&SemanticIndex, &SourceFile, FilePosition) -> T,
) -> Option<String> {
    render_targets(files, title, "destination", goto)
}

#[track_caller]
pub(crate) fn render_hover(files: &[&str]) -> Option<String> {
    let fixture = SemanticFixture::new(files);
    let file_index = fixture
        .sources
        .iter()
        .position(|file| file.0 == fixture.position.file_id)
        .unwrap();
    let file = &fixture.sources[file_index];
    let hover = hover(&fixture.index, &file.2, fixture.position)?;
    let range = hover.range.value;
    let snippet = Snippet::source(file.1.as_str())
        .path(format!("file{file_index}.sql"))
        .fold(true)
        .annotation(
            AnnotationKind::Context
                .span(range.start().into()..range.end().into())
                .label("hover"),
        );
    let title = format!("hover: {}", hover.snippet);
    let mut group = Level::INFO.primary_title(&title).element(snippet);
    if let Some(comment) = &hover.comment {
        group = group.element(Level::NOTE.message(comment));
    }
    Some(
        Renderer::plain()
            .decor_style(DecorStyle::Unicode)
            .render(&[group])
            .replace("info: hover:", "hover:"),
    )
}

struct SemanticFixture {
    _db: Database,
    sources: Vec<(FileId, String, SourceFile)>,
    position: FilePosition,
    index: SemanticIndex,
}

impl SemanticFixture {
    #[track_caller]
    fn new(files: &[&str]) -> SemanticFixture {
        let mut db = Database::default();
        let mut cursor = None;
        let mut sources = vec![];
        for sql in files {
            let file_id = db.add_file(sql.replace("$0", "").into()).into();
            if let Some(offset) = sql.find("$0") {
                let before = sql[..offset]
                    .char_indices()
                    .last()
                    .map_or(0, |(offset, _)| offset);
                cursor = Some(InFile::new(file_id, TextSize::try_from(before).unwrap()));
            }
            let sql = sql.replace("$0", "");
            let parse = SourceFile::parse(&sql);
            assert_eq!(parse.errors(), vec![]);
            sources.push((file_id, sql, parse.tree()));
        }
        let position = cursor.expect("missing $0 cursor marker");
        let mut builder = SemanticIndexBuilder::new();
        for (file_id, _, tree) in &sources {
            builder.add_file(InFile::new(*file_id, tree));
        }
        let index = builder.build();
        SemanticFixture {
            _db: db,
            sources,
            position,
            index,
        }
    }
}

#[track_caller]
pub(crate) fn render_targets<T: IntoIterator<Item = FileRange>>(
    files: &[&str],
    title: &str,
    target_label: &str,
    goto: impl Fn(&SemanticIndex, &SourceFile, FilePosition) -> T,
) -> Option<String> {
    let SemanticFixture {
        sources,
        position,
        index,
        _db,
    } = SemanticFixture::new(files);

    let file = &sources
        .iter()
        .find(|file| file.0 == position.file_id)
        .unwrap()
        .2;
    let targets: Vec<FileRange> = goto(&index, file, position).into_iter().collect();
    if targets.is_empty() {
        return None;
    }
    let source = item_at_position(&index, file, position)
        .unwrap()
        .map(|item| item.range);

    let paths: Vec<String> = (0..files.len())
        .map(|index| format!("file{index}.sql"))
        .collect();
    let snippet = |file_id: FileId| {
        let (source, path) = if file_id == FileId::Builtins {
            (BUILTINS_SQL, "builtins.sql")
        } else if let Some(extension) = BundledExtension::from_file(file_id) {
            (extension.sql(), extension.path())
        } else {
            let index = sources.iter().position(|file| file.0 == file_id).unwrap();
            (sources[index].1.as_str(), paths[index].as_str())
        };
        Snippet::source(source).path(path).fold(true)
    };
    let labels: Vec<String> = (0..targets.len())
        .map(|index| format!("{}. {target_label}", index + 2))
        .collect();
    let mut file_ids = vec![source.file_id];
    for target in &targets {
        if !file_ids.contains(&target.file_id) {
            file_ids.push(target.file_id);
        }
    }
    let snippets: Vec<_> = file_ids
        .iter()
        .map(|&file_id| {
            let mut snippet = snippet(file_id);
            if file_id == source.file_id {
                snippet = snippet.annotation(
                    AnnotationKind::Context
                        .span(source.value.start().into()..source.value.end().into())
                        .label("1. source"),
                );
            }
            for (target, label) in targets.iter().zip(&labels) {
                if target.file_id == file_id {
                    snippet = snippet.annotation(
                        AnnotationKind::Context
                            .span(target.value.start().into()..target.value.end().into())
                            .label(label),
                    );
                }
            }
            snippet
        })
        .collect();
    Some(
        Renderer::plain()
            .decor_style(DecorStyle::Unicode)
            .render(&[Level::INFO.primary_title(title).elements(snippets)]),
    )
}

#[cfg(test)]
mod tests {
    use super::{Fixture, TextSize};

    #[test]
    fn marker_tracks_offset_and_offset_before_for_multibyte_chars() {
        let fixture = Fixture::new("select 🦀$0");
        let marker = fixture.marker();

        assert_eq!(usize::from(marker.offset().value), "select 🦀".len());
        assert_eq!(marker.offset_before().value, TextSize::of("select "));
        assert_eq!(marker.range(), "select ".len().."select 🦀".len());
    }
}
