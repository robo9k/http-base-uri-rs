//! HTTP/S base URI with optional path and no query - for API clients
//!
//! ```rust
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Look Ma'; scheme and authority and path, but no query!
//! let base_uri = "https://api.example.com/rest/v2".parse::<http_base_uri::Uri>()?;
//!
//! assert_eq!(base_uri.scheme(), &http::uri::Scheme::HTTPS);
//! assert_eq!(base_uri.authority(), "api.example.com");
//! assert_eq!(base_uri.path(), "/rest/v2");
//!
//! // http_base_uri::Uri is a subset of http::Uri
//! let uri: http::Uri = base_uri.into();
//!
//! // also works with http scheme and without path
//! let base_uri = "http://example.com".parse::<http_base_uri::Uri>()?;
//!
//! assert_eq!(base_uri.scheme(), "http");
//! assert_eq!(base_uri.authority(), "example.com");
//! assert_eq!(base_uri.path(), "/"); // this is a quirk of http::Uri
//!
//! // invalid; has query
//! let not_a_base_uri = "https://api.example.com/rest/v2?param=value";
//! assert!(not_a_base_uri.parse::<http_base_uri::Uri>().is_err());
//!
//! // invalid; missing scheme
//! let not_a_base_uri_either = "api.example.com";
//! assert!(not_a_base_uri_either.parse::<http_base_uri::Uri>().is_err());
//!
//! // invalid; wrong scheme - come on now, it's in the name!
//! let seriously_not_a_base_uri = "ftp://api.example.com";
//! assert!(seriously_not_a_base_uri.parse::<http_base_uri::Uri>().is_err());
//!
//! struct MyHttpApiClient {
//!     // typesafe, correct by construction 😌
//!     base_uri: http_base_uri::Uri,
//! }
//! # Ok(())
//! # }
//! ```
// TODO: serde_core

#![deny(unsafe_code)]
#![cfg_attr(not(any(test)), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

use core::str::FromStr;

/// [`http::uri::Scheme`] newtype that is either HTTP or HTTPS
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Scheme(http::uri::Scheme);

impl Scheme {
    pub const HTTP: Self = Self(http::uri::Scheme::HTTP);
    pub const HTTPS: Self = Self(http::uri::Scheme::HTTPS);

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl core::fmt::Display for Scheme {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<http::uri::Scheme> for Scheme {
    fn as_ref(&self) -> &http::uri::Scheme {
        &self.0
    }
}

impl AsRef<str> for Scheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl From<Scheme> for http::uri::Scheme {
    fn from(value: Scheme) -> Self {
        value.0
    }
}

impl PartialEq<http::uri::Scheme> for Scheme {
    fn eq(&self, other: &http::uri::Scheme) -> bool {
        self.0 == *other
    }
}

impl PartialEq<str> for Scheme {
    fn eq(&self, other: &str) -> bool {
        self.0 == *other
    }
}

/// [`Error`](core::error::Error) for [`Scheme`]
#[derive(Debug)]
pub struct InvalidSchemeError(InvalidSchemeKind);

#[derive(Debug)]
enum InvalidSchemeKind {
    ParseScheme { source: http::uri::InvalidUri },
    NotHttp { scheme: http::uri::Scheme },
}

impl core::fmt::Display for InvalidSchemeError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self(InvalidSchemeKind::ParseScheme { .. }) => {
                write!(f, "could not parse scheme")
            }
            Self(InvalidSchemeKind::NotHttp { scheme }) => {
                write!(f, "scheme not http/s: {scheme}")
            }
        }
    }
}

impl core::error::Error for InvalidSchemeError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self(InvalidSchemeKind::ParseScheme { source }) => Some(source),
            Self(InvalidSchemeKind::NotHttp { .. }) => None,
        }
    }
}

impl TryFrom<http::uri::Scheme> for Scheme {
    type Error = InvalidSchemeError;

    fn try_from(value: http::uri::Scheme) -> Result<Self, Self::Error> {
        if value == http::uri::Scheme::HTTP || value == http::uri::Scheme::HTTPS {
            Ok(Self(value))
        } else {
            Err(InvalidSchemeError(InvalidSchemeKind::NotHttp {
                scheme: value,
            }))
        }
    }
}

