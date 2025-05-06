use std::num::ParseIntError;
use std::str::FromStr;

use serde::{de::Error, Deserialize, Deserializer};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Version {
    major: i32,
    minor: Option<i32>,
    patch: Option<i32>,
}

impl Version {
    #[must_use]
    pub(crate) fn new(
        major: i32,
        minor: impl Into<Option<i32>>,
        patch: impl Into<Option<i32>>,
    ) -> Self {
        Self {
            major,
            minor: minor.into(),
            patch: patch.into(),
        }
    }
}

// Allow us to deserialize our version from a string in .squawk.toml.
// from https://stackoverflow.com/a/46755370/
impl<'de> Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        Version::from_str(s).map_err(D::Error::custom)
    }
}

#[derive(Debug, PartialEq)]
pub struct InvalidNumber {
    pub version: String,
    pub e: ParseIntError,
}

#[derive(Debug, PartialEq)]
pub struct EmptyVersion {
    pub version: String,
}

#[derive(Debug, PartialEq)]
pub enum ParseVersionError {
    EmptyVersion(EmptyVersion),
    InvalidNumber(InvalidNumber),
}

fn parse_int(s: &str) -> Result<i32, ParseVersionError> {
    Ok(s.parse().map_err(|e| {
        ParseVersionError::InvalidNumber(InvalidNumber {
            version: s.to_string(),
            e,
        })
    }))?
}

impl std::fmt::Display for ParseVersionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Self::EmptyVersion(ref err) => {
                write!(f, "Empty version number provided: {:?}", err.version)
            }
            Self::InvalidNumber(ref err) => {
                write!(
                    f,
                    "Invalid number in version: {:?}. Parse error: {}",
                    err.version, err.e
                )
            }
        }
    }
}

impl FromStr for Version {
    type Err = ParseVersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let version_pieces: Vec<&str> = s.split('.').collect();

        if version_pieces.is_empty() {
            return Err(ParseVersionError::EmptyVersion(EmptyVersion {
                version: s.to_string(),
            }));
        }
        let major = parse_int(version_pieces[0])?;

        let minor: Option<i32> = if version_pieces.len() > 1 {
            Some(parse_int(version_pieces[1])?)
        } else {
            None
        };
        let patch: Option<i32> = if version_pieces.len() > 2 {
            Some(parse_int(version_pieces[2])?)
        } else {
            None
        };

        Ok(Version {
            major,
            minor,
            patch,
        })
    }
}

#[cfg(test)]
mod test_pg_version {
    #![allow(clippy::neg_cmp_op_on_partial_ord)]
    use insta::assert_debug_snapshot;

    use super::*;
    #[test]
    fn eq() {
        assert_eq!(Version::new(10, None, None), Version::new(10, None, None));
    }
    #[test]
    fn gt() {
        assert!(Version::new(10, Some(1), None) > Version::new(10, None, None));
        assert!(Version::new(10, None, Some(1)) > Version::new(10, None, None));
        assert!(Version::new(10, None, Some(1)) > Version::new(9, None, None));

        assert!(!(Version::new(10, None, None) > Version::new(10, None, None)));
    }
    #[test]
    fn parse() {
        assert_eq!(
            Version::from_str("10.1"),
            Ok(Version::new(10, Some(1), None))
        );
        assert_eq!(Version::from_str("10"), Ok(Version::new(10, None, None)));
        assert_eq!(
            Version::from_str("10.2.1"),
            Ok(Version::new(10, Some(2), Some(1)))
        );
        assert_debug_snapshot!(Version::from_str("test").unwrap_err());
    }
}
