// SPDX-License-Identifier: Apache-2.0
#![no_std]
//! SigmaOS-documentation - Documentation generator model.
//!
//! Models documentation features: doc generation,
//! API docs. no_std compatible.

/// Document
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Document {
    pub id: u32,
}

/// Documentation state with fixed-capacity storage
pub struct Documentation {
    pub docs: [Option<Document>; 4],
}

impl Documentation {
    pub fn new() -> Self {
        Self { docs: [None; 4] }
    }

    pub fn add_doc(&mut self, doc: Document) -> Result<(), DocumentationError> {
        for i in 0..self.docs.len() {
            if self.docs[i].is_none() {
                self.docs[i] = Some(doc);
                return Ok(());
            }
        }
        Err(DocumentationError::Full)
    }

    pub fn count(&self) -> usize {
        self.docs.iter().filter(|d| d.is_some()).count()
    }
}

impl Default for Documentation {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum DocumentationError {
    Full,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let d = Documentation::new();
        assert_eq!(d.count(), 0);
    }

    #[test]
    fn test_add_doc() {
        let mut d = Documentation::new();
        let doc = Document { id: 1 };
        assert!(d.add_doc(doc).is_ok());
        assert_eq!(d.count(), 1);
    }
}
