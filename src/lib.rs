//! HTTP/S base URI for API clients
//!
//! ```rust
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use http_base_uri::HttpBaseUri;
//!
//! // Look Ma'; scheme and authority and path, but no query!
//! let base_uri = "https://api.example.com/rest/v2".parse::<HttpBaseUri>()?;
//!
//! assert_eq!(base_uri.scheme(), &http::uri::Scheme::HTTPS);
//! assert_eq!(base_uri.authority(), "api.example.com");
//! assert_eq!(base_uri.path(), "/rest/v2");
//!
//! // HttpBaseUri is a subset of Uri
//! let uri: http::Uri = base_uri.into();
//!
//! // also works with http and without path
//! let base_uri = "http://example.com".parse::<HttpBaseUri>()?;
//!
//! assert_eq!(base_uri.scheme(), "http");
//! assert_eq!(base_uri.authority(), "example.com");
//! assert_eq!(base_uri.path(), "/"); // this is a quirk of http::Uri
//!
//! // invalid; has query
//! let not_a_base_uri = "https://api.example.com/rest/v2?param=value";
//! assert!(not_a_base_uri.parse::<HttpBaseUri>().is_err());
//!
//! // invalid; missing scheme
//! let not_a_base_uri_either = "api.example.com";
//! assert!(not_a_base_uri_either.parse::<HttpBaseUri>().is_err());
//!
//! // invalid; wrong scheme - come on now, it's in the name!
//! let seriously_not_a_base_uri = "ftp://api.example.com";
//! assert!(seriously_not_a_base_uri.parse::<HttpBaseUri>().is_err());
//!
//! struct MyHttpApiClient {
//!     // typesafe, correct by construction 😌
//!     base_uri: HttpBaseUri,
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
pub struct HttpScheme(http::uri::Scheme);

impl HttpScheme {
    pub const HTTP: Self = Self(http::uri::Scheme::HTTP);
    pub const HTTPS: Self = Self(http::uri::Scheme::HTTPS);

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl core::fmt::Display for HttpScheme {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<http::uri::Scheme> for HttpScheme {
    fn as_ref(&self) -> &http::uri::Scheme {
        &self.0
    }
}

impl AsRef<str> for HttpScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl From<HttpScheme> for http::uri::Scheme {
    fn from(value: HttpScheme) -> Self {
        value.0
    }
}

impl PartialEq<http::uri::Scheme> for HttpScheme {
    fn eq(&self, other: &http::uri::Scheme) -> bool {
        self.0 == *other
    }
}

impl PartialEq<str> for HttpScheme {
    fn eq(&self, other: &str) -> bool {
        self.0 == *other
    }
}

#[derive(Debug)]
pub struct InvalidSchemeError(());

impl core::fmt::Display for InvalidSchemeError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "invalid scheme")
    }
}

impl core::error::Error for InvalidSchemeError {}

impl TryFrom<http::uri::Scheme> for HttpScheme {
    type Error = InvalidSchemeError;

    fn try_from(value: http::uri::Scheme) -> Result<Self, InvalidSchemeError> {
        if value == http::uri::Scheme::HTTP || value == http::uri::Scheme::HTTPS {
            Ok(Self(value))
        } else {
            Err(InvalidSchemeError(()))
        }
    }
}

impl FromStr for HttpScheme {
    type Err = InvalidSchemeError;

    fn from_str(s: &str) -> Result<Self, InvalidSchemeError> {
        let scheme = http::uri::Scheme::from_str(s).map_err(|_e| InvalidSchemeError(()))?;

        scheme.try_into()
    }
}

impl<'a> TryFrom<&'a [u8]> for HttpScheme {
    type Error = InvalidSchemeError;

    fn try_from(value: &'a [u8]) -> Result<Self, InvalidSchemeError> {
        let scheme = http::uri::Scheme::try_from(value).map_err(|_e| InvalidSchemeError(()))?;

        scheme.try_into()
    }
}

impl<'a> TryFrom<&'a str> for HttpScheme {
    type Error = InvalidSchemeError;

    fn try_from(value: &'a str) -> Result<Self, InvalidSchemeError> {
        let scheme = http::uri::Scheme::try_from(value).map_err(|_e| InvalidSchemeError(()))?;

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

#[derive(Debug)]
pub struct InvalidPathError(());

impl core::fmt::Display for InvalidPathError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "invalid scheme")
    }
}

