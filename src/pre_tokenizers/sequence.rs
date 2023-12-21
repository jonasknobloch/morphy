use serde::{Deserialize, Serialize};
use tokenizers::impl_serde_type;
use tokenizers::tokenizer::{PreTokenizedString, PreTokenizer, Result};
use tokenizers::utils::macro_rules_attribute;

use crate::pre_tokenizers::PreTokenizerWrapper;

#[derive(Clone, Debug, PartialEq)]
#[macro_rules_attribute(impl_serde_type!)]
pub struct Sequence {
    pretokenizers: Vec<PreTokenizerWrapper>,
}

impl Sequence {
    pub fn new(pretokenizers: Vec<PreTokenizerWrapper>) -> Self {
        Self { pretokenizers }
    }

    pub fn get_pre_tokenizers(&self) -> &[PreTokenizerWrapper] {
        &self.pretokenizers
    }

    pub fn get_pre_tokenizers_mut(&mut self) -> &mut [PreTokenizerWrapper] {
        &mut self.pretokenizers
    }
}

impl PreTokenizer for Sequence {
    fn pre_tokenize(&self, pretokenized: &mut PreTokenizedString) -> Result<()> {
        for pretokenizer in &self.pretokenizers {
            pretokenizer.pre_tokenize(pretokenized)?;
        }

        Ok(())
    }
}
