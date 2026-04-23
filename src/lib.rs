//! HTTP/S base URI for API clients
//!
//! ```rust
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use http_base_uri::HttpBaseUri;
//!
//! // Look Ma'; scheme and authority and path, but no query nor fragment!
//! let base_uri = "https://api.example.com/rest/v2".parse::<HttpBaseUri>()?;
//!
//! assert_eq!(base_uri.scheme(), &http::uri::Scheme::HTTPS);
//! assert_eq!(base_uri.authority(), "api.example.com");
//! assert_eq!(base_uri.path(), "/rest/v2");
//!
//! // BaseURI is a subset of Uri
//! let uri: http::Uri = base_uri.into();
//! # Ok(())
//! # }
//! ```
// TODO: serde_core
use core::str::FromStr;

// TODO: pub fn as_str(&self)
// TODO: impl Clone, Display, FromStr, Hash, PartialEq, TryFrom<&'a [u8]>, TryFrom<&'a str>
/// [`http::uri::Scheme`] newtype that is either HTTP or HTTPS
#[derive(Debug)]
pub struct HttpScheme(http::uri::Scheme);

impl HttpScheme {
    pub const HTTP: Self = Self(http::uri::Scheme::HTTP);
    pub const HTTPS: Self = Self(http::uri::Scheme::HTTPS);
}

impl AsRef<http::uri::Scheme> for HttpScheme {
    fn as_ref(&self) -> &http::uri::Scheme {
        &self.0
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

// TODO: impl Error, Display
#[derive(Debug)]
pub struct InvalidSchemeError(());

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

// TODO: join PathAndQuery ?
// TODO: pub const fn from_static(src: &'static str)
// TODO: pub fn from_maybe_shared<T>(src: T)
// TODO: pub fn as_str(&self)
// TODO: impl Clone, Display, FromStr, Hash, PartialEq, PartialOrd, Eq, TryFrom<&'a [u8]>, TryFrom<&'a str>, TryFrom<String>, TryFrom<Vec<u8>>
/// [`http::uri::PathAndQuery`] newtype without query
#[derive(Debug)]
pub struct Path(http::uri::PathAndQuery);

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

// TODO: impl Error, Display
#[derive(Debug)]
pub struct InvalidPathError(());

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

// TODO: join PathAndQuery ?
// TODO: AsRef Scheme, Authority, PathAndQuery ?
// TODO: pub fn from_parts(src: Parts)
// TODO: pub fn from_maybe_shared<T>(src: T)
// TODO: pub fn from_static(src: &'static str)
// TODO: pub fn into_parts(self)
// TODO: impl Clone, Display, Hash, PartialEq, TryFrom<&'a [u8]>, TryFrom<&'a str>, TryFrom<(parts)>, TryFrom<String>, TryFrom<Vec<u8>>, Eq
/// [`http::uri::Uri`] with [`HttpScheme`] and [`Path`] instead
#[derive(Debug)]
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
    fn path_tryfrom() -> Result<(), Box<dyn std::error::Error>> {
        let invalid_pathandquery = http::uri::PathAndQuery::from_str("/resource?param=value")?;

        let path = Path::try_from(invalid_pathandquery);
        assert!(path.is_err());

        Ok(())
    }
}
