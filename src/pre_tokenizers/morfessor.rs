use lazy_static::lazy_static;
use tokenizers::{NormalizedString, Offsets, PreTokenizedString, PreTokenizer};

use serde::{Deserialize, Serialize};
use tokenizers::utils::{macro_rules_attribute, SysRegex};
use tokenizers::{impl_serde_type, SplitDelimiterBehavior};
use tokenizers::normalizer::Range;

use crate::morfessor::morfessor;
use crate::morfessor::morfessor::morfessor::BaselineModel;
use crate::morfessor::morfessor::viterbi_segment;


lazy_static! {
    static ref RE: SysRegex = SysRegex::new(
        r"'s|'t|'re|'ve|'m|'ll|'d| ?\p{L}+| ?\p{N}+| ?[^\s\p{L}\p{N}]+|\s+(?!\S)|\s+"
    )
    .unwrap();
}

#[derive(Clone, Debug, PartialEq)]
#[macro_rules_attribute(impl_serde_type!)]
pub struct Morfessor {
    #[serde(skip_serializing)]
    pub baseline: BaselineModel,
}

impl Morfessor {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Morfessor {
    fn default() -> Self {
        let model = match morfessor::decode_model("scripts/unsup_model.proto") {
            Ok(model) => model,
            Err(_) => panic!("Failed to decode model"),
        };

        return Self {
            baseline: model,
        }
    }

}

impl Morfessor {
    pub fn split(&self, message: &str) -> Vec<Offsets> {
        let (segments, score) = viterbi_segment(&self.baseline, message, 0.0, 30);

        if score < 50.0 && segments.len() > 1 {
            let mut foo = true;

            for segment in segments.clone().iter() {
                if segment.clone().chars().count() == 1 {
                    foo = false;
                    break;
                }
            }

            if foo {
                // println!("Segments: {:?}, Score: {}", segments, score);
            } else {
                return vec![(0, message.len())];
            }
        } else {
            return vec![(0, message.len())];
        }

        let mut byte_offsets: Vec<(usize, usize)> = vec![];

        // byte_offsets.push((0, message.len()));

        let mut index = 0;

        for segment in segments.iter() {
            let length = segment.chars().count();

            byte_offsets.push((index, index+length)); // TODO make unicode compatible

            index += length
        }

        return byte_offsets;
    }
}

impl PreTokenizer for Morfessor {
    // fn pre_tokenize(&self, pretokenized: &mut PreTokenizedString) -> tokenizers::Result<()> {
    //
    //     pretokenized.split(|_, normalized| {
    //         let segments = viterbi_segment(&self.baseline, normalized.get(), 0.0, 30);
    //
    //         println!("{:?}", segments);
    //
    //         normalized.split("foobar", SplitDelimiterBehavior::Isolated)
    //     })
    //
    //     // Ok(())
    // }

    fn pre_tokenize(&self, pretokenized: &mut PreTokenizedString) -> tokenizers::Result<()> {
        let re_ref: &SysRegex = &crate::pre_tokenizers::morfessor::RE;
        pretokenized.split(|_, mut normalized| {
            if !normalized.get().starts_with(' ') {
                // normalized.prepend(" "); // TODO add option to disable via arg
            }

            normalized.split(re_ref, SplitDelimiterBehavior::Isolated)
        })?;
        pretokenized.split(|_, normalized| {
            let form = normalized.get();

            let splits = if form.starts_with(' ') {
                self.split(&form[1..])
            } else {
                self.split(form)
            };

            // println!("{:?} {:?}", form, splits);

            let mut result: Vec<NormalizedString> = vec![];

            for (i, offsets) in splits.iter().enumerate() {
                if form.starts_with(' ') {
                    if i == 0 {
                        result.push(
                            normalized
                                .slice(Range::Normalized(offsets.0..offsets.1+1))
                                .unwrap(),
                        );
                    } else {
                        result.push(
                            normalized
                                .slice(Range::Normalized(offsets.0+1..offsets.1+1))
                                .unwrap(),
                        );
                    }

                    continue;
                }

                result.push(
                    normalized
                        .slice(Range::Normalized(offsets.0..offsets.1))
                        .unwrap(),
                );
            }

            if result.is_empty() {
                panic!();
            }

            return Ok(result);
        })
    }
}