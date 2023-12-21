pub mod external;
pub mod sequence;

use serde::{Deserialize, Serialize};

use tokenizers::pre_tokenizers::byte_level::ByteLevel;
use tokenizers::pre_tokenizers::PreTokenizerWrapper as TokenizersPreTokenizerWrapper;
use tokenizers::{PreTokenizedString, PreTokenizer};

use crate::pre_tokenizers::external::External;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum PreTokenizerWrapper {
    External(External),
    TokenizersPreTokenizerWrapper(TokenizersPreTokenizerWrapper),
}

impl PreTokenizer for PreTokenizerWrapper {
    fn pre_tokenize(&self, normalized: &mut PreTokenizedString) -> tokenizers::Result<()> {
        match self {
            Self::External(ext) => ext.pre_tokenize(normalized),
            Self::TokenizersPreTokenizerWrapper(ptw) => ptw.pre_tokenize(normalized),
        }
    }
}

// PreTokenizerWrapper::from(External::default());
// PreTokenizerWrapper::External(External::default());
impl From<External> for PreTokenizerWrapper {
    fn from(from: External) -> Self {
        PreTokenizerWrapper::External(from)
    }
}

// PreTokenizerWrapper::from(ByteLevel::default());
// PreTokenizerWrapper::TokenizersPreTokenizerWrapper(TokenizersPreTokenizerWrapper::ByteLevel(ByteLevel::default()));
impl From<ByteLevel> for PreTokenizerWrapper {
    fn from(from: ByteLevel) -> Self {
        PreTokenizerWrapper::TokenizersPreTokenizerWrapper(
            TokenizersPreTokenizerWrapper::ByteLevel(from),
        )
    }
}
