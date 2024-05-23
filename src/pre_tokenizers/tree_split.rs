use std::env;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::pre_tokenizers::pre_byte_level::PreByteLevel;
use crate::pre_tokenizers::segmenter::{Segmenter, SegmenterWrapper};

use crate::unimorph::unimorph::Unimorph;
use crate::utils::radix::split_path;

use radix_tree::{Node, Radix};

use crate::utils::offsets;

pub fn new_pre_tokenizer(
    add_prefix_space: bool,
    use_regex: bool,
    unimorph_dict: &str,
) -> PreByteLevel {
    let mut unimorph = Unimorph::new();

    let home = env::var("HOME").unwrap_or_else(|_| "".to_string());
    let dict = Path::new(&home).join(Path::new(unimorph_dict));

    let _ = unimorph.init(dict.to_str().unwrap());

    let segmenter = TreeSplit { unimorph };

    PreByteLevel::new(
        add_prefix_space,
        use_regex,
        SegmenterWrapper::TreeSplit(segmenter),
    )
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct TreeSplit {
    #[serde(skip_deserializing, skip_serializing)]
    pub unimorph: Unimorph,
}

impl Segmenter for TreeSplit {
    fn segment(&self, message: &str) -> Vec<(usize, usize)> {
        let chars = message.chars().collect::<Vec<char>>();

        let mut tree = Node::<char, bool>::new(chars.clone(), Some(true));

        let lemmas = self.unimorph.analyze(message);

        if lemmas.is_empty() {
            return vec![(0, message.len())];
        }

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

        // TODO use offset type char and byte instead of converting splits manually
        //  see tokenizers-0.15.0/src/tokenizer/pre_tokenizer.rs
        //  bpe pre_tokenizer test -> get_splits has offeset type argument

        // decomposed unicode sequences probably result in invalid character offsets
        // see utils/offsets.rs for details on differing unicode representations

        offsets::scalar_to_byte_offsets(message, split_path(tree, chars))
    }
}