impl core::error::Error for InvalidPathError {}

impl TryFrom<http::uri::PathAndQuery> for Path {
    type Error = InvalidPathError;

    fn try_from(value: http::uri::PathAndQuery) -> Result<Self, InvalidPathError> {
        if value.query().is_none() {
            Ok(Self(value))
        } else {
            Err(InvalidPathError(()))
        }
    }
}

impl FromStr for Path {
    type Err = InvalidPathError;

    fn from_str(s: &str) -> Result<Self, InvalidPathError> {
        let path = http::uri::PathAndQuery::from_str(s).map_err(|_e| InvalidPathError(()))?;

        path.try_into()
    }
}

impl<'a> TryFrom<&'a [u8]> for Path {
    type Error = InvalidPathError;

    fn try_from(value: &'a [u8]) -> Result<Self, InvalidPathError> {
        let path = http::uri::PathAndQuery::try_from(value).map_err(|_e| InvalidPathError(()))?;

        path.try_into()
    }
}

impl<'a> TryFrom<&'a str> for Path {
    type Error = InvalidPathError;

    fn try_from(value: &'a str) -> Result<Self, InvalidPathError> {
        let path = http::uri::PathAndQuery::try_from(value).map_err(|_e| InvalidPathError(()))?;

        path.try_into()
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<alloc::string::String> for Path {
    type Error = InvalidPathError;

    fn try_from(value: alloc::string::String) -> Result<Self, InvalidPathError> {
        let path = http::uri::PathAndQuery::try_from(value).map_err(|_e| InvalidPathError(()))?;

        path.try_into()
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<alloc::vec::Vec<u8>> for Path {
    type Error = InvalidPathError;

    fn try_from(value: alloc::vec::Vec<u8>) -> Result<Self, InvalidPathError> {
        let path = http::uri::PathAndQuery::try_from(value).map_err(|_e| InvalidPathError(()))?;

        path.try_into()
    }
}

// TODO: join PathAndQuery ?
// TODO: AsRef Scheme, Authority, PathAndQuery ?
// TODO: pub fn from_parts(src: Parts)
// TODO: pub fn from_maybe_shared<T>(src: T)
// TODO: pub fn from_static(src: &'static str)
// TODO: pub fn into_parts(self)
/// [`http::uri::Uri`] with [`HttpScheme`] and [`Path`] instead
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct HttpBaseUri {
    scheme: HttpScheme,
    authority: http::uri::Authority,
    path: Path,
}

impl HttpBaseUri {
    pub const fn scheme(&self) -> &HttpScheme {
        &self.scheme
    }

    pub const fn authority(&self) -> &http::uri::Authority {
        &self.authority
    }

    pub const fn path(&self) -> &Path {
        &self.path
    }
}

impl core::fmt::Display for HttpBaseUri {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}://", self.scheme)?;

        write!(f, "{}", self.authority)?;

        write!(f, "{}", self.path)?;

        Ok(())
    }
}

impl From<HttpBaseUri> for http::uri::Uri {
    fn from(value: HttpBaseUri) -> http::uri::Uri {
        let mut parts = http::uri::Parts::default();
        parts.scheme = Some(value.scheme.0);
        parts.authority = Some(value.authority);
        parts.path_and_query = Some(value.path.0);

        http::uri::Uri::try_from(parts).expect("HttpBaseUri parts are valid Parts")
    }
}

#[derive(Debug)]
pub struct InvalidHttpUriError(());

impl core::fmt::Display for InvalidHttpUriError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "invalid HTTP URI")
    }
}

impl core::error::Error for InvalidHttpUriError {}

impl TryFrom<http::uri::Parts> for HttpBaseUri {
    type Error = InvalidHttpUriError;

    fn try_from(value: http::uri::Parts) -> Result<Self, InvalidHttpUriError> {
        let scheme: HttpScheme = if let Some(scheme) = value.scheme {
            scheme.try_into().map_err(|_e| InvalidHttpUriError(()))?
        } else {
            return Err(InvalidHttpUriError(()));
        };

        let authority = if let Some(authority) = value.authority {
            authority
        } else {
            return Err(InvalidHttpUriError(()));
        };

        let path: Path = if let Some(path_and_query) = value.path_and_query {
            path_and_query
                .try_into()
                .map_err(|_e| InvalidHttpUriError(()))?
        } else {
            return Err(InvalidHttpUriError(()));
        };

        Ok(Self {
            scheme,
            authority,
            path,
        })
    }
}

