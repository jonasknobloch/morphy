pub mod isolate_lemmas;
pub mod sequence;

use serde::{Deserialize, Serialize};

use tokenizers::{PreTokenizedString, PreTokenizer};
use tokenizers::pre_tokenizers::byte_level::ByteLevel;
use tokenizers::pre_tokenizers::PreTokenizerWrapper as TokenizersPreTokenizerWrapper;

use crate::pre_tokenizers::isolate_lemmas::IsolateLemmas;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum PreTokenizerWrapper {
    IsolateLemmas(IsolateLemmas),
    TokenizersPreTokenizerWrapper(TokenizersPreTokenizerWrapper),
}

impl PreTokenizer for PreTokenizerWrapper {
    fn pre_tokenize(&self, normalized: &mut PreTokenizedString) -> tokenizers::Result<()> {
        match self {
            Self::IsolateLemmas(bpt) => bpt.pre_tokenize(normalized),
            Self::TokenizersPreTokenizerWrapper(ptw) => ptw.pre_tokenize(normalized),
        }
    }
}

// PreTokenizerWrapper::from(IsolateLemmas::default());
// PreTokenizerWrapper::IsolateLemmas(IsolateLemmas::default());
impl From<IsolateLemmas> for PreTokenizerWrapper {
    fn from(from: IsolateLemmas) -> Self {
        PreTokenizerWrapper::IsolateLemmas(from)
    }
}

// PreTokenizerWrapper::from(ByteLevel::default());
// PreTokenizerWrapper::TokenizersPreTokenizerWrapper(TokenizersPreTokenizerWrapper::ByteLevel(ByteLevel::default()));
impl From<ByteLevel> for PreTokenizerWrapper {
    fn from(from: ByteLevel) -> Self {
        PreTokenizerWrapper::TokenizersPreTokenizerWrapper(TokenizersPreTokenizerWrapper::ByteLevel(from))
    }
}