impl FromStr for Scheme {
    type Err = InvalidSchemeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let scheme = http::uri::Scheme::from_str(s)
            .map_err(|e| InvalidSchemeError(InvalidSchemeKind::ParseScheme { source: e }))?;

        scheme.try_into()
    }
}

impl<'a> TryFrom<&'a [u8]> for Scheme {
    type Error = InvalidSchemeError;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        let scheme = http::uri::Scheme::try_from(value)
            .map_err(|e| InvalidSchemeError(InvalidSchemeKind::ParseScheme { source: e }))?;

        scheme.try_into()
    }
}

impl<'a> TryFrom<&'a str> for Scheme {
    type Error = InvalidSchemeError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let scheme = http::uri::Scheme::try_from(value)
            .map_err(|e| InvalidSchemeError(InvalidSchemeKind::ParseScheme { source: e }))?;

        scheme.try_into()
    }
}

// TODO: join PathAndQuery ?
// TODO: pub const fn from_static(src: &'static str)
// TODO: pub fn from_maybe_shared<T>(src: T)
/// [`http::uri::PathAndQuery`] newtype without query
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd)]
pub struct Path(http::uri::PathAndQuery);

impl Path {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl core::fmt::Display for Path {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<http::uri::PathAndQuery> for Path {
    fn as_ref(&self) -> &http::uri::PathAndQuery {
        &self.0
    }
}

impl From<Path> for http::uri::PathAndQuery {
    fn from(value: Path) -> Self {
        value.0
    }
}

impl PartialEq<str> for Path {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl PartialEq<http::uri::PathAndQuery> for Path {
    fn eq(&self, other: &http::uri::PathAndQuery) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<str> for Path {
    fn partial_cmp(&self, other: &str) -> Option<core::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl PartialOrd<http::uri::PathAndQuery> for Path {
    fn partial_cmp(&self, other: &http::uri::PathAndQuery) -> Option<core::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

/// [`Error`](core::error::Error) for [`Path`]
#[derive(Debug)]
pub struct InvalidPathError(InvalidPathKind);

#[derive(Debug)]
enum InvalidPathKind {
    ParsePathAndQuery {
        source: http::uri::InvalidUri,
    },
    HasQuery {
        path_and_query: http::uri::PathAndQuery,
    },
}

impl core::fmt::Display for InvalidPathError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self(InvalidPathKind::ParsePathAndQuery { .. }) => {
                write!(f, "could not parse scheme")
            }
            Self(InvalidPathKind::HasQuery { path_and_query }) => {
                write!(
                    f,
                    "has query: {}",
                    path_and_query
                        .query()
                        .expect("PathAndQuery has query as validated previously")
                )
            }
        }
    }
}

impl core::error::Error for InvalidPathError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self(InvalidPathKind::ParsePathAndQuery { source }) => Some(source),
            Self(InvalidPathKind::HasQuery { .. }) => None,
        }
    }
}

impl TryFrom<http::uri::PathAndQuery> for Path {
    type Error = InvalidPathError;

    fn try_from(value: http::uri::PathAndQuery) -> Result<Self, Self::Error> {
        if value.query().is_none() {
            Ok(Self(value))
        } else {
            Err(InvalidPathError(InvalidPathKind::HasQuery {
                path_and_query: value,
            }))
        }
    }
}

impl FromStr for Path {
    type Err = InvalidPathError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let path = http::uri::PathAndQuery::from_str(s)
            .map_err(|e| InvalidPathError(InvalidPathKind::ParsePathAndQuery { source: e }))?;

        path.try_into()
    }
}

impl<'a> TryFrom<&'a [u8]> for Path {
    type Error = InvalidPathError;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        let path = http::uri::PathAndQuery::try_from(value)
            .map_err(|e| InvalidPathError(InvalidPathKind::ParsePathAndQuery { source: e }))?;

        path.try_into()
    }
}

impl<'a> TryFrom<&'a str> for Path {
    type Error = InvalidPathError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let path = http::uri::PathAndQuery::try_from(value)
            .map_err(|e| InvalidPathError(InvalidPathKind::ParsePathAndQuery { source: e }))?;