impl TryFrom<http::uri::Uri> for HttpBaseUri {
    type Error = InvalidHttpUriError;

    fn try_from(value: http::uri::Uri) -> Result<Self, InvalidHttpUriError> {
        let parts = value.into_parts();

        parts.try_into()
    }
}

impl FromStr for HttpBaseUri {
    type Err = InvalidHttpUriError;

    fn from_str(s: &str) -> Result<Self, InvalidHttpUriError> {
        let uri = http::uri::Uri::from_str(s).map_err(|_e| InvalidHttpUriError(()))?;

        uri.try_into()
    }
}

impl<'a> TryFrom<&'a [u8]> for HttpBaseUri {
    type Error = InvalidHttpUriError;

    fn try_from(value: &'a [u8]) -> Result<Self, InvalidHttpUriError> {
        let uri = http::uri::Uri::try_from(value).map_err(|_e| InvalidHttpUriError(()))?;

        uri.try_into()
    }
}

impl<'a> TryFrom<&'a str> for HttpBaseUri {
    type Error = InvalidHttpUriError;

    fn try_from(value: &'a str) -> Result<Self, InvalidHttpUriError> {
        let uri = http::uri::Uri::try_from(value).map_err(|_e| InvalidHttpUriError(()))?;

        uri.try_into()
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<alloc::string::String> for HttpBaseUri {
    type Error = InvalidHttpUriError;

    fn try_from(value: alloc::string::String) -> Result<Self, InvalidHttpUriError> {
        let uri = http::uri::Uri::try_from(value).map_err(|_e| InvalidHttpUriError(()))?;

        uri.try_into()
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<alloc::vec::Vec<u8>> for HttpBaseUri {
    type Error = InvalidHttpUriError;

    fn try_from(value: alloc::vec::Vec<u8>) -> Result<Self, InvalidHttpUriError> {
        let uri = http::uri::Uri::try_from(value).map_err(|_e| InvalidHttpUriError(()))?;

        uri.try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn httpscheme_tryfrom() -> Result<(), Box<dyn std::error::Error>> {
        let invalid_scheme = http::uri::Scheme::from_str("ftp")?;

        let http_scheme = HttpScheme::try_from(invalid_scheme);
        assert!(http_scheme.is_err());

        Ok(())
    }

    #[test]
    fn httpscheme_into() -> Result<(), Box<dyn std::error::Error>> {
        let http_scheme = HttpScheme::HTTPS;
        let scheme: http::uri::Scheme = http_scheme.into();
        assert_eq!(scheme, http::uri::Scheme::HTTPS);

        Ok(())
    }

    #[test]
    fn httpscheme_eq() -> Result<(), Box<dyn std::error::Error>> {
        let http_scheme = HttpScheme::HTTPS;
        assert_eq!(http_scheme, *"https");

        Ok(())
    }

    #[test]
    fn httpscheme_fromstr() -> Result<(), Box<dyn std::error::Error>> {
        let http_scheme = HttpScheme::from_str("https")?;
        assert_eq!(http_scheme, HttpScheme::HTTPS);

        Ok(())
    }

    #[test]
    fn path_tryfrom() -> Result<(), Box<dyn std::error::Error>> {
        let invalid_pathandquery = http::uri::PathAndQuery::from_str("/resource?param=value")?;

        let path = Path::try_from(invalid_pathandquery);
        assert!(path.is_err());

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
    fn httpbaseuri_tryfrom() -> Result<(), Box<dyn std::error::Error>> {
        let invalid_httpbaseuri = "/resource?param=value";

        let httpbaseuri = HttpBaseUri::try_from(invalid_httpbaseuri);
        assert!(httpbaseuri.is_err());

        let invalid_httpbaseuri = "api.example.com";

        let httpbaseuri = HttpBaseUri::try_from(invalid_httpbaseuri);
        assert!(httpbaseuri.is_err());

        Ok(())
    }

    #[test]
    fn httpbaseuri_display() -> Result<(), Box<dyn std::error::Error>> {
        let httpbaseuri = HttpBaseUri::try_from("https://api.example.com/rest/v2#frag1")?;

        assert_eq!(format!("{httpbaseuri}"), "https://api.example.com/rest/v2");

        Ok(())
    }
}
