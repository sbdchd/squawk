#[cfg(not(target_arch = "wasm32"))]
use etcetera::BaseStrategy;
use line_index::LineIndex;
use salsa::Database as Db;
use squawk_syntax::{Parse, SourceFile};
#[cfg(not(target_arch = "wasm32"))]
use url::Url;

use crate::binder::{self, Binder};

pub(crate) const BUILTINS_SQL: &str = include_str!("generated/builtins.sql");

#[salsa::tracked]
pub fn parse_builtins(_db: &dyn Db) -> Parse<SourceFile> {
    SourceFile::parse(BUILTINS_SQL)
}

#[salsa::tracked]
pub fn builtins_line_index(_db: &dyn Db) -> LineIndex {
    LineIndex::new(BUILTINS_SQL)
}

#[salsa::tracked]
pub fn builtins_binder(db: &dyn Db) -> Binder {
    let builtins_tree = parse_builtins(db).tree();
    binder::bind(&builtins_tree)
}

#[cfg(not(target_arch = "wasm32"))]
#[salsa::tracked]
pub fn builtins_url(_db: &dyn Db) -> Option<Url> {
    let strategy = etcetera::base_strategy::choose_base_strategy().ok()?;
    let config_dir = strategy.config_dir();
    let cache_dir = config_dir.join("squawk/stubs");
    let path = cache_dir.join("builtins.sql");
    std::fs::create_dir_all(&cache_dir).ok()?;
    std::fs::write(&path, BUILTINS_SQL).ok()?;
    Url::from_file_path(path).ok()
}

#[cfg(test)]
mod test {
    use squawk_syntax::ast;

    use crate::builtins::BUILTINS_SQL;

    #[test]
    fn no_errors() {
        let parse = ast::SourceFile::parse(BUILTINS_SQL);
        assert_eq!(parse.errors(), vec![]);
    }
}
