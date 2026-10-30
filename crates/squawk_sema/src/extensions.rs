use std::sync::OnceLock;

use squawk_syntax::{Parse, SourceFile};

use crate::file::FileId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BundledExtension {
    Bloom,
    BtreeGin,
    BtreeGist,
    Citext,
    Cube,
    H3,
    Hll,
    Hstore,
    Isn,
    Ltree,
    PgStatStatements,
    PgTrgm,
    PgWalinspect,
    Pgcrypto,
    Postgis,
    PostgresFdw,
    Tablefunc,
    Vector,
}

static PARSES: [OnceLock<Parse<SourceFile>>; BundledExtension::COUNT as usize] =
    [const { OnceLock::new() }; BundledExtension::COUNT as usize];

impl BundledExtension {
    pub(crate) const COUNT: u32 = 18;

    #[cfg(test)]
    pub(crate) const ALL: [BundledExtension; BundledExtension::COUNT as usize] = [
        BundledExtension::Bloom,
        BundledExtension::BtreeGin,
        BundledExtension::BtreeGist,
        BundledExtension::Citext,
        BundledExtension::Cube,
        BundledExtension::H3,
        BundledExtension::Hll,
        BundledExtension::Hstore,
        BundledExtension::Isn,
        BundledExtension::Ltree,
        BundledExtension::PgStatStatements,
        BundledExtension::PgTrgm,
        BundledExtension::PgWalinspect,
        BundledExtension::Pgcrypto,
        BundledExtension::Postgis,
        BundledExtension::PostgresFdw,
        BundledExtension::Tablefunc,
        BundledExtension::Vector,
    ];

    pub(crate) fn from_name(name: &str) -> Option<BundledExtension> {
        Some(match name {
            "bloom" => BundledExtension::Bloom,
            "btree_gin" => BundledExtension::BtreeGin,
            "btree_gist" => BundledExtension::BtreeGist,
            "citext" => BundledExtension::Citext,
            "cube" => BundledExtension::Cube,
            "h3" => BundledExtension::H3,
            "hll" => BundledExtension::Hll,
            "hstore" => BundledExtension::Hstore,
            "isn" => BundledExtension::Isn,
            "ltree" => BundledExtension::Ltree,
            "pg_stat_statements" => BundledExtension::PgStatStatements,
            "pg_trgm" => BundledExtension::PgTrgm,
            "pg_walinspect" => BundledExtension::PgWalinspect,
            "pgcrypto" => BundledExtension::Pgcrypto,
            "postgis" => BundledExtension::Postgis,
            "postgres_fdw" => BundledExtension::PostgresFdw,
            "tablefunc" => BundledExtension::Tablefunc,
            "vector" => BundledExtension::Vector,
            _ => return None,
        })
    }

    pub fn from_file(file_id: FileId) -> Option<BundledExtension> {
        match file_id {
            FileId::Extension(extension) => Some(extension),
            _ => None,
        }
    }

    pub(crate) fn file_id(self) -> FileId {
        FileId::Extension(self)
    }

    #[cfg(test)]
    pub(crate) fn name(self) -> &'static str {
        match self {
            BundledExtension::Bloom => "bloom",
            BundledExtension::BtreeGin => "btree_gin",
            BundledExtension::BtreeGist => "btree_gist",
            BundledExtension::Citext => "citext",
            BundledExtension::Cube => "cube",
            BundledExtension::H3 => "h3",
            BundledExtension::Hll => "hll",
            BundledExtension::Hstore => "hstore",
            BundledExtension::Isn => "isn",
            BundledExtension::Ltree => "ltree",
            BundledExtension::PgStatStatements => "pg_stat_statements",
            BundledExtension::PgTrgm => "pg_trgm",
            BundledExtension::PgWalinspect => "pg_walinspect",
            BundledExtension::Pgcrypto => "pgcrypto",
            BundledExtension::Postgis => "postgis",
            BundledExtension::PostgresFdw => "postgres_fdw",
            BundledExtension::Tablefunc => "tablefunc",
            BundledExtension::Vector => "vector",
        }
    }

    pub fn path(self) -> &'static str {
        match self {
            BundledExtension::Bloom => "bloom.sql",
            BundledExtension::BtreeGin => "btree_gin.sql",
            BundledExtension::BtreeGist => "btree_gist.sql",
            BundledExtension::Citext => "citext.sql",
            BundledExtension::Cube => "cube.sql",
            BundledExtension::H3 => "h3.sql",
            BundledExtension::Hll => "hll.sql",
            BundledExtension::Hstore => "hstore.sql",
            BundledExtension::Isn => "isn.sql",
            BundledExtension::Ltree => "ltree.sql",
            BundledExtension::PgStatStatements => "pg_stat_statements.sql",
            BundledExtension::PgTrgm => "pg_trgm.sql",
            BundledExtension::PgWalinspect => "pg_walinspect.sql",
            BundledExtension::Pgcrypto => "pgcrypto.sql",
            BundledExtension::Postgis => "postgis.sql",
            BundledExtension::PostgresFdw => "postgres_fdw.sql",
            BundledExtension::Tablefunc => "tablefunc.sql",
            BundledExtension::Vector => "vector.sql",
        }
    }

    pub fn sql(self) -> &'static str {
        match self {
            BundledExtension::Bloom => include_str!("generated/extensions/bloom.sql"),
            BundledExtension::BtreeGin => include_str!("generated/extensions/btree_gin.sql"),
            BundledExtension::BtreeGist => include_str!("generated/extensions/btree_gist.sql"),
            BundledExtension::Citext => include_str!("generated/extensions/citext.sql"),
            BundledExtension::Cube => include_str!("generated/extensions/cube.sql"),
            BundledExtension::H3 => include_str!("generated/extensions/h3.sql"),
            BundledExtension::Hll => include_str!("generated/extensions/hll.sql"),
            BundledExtension::Hstore => include_str!("generated/extensions/hstore.sql"),
            BundledExtension::Isn => include_str!("generated/extensions/isn.sql"),
            BundledExtension::Ltree => include_str!("generated/extensions/ltree.sql"),
            BundledExtension::PgStatStatements => {
                include_str!("generated/extensions/pg_stat_statements.sql")
            }
            BundledExtension::PgTrgm => include_str!("generated/extensions/pg_trgm.sql"),
            BundledExtension::PgWalinspect => {
                include_str!("generated/extensions/pg_walinspect.sql")
            }
            BundledExtension::Pgcrypto => include_str!("generated/extensions/pgcrypto.sql"),
            BundledExtension::Postgis => include_str!("generated/extensions/postgis.sql"),
            BundledExtension::PostgresFdw => include_str!("generated/extensions/postgres_fdw.sql"),
            BundledExtension::Tablefunc => include_str!("generated/extensions/tablefunc.sql"),
            BundledExtension::Vector => include_str!("generated/extensions/vector.sql"),
        }
    }

    pub(crate) fn parse(self) -> &'static Parse<SourceFile> {
        PARSES[self as usize].get_or_init(|| SourceFile::parse(self.sql()))
    }
}
