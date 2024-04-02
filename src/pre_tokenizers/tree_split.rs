use std::env;
use std::path::Path;

use lazy_static::lazy_static;

use serde::{Deserialize, Serialize};
use tokenizers::utils::{macro_rules_attribute, SysRegex};
use tokenizers::{impl_serde_type, SplitDelimiterBehavior};

use tokenizers::normalizer::Range;
use tokenizers::{NormalizedString, Offsets, PreTokenizedString, PreTokenizer};

use crate::unimorph::unimorph::Unimorph;
use crate::utils::radix::split_path;

use radix_tree::{Node, Radix};

lazy_static! {
    static ref RE: SysRegex = SysRegex::new(
        r"'s|'t|'re|'ve|'m|'ll|'d| ?\p{L}+| ?\p{N}+| ?[^\s\p{L}\p{N}]+|\s+(?!\S)|\s+"
    )
    .unwrap();
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[macro_rules_attribute(impl_serde_type!)]
pub struct TreeSplit {
    add_prefix_space: bool,
    use_regex: bool,
    #[serde(skip_serializing)]
    unimorph: Unimorph,
}

impl TreeSplit {
    pub fn new(add_prefix_space: bool, use_regex: bool) -> Self {
        let mut tree_split = Self::default();

        tree_split.add_prefix_space = add_prefix_space;
        tree_split.use_regex = use_regex;

        return tree_split;
    }
}

impl Default for TreeSplit {
    fn default() -> Self {
        let mut unimorph = Unimorph::new();

        let home = env::var("HOME").unwrap_or_else(|_| "".to_string());
        let dict = Path::new(&home).join(Path::new(".unimorph/ces/ces"));

        // println!("{}", dict.to_str().unwrap());

        let _ = unimorph.init(dict.to_str().unwrap());

        return Self {
            add_prefix_space: false,
            use_regex: true,
            unimorph,
        };
    }
}

impl TreeSplit {
    pub fn split(&self, message: &str) -> Vec<Offsets> {
        let lemmas = self.unimorph.analyze(message);

        if lemmas.is_empty() {
            return vec![(0, message.len())];
        }

        let chars = message.chars().collect::<Vec<char>>();

        let mut tree = Node::<char, bool>::new(chars.clone(), Some(true));

        for lemma in lemmas {
            // if !lemma.starts_with(message) {
            //     continue;
            // }

            for form in self.unimorph.analyze(lemma.as_str()) {
                if form == message {
                    continue;
                }

                if !form.starts_with(message) && !message.starts_with(&form.to_owned()) {
                    continue;
                }

                tree.insert(form.chars().collect::<Vec<char>>(), true);
            }
        }

        // convert character offsets into byte offsets

        // TODO use offset type char and byte instead of converting splits manually
        //  see tokenizers-0.15.0/src/tokenizer/pre_tokenizer.rs
        //  bpe pre_tokenizer test -> get_splits has offeset type argument

        let mut byte_offsets: Vec<(usize, usize)> = vec![];

        let mut index = 0;

        for offsets in split_path(tree, message.chars().collect()) {
            let length = chars[offsets.0..offsets.1]
                .iter()
                .map(|c| c.len_utf8())
                .sum::<usize>();

            byte_offsets.push((index, index + length));

            index += length;
        }

        return byte_offsets;
    }
}

impl PreTokenizer for TreeSplit {
    fn pre_tokenize(&self, pretokenized: &mut PreTokenizedString) -> tokenizers::Result<()> {
        let re_ref: &SysRegex = &RE;
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
