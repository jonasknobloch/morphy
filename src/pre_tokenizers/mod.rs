pub mod isolate_lemmas;
pub mod sequence;

use serde::{Deserialize, Serialize};

use tokenizers::pre_tokenizers::byte_level::ByteLevel;
use tokenizers::{PreTokenizedString, PreTokenizer};

use crate::pre_tokenizers::isolate_lemmas::IsolateLemmas;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum PreTokenizerWrapper {
    ByteLevel(ByteLevel),
    IsolateLemmas(IsolateLemmas),
}

impl PreTokenizer for PreTokenizerWrapper {
    fn pre_tokenize(&self, normalized: &mut PreTokenizedString) -> tokenizers::Result<()> {
        match self {
            Self::ByteLevel(bpt) => bpt.pre_tokenize(normalized),
            Self::IsolateLemmas(bpt) => bpt.pre_tokenize(normalized),
        }
    }
}

macro_rules! impl_enum_from (
    ($from_ty:ty, $enum:ty, $variant:ident) => {
        impl From<$from_ty> for $enum {
            fn from(from: $from_ty) -> Self {
                <$enum>::$variant(from)
            }
        }
    }
);

impl_enum_from!(ByteLevel, PreTokenizerWrapper, ByteLevel);
impl_enum_from!(IsolateLemmas, PreTokenizerWrapper, IsolateLemmas);
