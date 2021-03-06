#[cfg(test)]
mod unit_tests {
    #[cfg(any(feature = "json", feature = "yaml", feature = "cbor"))]
    use crate::error::Error;
    #[cfg(any(feature = "json", feature = "yaml", feature = "cbor"))]
    use try_encoding_from::Error as EncodingError;

    #[test]
    #[cfg(all(feature = "serde", feature = "cbor"))]
    fn test_from_cbor_encoding_error() {
        assert_eq!(
            Error::EncodingError(EncodingError::CborError),
            EncodingError::CborError.into()
        );
    }

    #[test]
    #[cfg(all(feature = "serde", feature = "json"))]
    fn test_from_json_encoding_error() {
        assert_eq!(
            Error::EncodingError(EncodingError::JsonError),
            EncodingError::JsonError.into()
        );
    }

    #[test]
    #[cfg(all(feature = "serde", feature = "yaml"))]
    fn test_from_yaml_encoding_error() {
        assert_eq!(
            Error::EncodingError(EncodingError::YamlError),
            EncodingError::YamlError.into()
        );
    }
}