        path.try_into()
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<alloc::string::String> for Path {
    type Error = InvalidPathError;

    fn try_from(value: alloc::string::String) -> Result<Self, Self::Error> {
        let path = http::uri::PathAndQuery::try_from(value)
            .map_err(|e| InvalidPathError(InvalidPathKind::ParsePathAndQuery { source: e }))?;

        path.try_into()
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<alloc::vec::Vec<u8>> for Path {
    type Error = InvalidPathError;

    fn try_from(value: alloc::vec::Vec<u8>) -> Result<Self, Self::Error> {
        let path = http::uri::PathAndQuery::try_from(value)
            .map_err(|e| InvalidPathError(InvalidPathKind::ParsePathAndQuery { source: e }))?;

        path.try_into()
    }
}

/// [`http::uri::Parts`] for [`Uri`]
#[derive(Debug)]
#[non_exhaustive]
pub struct Parts {
    pub scheme: Scheme,

    pub authority: http::uri::Authority,

    pub path: Path,
}

impl Parts {
    pub const fn new(scheme: Scheme, authority: http::uri::Authority, path: Path) -> Self {
        Self {
            authority,
            scheme,
            path,
        }
    }
}

/// [`Error`](core::error::Error) for [`Parts`]
#[derive(Debug)]
pub struct InvalidPartsError(InvalidPartsKind);

#[derive(Debug)]
enum InvalidPartsKind {
    InvalidScheme { source: InvalidSchemeError },
    MissingScheme,
    MissingAuthority,
    InvalidPathAndQuery { source: InvalidPathError },
    MissingPathAndQuery,
}

impl core::fmt::Display for InvalidPartsError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self(InvalidPartsKind::InvalidScheme { .. }) => {
                write!(f, "invalid scheme")
            }
            Self(InvalidPartsKind::MissingScheme) => write!(f, "missing scheme"),
            Self(InvalidPartsKind::MissingAuthority) => {
                write!(f, "missing authority")
            }
            Self(InvalidPartsKind::InvalidPathAndQuery { .. }) => {
                write!(f, "invalid path and query")
            }
            Self(InvalidPartsKind::MissingPathAndQuery) => {
                write!(f, "missing path and query")
            }
        }
    }
}

impl core::error::Error for InvalidPartsError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self(InvalidPartsKind::InvalidScheme { source }) => Some(source),
            Self(InvalidPartsKind::InvalidPathAndQuery { source }) => Some(source),
            Self(
                InvalidPartsKind::MissingScheme
                | InvalidPartsKind::MissingAuthority
                | InvalidPartsKind::MissingPathAndQuery,
            ) => None,
        }
    }
}

impl TryFrom<http::uri::Parts> for Parts {
    type Error = InvalidPartsError;

    fn try_from(value: http::uri::Parts) -> Result<Self, Self::Error> {
        let scheme: Scheme = if let Some(scheme) = value.scheme {
            scheme
                .try_into()
                .map_err(|e| InvalidPartsError(InvalidPartsKind::InvalidScheme { source: e }))?
        } else {
            return Err(InvalidPartsError(InvalidPartsKind::MissingScheme));
        };

        let Some(authority) = value.authority else {
            return Err(InvalidPartsError(InvalidPartsKind::MissingAuthority));
        };

        let path: Path = if let Some(path_and_query) = value.path_and_query {
            path_and_query.try_into().map_err(|e| {
                InvalidPartsError(InvalidPartsKind::InvalidPathAndQuery { source: e })
            })?
        } else {
            return Err(InvalidPartsError(InvalidPartsKind::MissingPathAndQuery));
        };

        Ok(Self {
            scheme,
            authority,
            path,
        })
    }
}

impl TryFrom<http::uri::Uri> for Parts {
    type Error = InvalidPartsError;

    fn try_from(value: http::uri::Uri) -> Result<Self, Self::Error> {
        Self::try_from(value.into_parts())
    }
}

impl From<Parts> for http::uri::Parts {
    fn from(value: Parts) -> http::uri::Parts {
        let mut parts = http::uri::Parts::default();

        parts.scheme = Some(value.scheme.into());
        parts.authority = Some(value.authority);
        parts.path_and_query = Some(value.path.into());

        parts
    }
}

