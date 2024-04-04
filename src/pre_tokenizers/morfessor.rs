use serde::{Deserialize, Serialize};
use crate::morfessor::morfessor;
use crate::morfessor::morfessor::morfessor::BaselineModel;
use crate::morfessor::morfessor::viterbi_segment;
use crate::pre_tokenizers::pre_byte_level::PreByteLevel;
use crate::pre_tokenizers::segmenter::{Segmenter, SegmenterWrapper};
use crate::utils::offsets::{collect_grapheme_offsets, to_byte_offsets};

pub fn new_pre_tokenizer(add_prefix_space: bool, use_regex: bool, model_path: &str) -> PreByteLevel {
    let model = match morfessor::decode_model(model_path) {
        Ok(model) => model,
        Err(_) => panic!("Failed to decode model"),
    };

    let segmenter = Morfessor {
        morfessor: model
    };

    PreByteLevel::new(add_prefix_space, use_regex, SegmenterWrapper::Morfessor(segmenter))
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Morfessor {
    #[serde(skip_deserializing, skip_serializing)]
    morfessor: BaselineModel,
}

impl Segmenter for Morfessor {
    fn segment(&self, message: &str) -> Vec<(usize, usize)> {
        let (segments, score) = viterbi_segment(&self.morfessor, message, 0.0, 30);

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

        to_byte_offsets(message, collect_grapheme_offsets(segments))
    }
}
