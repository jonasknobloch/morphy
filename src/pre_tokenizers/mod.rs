pub mod external;
pub mod sequence;
pub mod tree_split;
pub mod morfessor;

use crate::pre_tokenizers::morfessor::Morfessor;
use serde::{Deserialize, Serialize};

use tokenizers::pre_tokenizers::byte_level::ByteLevel;
use tokenizers::pre_tokenizers::PreTokenizerWrapper as TokenizersPreTokenizerWrapper;
use tokenizers::{PreTokenizedString, PreTokenizer};

use crate::pre_tokenizers::external::External;
use crate::pre_tokenizers::sequence::Sequence;
use crate::pre_tokenizers::tree_split::TreeSplit;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum PreTokenizerWrapper {
    External(External),
    TreeSplit(TreeSplit),
    Morfessor(Morfessor),
    Sequence(Sequence),
    TokenizersPreTokenizerWrapper(TokenizersPreTokenizerWrapper),
}

impl PreTokenizer for PreTokenizerWrapper {
    fn pre_tokenize(&self, normalized: &mut PreTokenizedString) -> tokenizers::Result<()> {
        match self {
            Self::External(ext) => ext.pre_tokenize(normalized),
            Self::TreeSplit(trs) => trs.pre_tokenize(normalized),
            Self::Morfessor(morf) => morf.pre_tokenize(normalized),
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

// PreTokenizerWrapper::from(TreeSplit::default());
// PreTokenizerWrapper::TreeSplit(TreeSplit::default());
impl From<TreeSplit> for PreTokenizerWrapper {
    fn from(from: TreeSplit) -> Self {
        PreTokenizerWrapper::TreeSplit(from)
    }
}

// PreTokenizerWrapper::from(Morfessor::default());
// PreTokenizerWrapper::Morfessor(Morfessor::default());
impl From<Morfessor> for PreTokenizerWrapper {
    fn from(from: Morfessor) -> Self {
        PreTokenizerWrapper::Morfessor(from)
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