impl From<Parts> for http::uri::Uri {
    fn from(value: Parts) -> http::uri::Uri {
        let parts: http::uri::Parts = value.into();

        http::uri::Uri::try_from(parts).expect("Parts are valid http::uri::Uri")
    }
}

// TODO: join PathAndQuery ?
// TODO: AsRef Scheme, Authority, PathAndQuery ?
// TODO: pub fn from_parts(src: Parts)
// TODO: pub fn from_maybe_shared<T>(src: T)
// TODO: pub fn from_static(src: &'static str)
// TODO: pub fn into_parts(self)
/// [`http::uri::Uri`] subset with [`crate::Scheme`] and [`Path`] instead
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Uri {
    scheme: Scheme,
    authority: http::uri::Authority,
    path: Path,
}

impl Uri {
    pub const fn scheme(&self) -> &Scheme {
        &self.scheme
    }

    pub const fn authority(&self) -> &http::uri::Authority {
        &self.authority
    }

    pub const fn path(&self) -> &Path {
        &self.path
    }

    pub fn from_parts(src: Parts) -> Self {
        let Parts {
            scheme,
            authority,
            path,
        } = src;

        Self {
            scheme,
            authority,
            path,
        }
    }
}

impl core::fmt::Display for Uri {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}://", self.scheme)?;

        write!(f, "{}", self.authority)?;

        write!(f, "{}", self.path)?;

        Ok(())
    }
}

impl From<Uri> for http::uri::Parts {
    fn from(value: Uri) -> http::uri::Parts {
        let mut parts = http::uri::Parts::default();
        parts.scheme = Some(value.scheme.0);
        parts.authority = Some(value.authority);
        parts.path_and_query = Some(value.path.0);

        parts
    }
}

impl From<Uri> for http::uri::Uri {
    fn from(value: Uri) -> http::uri::Uri {
        let parts: http::uri::Parts = value.into();

        http::uri::Uri::try_from(parts).expect("Parts are valid http::uri::Uri")
    }
}

/// [`Error`](core::error::Error) for [`Uri`]
#[derive(Debug)]
pub struct InvalidUriError(InvalidUriKind);

#[derive(Debug)]
enum InvalidUriKind {
    ParseUri { source: http::uri::InvalidUri },
    InvalidParts { source: InvalidPartsError },
}

impl core::fmt::Display for InvalidUriError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self(InvalidUriKind::ParseUri { .. }) => {
                write!(f, "could not parse Uri")
            }
            Self(InvalidUriKind::InvalidParts { .. }) => {
                write!(f, "invalid parts")
            }
        }
    }
}

impl core::error::Error for InvalidUriError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self(InvalidUriKind::ParseUri { source }) => Some(source),
            Self(InvalidUriKind::InvalidParts { source }) => Some(source),
        }
    }
}

impl TryFrom<http::uri::Parts> for Uri {
    type Error = InvalidPartsError;

    fn try_from(value: http::uri::Parts) -> Result<Self, Self::Error> {
        let parts: Parts = value.try_into()?;

        Ok(Self::from(parts))
    }
}

impl From<Parts> for Uri {
    fn from(value: Parts) -> Self {
        Self::from_parts(value)
    }
}

impl TryFrom<http::uri::Uri> for Uri {
    type Error = InvalidPartsError;

    fn try_from(value: http::uri::Uri) -> Result<Self, Self::Error> {
        let parts: Parts = value.try_into()?;

        Ok(Self::from(parts))
    }
}

impl FromStr for Uri {
    type Err = InvalidUriError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let uri = http::uri::Uri::from_str(s)
            .map_err(|e| InvalidUriError(InvalidUriKind::ParseUri { source: e }))?;

        uri.try_into()
            .map_err(|e| InvalidUriError(InvalidUriKind::InvalidParts { source: e }))
    }
}

impl<'a> TryFrom<&'a [u8]> for Uri {
    type Error = InvalidUriError;

