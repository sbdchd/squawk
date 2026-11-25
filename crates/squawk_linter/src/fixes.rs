use squawk_diagnostics::{Edit, Fix};
use squawk_rowan::TextSize;

pub(crate) fn add_concurrently(at: TextSize) -> Fix {
    let edit = Edit::insert(" concurrently", at);
    Fix::new("Add `concurrently`", vec![edit])
}
