use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use tokenizers::normalizer::Range;
use tokenizers::utils::{macro_rules_attribute, SysRegex};
use tokenizers::{impl_serde_type, NormalizedString, PreTokenizedString, PreTokenizer, SplitDelimiterBehavior};

use crate::pre_tokenizers::segmenter::{Segmenter, SegmenterWrapper};

lazy_static! {
    static ref RE: SysRegex = SysRegex::new(
        r"'s|'t|'re|'ve|'m|'ll|'d| ?\p{L}+| ?\p{N}+| ?[^\s\p{L}\p{N}]+|\s+(?!\S)|\s+"
    )
    .unwrap();
}

#[derive(Clone, Debug, PartialEq)]
#[macro_rules_attribute(impl_serde_type!)]
#[non_exhaustive]
pub struct PreByteLevel {
    add_prefix_space: bool,
    use_regex: bool,
    segmenter: SegmenterWrapper,
}

impl PreByteLevel {
    pub fn new(add_prefix_space: bool, use_regex: bool, segmenter: SegmenterWrapper) -> Self {
        Self {
            add_prefix_space,
            use_regex,
            segmenter,
        }
    }
}

impl PreTokenizer for PreByteLevel {
    fn pre_tokenize(&self, pretokenized: &mut PreTokenizedString) -> tokenizers::Result<()> {
        let re_ref: &SysRegex = &RE;

        pretokenized.split(|_, mut normalized| {
            if self.add_prefix_space && !normalized.get().starts_with(' ') {
                normalized.prepend(" ");
            }

            if self.use_regex {
                normalized.split(re_ref, SplitDelimiterBehavior::Isolated)
            } else {
                Ok(vec![normalized])
            }
        })?;

        pretokenized.split(|_, normalized| {
            let form = normalized.get();

            let prefix_length = if form.starts_with(' ') { 1 } else { 0 };

            let splits = self.segmenter.segment(&form[prefix_length..]);

            let mut result: Vec<NormalizedString> = vec![];

            for (i, offsets) in splits.iter().enumerate() {
                let left = if i == 0 {
                    offsets.0
                } else {
                    offsets.0 + prefix_length
                };

                let right= offsets.1 + prefix_length;

                result.push(
                    normalized
                        .slice(Range::Normalized(left..right))
                        .unwrap(),
                );
            }

            if result.is_empty() {
                panic!("empty segmentation");
            }

            return Ok(result);
        })
    }
}