    fn try_from(value: &'a [u8]) -> Result<Self, InvalidUriError> {
        let uri = http::uri::Uri::try_from(value)
            .map_err(|e| InvalidUriError(InvalidUriKind::ParseUri { source: e }))?;

        uri.try_into()
            .map_err(|e| InvalidUriError(InvalidUriKind::InvalidParts { source: e }))
    }
}

impl<'a> TryFrom<&'a str> for Uri {
    type Error = InvalidUriError;

    fn try_from(value: &'a str) -> Result<Self, InvalidUriError> {
        let uri = http::uri::Uri::try_from(value)
            .map_err(|e| InvalidUriError(InvalidUriKind::ParseUri { source: e }))?;

        uri.try_into()
            .map_err(|e| InvalidUriError(InvalidUriKind::InvalidParts { source: e }))
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<alloc::string::String> for Uri {
    type Error = InvalidUriError;

    fn try_from(value: alloc::string::String) -> Result<Self, InvalidUriError> {
        let uri = http::uri::Uri::try_from(value)
            .map_err(|e| InvalidUriError(InvalidUriKind::ParseUri { source: e }))?;

        uri.try_into()
            .map_err(|e| InvalidUriError(InvalidUriKind::InvalidParts { source: e }))
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<alloc::vec::Vec<u8>> for Uri {
    type Error = InvalidUriError;

    fn try_from(value: alloc::vec::Vec<u8>) -> Result<Self, InvalidUriError> {
        let uri = http::uri::Uri::try_from(value)
            .map_err(|e| InvalidUriError(InvalidUriKind::ParseUri { source: e }))?;

        uri.try_into()
            .map_err(|e| InvalidUriError(InvalidUriKind::InvalidParts { source: e }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scheme_tryfrom() -> Result<(), Box<dyn std::error::Error>> {
        let invalid_scheme = http::uri::Scheme::from_str("ftp")?;

        let scheme = Scheme::try_from(invalid_scheme);
        assert_eq!(scheme.unwrap_err().to_string(), "scheme not http/s: ftp");

        Ok(())
    }

    #[test]
    fn scheme_into() -> Result<(), Box<dyn std::error::Error>> {
        let scheme = Scheme::HTTPS;
        let scheme: http::uri::Scheme = scheme.into();
        assert_eq!(scheme, http::uri::Scheme::HTTPS);

        Ok(())
    }

    #[test]
    fn scheme_eq() -> Result<(), Box<dyn std::error::Error>> {
        let scheme = Scheme::HTTPS;
        assert_eq!(scheme, *"https");

        Ok(())
    }

    #[test]
    fn cheme_fromstr() -> Result<(), Box<dyn std::error::Error>> {
        let scheme = Scheme::from_str("https")?;
        assert_eq!(scheme, Scheme::HTTPS);

        Ok(())
    }

    #[test]
    fn path_tryfrom() -> Result<(), Box<dyn std::error::Error>> {
        let invalid_pathandquery = http::uri::PathAndQuery::from_str("/resource?param=value")?;

        let path = Path::try_from(invalid_pathandquery);
        assert_eq!(path.unwrap_err().to_string(), "has query: param=value");

        Ok(())
    }

    #[test]
    fn path_into() -> Result<(), Box<dyn std::error::Error>> {
        let path = Path::from_str("/resource")?;
        let pathandquery: http::uri::PathAndQuery = path.into();
        assert_eq!(
            pathandquery,
            http::uri::PathAndQuery::from_str("/resource")?
        );

        Ok(())
    }

    #[test]
    fn uri_tryfrom() -> Result<(), Box<dyn std::error::Error>> {
        let invalid_uri = "/resource?param=value";

        let uri = Uri::try_from(invalid_uri);
        assert_eq!(uri.unwrap_err().to_string(), "invalid parts");

        let invalid_uri = "api.example.com";

        let uri = Uri::try_from(invalid_uri);
        assert_eq!(uri.unwrap_err().to_string(), "invalid parts");

        Ok(())
    }

    #[test]
    fn uri_display() -> Result<(), Box<dyn std::error::Error>> {
        let uri = Uri::try_from("https://api.example.com/rest/v2#frag1")?;

        assert_eq!(format!("{uri}"), "https://api.example.com/rest/v2");

        Ok(())
    }
}
