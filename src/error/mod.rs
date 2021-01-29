#[cfg(feature = "fmt")]
use core::fmt::{Display, Formatter, Result};

#[cfg(any(feature = "json", feature = "yaml", feature = "cbor"))]
use try_encoding_from::Error as EncodingError;

mod test;

#[cfg(feature = "fmt")]
static VERTEX_DOES_NOT_EXIST_ERROR: &str = "BForest Error: Vertex does not exist";
#[cfg(feature = "fmt")]
static EDGE_DOES_NOT_EXIST_ERROR: &str = "BForest Error: Edge does not exist";
#[cfg(feature = "fmt")]
static EDGE_EXISTS_ERROR: &str = "BForest Error: Edge exists";
#[cfg(feature = "fmt")]
static VERTEX_EXISTS_ERROR: &str = "BForest Error: Vertex exists";
#[cfg(feature = "fmt")]
static IMPROPER_DIMENSION_ERROR: &str = "BForest Error: Improper dimension";

/// Errors which may occur during normal usage of the library.
#[derive(PartialEq, Debug)]
pub enum Error {
    VertexDoesNotExist,
    EdgeDoesNotExist,
    VertexExists,
    EdgeExists,
    ImproperDimension,
    #[cfg(any(feature = "cbor", feature = "json", feature = "yaml"))]
    EncodingError(try_encoding_from::Error),
}

#[cfg(feature = "fmt")]
impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Error::VertexDoesNotExist => write!(f, "{}", VERTEX_DOES_NOT_EXIST_ERROR),
            Error::EdgeDoesNotExist => write!(f, "{}", EDGE_DOES_NOT_EXIST_ERROR),
            Error::VertexExists => write!(f, "{}", VERTEX_EXISTS_ERROR),
            Error::EdgeExists => write!(f, "{}", EDGE_EXISTS_ERROR),
            Error::ImproperDimension => write!(f, "{}", IMPROPER_DIMENSION_ERROR),
            #[cfg(any(feature = "cbor", feature = "json", feature = "yaml"))]
            Error::EncodingError(err) => write!(f, "{}", err),
        }
    }
}

#[cfg(any(feature = "cbor", feature = "json", feature = "yaml"))]
impl From<try_encoding_from::Error> for Error {
    fn from(e: try_encoding_from::Error) -> Error {
        Error::EncodingError(e)
    }
}

#[cfg(feature = "cbor")]
impl From<try_encoding_from::serde_cbor::Error> for Error {
    fn from(e: try_encoding_from::serde_cbor::Error) -> Error {
        Error::EncodingError(EncodingError::from(e))
    }
}

#[cfg(feature = "json")]
impl From<try_encoding_from::serde_json::Error> for Error {
    fn from(e: try_encoding_from::serde_json::Error) -> Error {
        Error::EncodingError(EncodingError::from(e))
    }
}

#[cfg(feature = "yaml")]
impl From<try_encoding_from::serde_yaml::Error> for Error {
    fn from(e: try_encoding_from::serde_yaml::Error) -> Error {
        Error::EncodingError(EncodingError::from(e))
    }
}
