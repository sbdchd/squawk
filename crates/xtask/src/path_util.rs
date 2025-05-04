use std::io;

// via: https://github.com/rust-lang/cargo/blob/3fe68eabf93cbf3772bbcad09a9206c783e2de3f/crates/xtask-build-man/src/main.rs#L80-L84
///
/// Change to workspace root.
///
/// Assumed this xtask is located in `[WORKSPACE]/crates/xtask`.
pub(crate) fn cwd_to_workspace_root() -> io::Result<()> {
    let pkg_root = std::env!("CARGO_MANIFEST_DIR");
    let ws_root = format!("{pkg_root}/../..");
    std::env::set_current_dir(ws_root)
}
