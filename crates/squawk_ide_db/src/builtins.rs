#[cfg(not(target_arch = "wasm32"))]
use std::hash::{Hash, Hasher};

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
    let builtins = builtins_file(db);
    let builtins_sql = builtins.content(db);

    let mut hasher = rustc_hash::FxHasher::default();
    builtins_sql.hash(&mut hasher);
    let key = format!("{:016x}", hasher.finish());

    let strategy = etcetera::base_strategy::choose_base_strategy().ok()?;
    let dir = strategy.cache_dir().join("squawk/stubs").join(key);
    let path = dir.join("builtins.sql");

    if !path.is_file() {
        std::fs::create_dir_all(&dir).ok()?;
        let tmp = dir.join(format!("builtins.sql.{}.tmp", std::process::id()));
        std::fs::write(&tmp, builtins_sql.as_ref()).ok()?;
        if std::fs::rename(&tmp, &path).is_err() {
            let _ = std::fs::remove_file(&tmp);
            if !path.is_file() {
                return None;
            }
        }
    }

    Url::from_file_path(path).ok()
}
