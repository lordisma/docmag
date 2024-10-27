use crate::common::token::Token;
use crate::index::normalize::Normalizer;
use crate::common::error::DocumentError;
use std::ops::ControlFlow;

#[derive(Debug)]
pub struct DocumentContent {
    pub contents: Vec<Token>
}

pub struct DocumentBuilder {
    pub normalizer: Vec<Box<dyn Normalizer>>,
    pub text: String
}

impl DocumentBuilder {
    fn new() -> Self {
        DocumentBuilder {
            normalizer: Vec::new(),
            text: String::new()
        }
    }

    /// Return the content of a document as a collection of tokens.
    fn build(self) -> Result<DocumentContent, DocumentError> {
        let norm_text = self.normalizer
            .into_iter()
            .try_fold(self.text, |text, norm| {
                let result = norm.normalize(&text)
                    .map_err(|e| DocumentError::NormalizationError(e.to_string()));

                if result.is_err() {
                    return ControlFlow::Break(result.err());
                } else {
                    return ControlFlow::Continue(result.unwrap());
                }
            });

        match norm_text {
            ControlFlow::Break(Some(err))=> return Err(err),
            ControlFlow::Break(None) => return Err(DocumentError::NormalizationError("Unknown error".to_string())),
            ControlFlow::Continue(norm_text) => {
                let tokens = norm_text.split_whitespace();
                Ok(DocumentContent {
                    contents: tokens.map(|t| t.to_string()).collect()
                })
            }
        }
    }

    /// Set normalization step for the raw document
    fn nomalization(mut self, normalizer: Box<dyn Normalizer>) -> Self {
        self.normalizer.push(normalizer);
        self
    }

    fn text(mut self, text: &str) -> Self {
        self.text = text.to_string();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::index::normalize::{LowercaseNormalizer, Normalizer};

    pub struct FailingNormalizer {}

    impl Normalizer for FailingNormalizer {
        fn normalize(&self, _: &str) -> Result<String, DocumentError> {
            Err(DocumentError::NormalizationError("Failing Normalizer".to_string()))
        }
    }

    #[test]
    fn test_document_builder() {
        let doc = DocumentBuilder::new()
            .nomalization(Box::new(LowercaseNormalizer {}))
            .text("HELLO WORLD")
            .build();

        assert!(doc.is_ok());
        let doc = doc.unwrap();

        assert_eq!(doc.contents.len(), 2);
        assert_eq!(doc.contents[0], "hello");
        assert_eq!(doc.contents[1], "world");
    }

    #[test]
    fn test_document_builder_with_failing_normalizer() {
        let doc = DocumentBuilder::new()
            .nomalization(Box::new(FailingNormalizer {}))
            .text("HELLO WORLD")
            .build();

        assert!(doc.is_err());
        let doc = doc.unwrap_err();

        // assert_eq!(doc.to_string(), "DocumentError: NormalizationError(\"Failing Normalizer\")");
    }
}