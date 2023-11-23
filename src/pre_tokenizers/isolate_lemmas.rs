use serde::{Serialize, Deserialize};
use tokenizers::{PreTokenizedString, PreTokenizer};

#[derive(Clone, Debug, PartialEq, Eq)]
#[derive(Default)]
#[derive(Serialize, Deserialize)]
pub struct IsolateLemmas<PT> {
    pub pre_tokenizer: PT
}

impl<PT: PreTokenizer> PreTokenizer for IsolateLemmas<PT> {
    fn pre_tokenize(&self, pretokenized: &mut PreTokenizedString) -> tokenizers::Result<()> {
        // TODO implement

        self.pre_tokenizer.pre_tokenize(pretokenized)
    }
}
