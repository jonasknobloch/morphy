use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use tokenizers::tokenizer::{PreTokenizedString, PreTokenizer, Result};

use crate::pre_tokenizers::PreTokenizerWrapper;

#[derive(Clone, Debug, PartialEq)]
pub struct Sequence {
    pretokenizers: Vec<PreTokenizerWrapper>,
}

impl Serialize for Sequence {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(2))?;

        map.serialize_entry("type", "Sequence")?;

        let mut valid_pretokenizers = Vec::new();

        for tokenizer in &self.pretokenizers {
            match tokenizer {
                PreTokenizerWrapper::External(_) => {
                    continue;
                }
                PreTokenizerWrapper::PreByteLevel(_) => {
                    continue;
                }
                _ => {
                    valid_pretokenizers.push(tokenizer);
                }
            }
        }

        map.serialize_entry("pretokenizers", &valid_pretokenizers)?;

        map.end()
    }
}

impl<'de> Deserialize<'de> for Sequence {
    fn deserialize<D>(_deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Err(serde::de::Error::custom("deserialization not implemented"))
    }
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
