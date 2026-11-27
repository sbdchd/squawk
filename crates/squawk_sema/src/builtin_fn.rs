#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BuiltinFn {
    Btrim,
    CurrentDatabase,
    CurrentSchema,
    CurrentUser,
    Extract,
    IsNormalized,
    LikeEscape,
    Ltrim,
    Normalize,
    Now,
    Overlaps,
    Overlay,
    PgCollationFor,
    Position,
    Rtrim,
    SessionUser,
    SimilarToEscape,
    Substring,
    SystemUser,
    Timezone,
    Unnest,
    Xmlexists,
}

impl BuiltinFn {
    pub(crate) fn name(self) -> &'static str {
        match self {
            BuiltinFn::Btrim => "btrim",
            BuiltinFn::CurrentDatabase => "current_database",
            BuiltinFn::CurrentSchema => "current_schema",
            BuiltinFn::CurrentUser => "current_user",
            BuiltinFn::Extract => "extract",
            BuiltinFn::IsNormalized => "is_normalized",
            BuiltinFn::LikeEscape => "like_escape",
            BuiltinFn::Ltrim => "ltrim",
            BuiltinFn::Normalize => "normalize",
            BuiltinFn::Now => "now",
            BuiltinFn::Overlaps => "overlaps",
            BuiltinFn::Overlay => "overlay",
            BuiltinFn::PgCollationFor => "pg_collation_for",
            BuiltinFn::Position => "position",
            BuiltinFn::Rtrim => "rtrim",
            BuiltinFn::SessionUser => "session_user",
            BuiltinFn::SimilarToEscape => "similar_to_escape",
            BuiltinFn::Substring => "substring",
            BuiltinFn::SystemUser => "system_user",
            BuiltinFn::Timezone => "timezone",
            BuiltinFn::Unnest => "unnest",
            BuiltinFn::Xmlexists => "xmlexists",
        }
    }
}
