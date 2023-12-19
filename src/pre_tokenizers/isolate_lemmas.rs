use serde::{Serialize, Deserialize};
use tokenizers::{PreTokenizedString, PreTokenizer, SplitDelimiterBehavior};

use crate::unimorph::socket::split;

#[derive(Clone, Debug, PartialEq, Eq)]
#[derive(Default)]
#[derive(Serialize, Deserialize)]
pub struct IsolateLemmas {
    pub with_prefix: String,
    pub split_delimiter: char,
}

impl PreTokenizer for IsolateLemmas {
    fn pre_tokenize(&self, pretokenized: &mut PreTokenizedString) -> tokenizers::Result<()> {
        pretokenized.split(|_, mut normalized| {
            let form = normalized.get();

            let has_prefix = form.starts_with(self.with_prefix.as_str());

            let prefix = if has_prefix {self.with_prefix.as_str() } else { "" };
            let tail = if has_prefix { form.strip_prefix(prefix).unwrap() } else { form };

            if has_prefix & tail.is_empty() {
                return Ok(vec![normalized]);
            }

            let split  =  prefix.to_owned() + split(tail)?.as_str();

            let mut new_chars: Vec<(char, isize)> = vec![];

            split.chars().for_each(|c| {
                if c == self.split_delimiter {
                    new_chars.push((c, 1));
                } else {
                    new_chars.push((c, 0));
                }
            });

            normalized.transform(new_chars, 0);
            normalized.split(self.split_delimiter, SplitDelimiterBehavior::Removed)
        })
    }
}
