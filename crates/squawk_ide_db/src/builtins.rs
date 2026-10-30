#[cfg(not(target_arch = "wasm32"))]
use etcetera::BaseStrategy;
use squawk_sema::builtins::BUILTINS_SQL;
use squawk_sema::file::FileId;
#[cfg(not(target_arch = "wasm32"))]
use url::Url;

use crate::{Db, File};

#[salsa::tracked]
pub fn builtins_file(db: &dyn Db) -> File {
    File::new(db, BUILTINS_SQL.into(), FileId::Builtins)
}

#[cfg(not(target_arch = "wasm32"))]
#[salsa::tracked]
pub fn builtins_url(db: &dyn Db) -> Option<Url> {
    let strategy = etcetera::base_strategy::choose_base_strategy().ok()?;
    let config_dir = strategy.config_dir();
    let cache_dir = config_dir.join("squawk/stubs");
    let path = cache_dir.join("builtins.sql");
    std::fs::create_dir_all(&cache_dir).ok()?;
    let builtins = builtins_file(db);
    let builtins_sql = builtins.content(db);
    std::fs::write(&path, builtins_sql.as_ref()).ok()?;
    Url::from_file_path(path).ok()
}
