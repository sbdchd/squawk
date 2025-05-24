use camino::Utf8PathBuf;

pub(crate) fn project_root() -> Utf8PathBuf {
    let binding = Utf8PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    Utf8PathBuf::from(binding.parent().unwrap().parent().unwrap())
}
