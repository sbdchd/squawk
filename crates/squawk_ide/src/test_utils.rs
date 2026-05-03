use std::ops::Range;

use rowan::TextSize;

// TODO: we should probably use something else since `$0` is valid syntax, maybe `%0`?
const MARKER: &str = "$0";

pub(crate) struct Fixture {
    marker_offset: TextSize,
    sql: String,
}

pub(crate) struct Marker {
    offset: TextSize,
    offset_before: TextSize,
    range: Range<usize>,
}

impl Fixture {
    #[track_caller]
    pub(crate) fn new(sql: &str) -> Self {
        if let Some(pos) = sql.find(MARKER) {
            return Self {
                marker_offset: TextSize::new(pos as u32),
                sql: sql.replace(MARKER, ""),
            };
        }
        panic!("No marker found in SQL. Did you forget to add a marker `$0`?");
    }

    pub(crate) fn marker(&self) -> Marker {
        let offset_before = self.offset_before_marker();
        Marker {
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

    pub(crate) fn sql(&self) -> &str {
        &self.sql
    }
}

impl Marker {
    pub(crate) fn offset(&self) -> TextSize {
        self.offset
    }

    pub(crate) fn offset_before(&self) -> TextSize {
        self.offset_before
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

        assert_eq!(usize::from(marker.offset()), "select 🦀".len());
        assert_eq!(marker.offset_before(), ("select ".len() as u32).into());
        assert_eq!(marker.range(), "select ".len().."select 🦀".len());
    }
}
