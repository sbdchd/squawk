use rowan::TextSize;

// TODO: we should probably use something else since `$0` is valid syntax, maybe `%0`?
const MARKER: &str = "$0";

#[track_caller]
pub(crate) fn fixture(sql: &str) -> (TextSize, String) {
    if let Some(pos) = sql.find(MARKER) {
        return (TextSize::new(pos as u32), sql.replace(MARKER, ""));
    }
    panic!("No marker found in SQL. Did you forget to add a marker `$0`?");
}
