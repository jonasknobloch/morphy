pub mod external;
pub mod sequence;
pub mod pre_byte_level;
pub mod segmenter;
pub mod tree_split;
pub mod morfessor;

use serde::{Deserialize, Serialize};

use tokenizers::pre_tokenizers::byte_level::ByteLevel;
use tokenizers::pre_tokenizers::PreTokenizerWrapper as TokenizersPreTokenizerWrapper;
use tokenizers::{PreTokenizedString, PreTokenizer};

use crate::pre_tokenizers::external::External;
use crate::pre_tokenizers::pre_byte_level::PreByteLevel;
use crate::pre_tokenizers::sequence::Sequence;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum PreTokenizerWrapper {
    External(External),
    PreByteLevel(PreByteLevel),
    Sequence(Sequence),
    TokenizersPreTokenizerWrapper(TokenizersPreTokenizerWrapper),
}

impl PreTokenizer for PreTokenizerWrapper {
    fn pre_tokenize(&self, normalized: &mut PreTokenizedString) -> tokenizers::Result<()> {
        match self {
            Self::External(ext) => ext.pre_tokenize(normalized),
            Self::PreByteLevel(pbl) => pbl.pre_tokenize(normalized),
            Self::Sequence(seq) => seq.pre_tokenize(normalized),
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
impl From<PreByteLevel> for PreTokenizerWrapper {
    fn from(from: PreByteLevel) -> Self {
        PreTokenizerWrapper::PreByteLevel(from)
    }
}

// PreTokenizerWrapper::from(Sequence::default());
// PreTokenizerWrapper::TreeSplit(Sequence::default());
impl From<Sequence> for PreTokenizerWrapper {
    fn from(from: Sequence) -> Self {
        PreTokenizerWrapper::Sequence(from)
    }
}

impl From<TokenizersPreTokenizerWrapper> for PreTokenizerWrapper {
    fn from(from: TokenizersPreTokenizerWrapper) -> Self {
        PreTokenizerWrapper::TokenizersPreTokenizerWrapper(from)
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
