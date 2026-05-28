use std::ops::Range;

use crate::db::{Database, File, set_include_builtins};
use crate::file::InFile;
use rowan::TextSize;

// TODO: we should probably use something else since `$0` is valid syntax, maybe `%0`?
const MARKER: &str = "$0";

pub(crate) struct Fixture {
    marker_offset: TextSize,
    sql: String,
    db: Database,
    file: File,
}

pub(crate) struct Marker {
    file: File,
    offset: TextSize,
    offset_before: TextSize,
    range: Range<usize>,
}

impl Fixture {
    #[track_caller]
    pub(crate) fn new(sql: &str) -> Self {
        let fixture = Self::new_allow_errors(sql);
        assert_eq!(crate::db::parse(&fixture.db, fixture.file).errors(), vec![]);
        fixture
    }

    #[track_caller]
    pub(crate) fn new_allow_errors(sql: &str) -> Self {
        if let Some(pos) = sql.find(MARKER) {
            let sql = sql.replace(MARKER, "");
            let mut db = Database::default();
            if !sql.trim_start().starts_with("-- include-builtins") {
                set_include_builtins(&mut db, false);
            }
            let file = File::new(&db, sql.as_str().into());
            return Self {
                marker_offset: TextSize::new(pos as u32),
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

        TextSize::new(
            self.sql[..marker_offset]
                .char_indices()
                .last()
                .map_or(0, |(offset, _)| offset) as u32,
        )
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

#[cfg(test)]
mod tests {
    use super::Fixture;

    #[test]
    fn marker_tracks_offset_and_offset_before_for_multibyte_chars() {
        let fixture = Fixture::new("select 🦀$0");
        let marker = fixture.marker();

        assert_eq!(usize::from(marker.offset().value), "select 🦀".len());
        assert_eq!(
            marker.offset_before().value,
            ("select ".len() as u32).into()
        );
        assert_eq!(marker.range(), "select ".len().."select 🦀".len());
    }
}
