use crate::common::error::DocumentError;

pub trait Normalizer {
    fn normalize(&self, text: &str) -> Result<String, DocumentError>;
}

pub struct LowercaseNormalizer {}

impl Normalizer for LowercaseNormalizer {
    fn normalize(&self, text: &str) -> Result<String, DocumentError> {
        Ok(text.to_lowercase())
    }
}