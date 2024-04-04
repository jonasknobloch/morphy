use serde::{Deserialize, Serialize};
use crate::morfessor::morfessor;
use crate::morfessor::morfessor::morfessor::BaselineModel;
use crate::morfessor::morfessor::viterbi_segment;
use crate::pre_tokenizers::pre_byte_level::PreByteLevel;
use crate::pre_tokenizers::segmenter::{Segmenter, SegmenterWrapper};
use crate::utils::offsets::{collect_grapheme_offsets, to_byte_offsets};

pub fn new_pre_tokenizer(add_prefix_space: bool, use_regex: bool, model_path: &str, model_config: MorfessorConfig) -> PreByteLevel {
    let model = match morfessor::decode_model(model_path) {
        Ok(model) => model,
        Err(_) => panic!("Failed to decode model"),
    };

    let segmenter = Morfessor {
        config: model_config,
        morfessor: model
    };

    PreByteLevel::new(add_prefix_space, use_regex, SegmenterWrapper::Morfessor(segmenter))
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Morfessor {
    config: MorfessorConfig,
    #[serde(skip_deserializing, skip_serializing)]
    morfessor: BaselineModel,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct MorfessorConfig {
    pub add_count: f64,
    pub max_len: usize,
    pub threshold: f64,
}

impl Default for MorfessorConfig {
    fn default() -> Self {
        MorfessorConfig {
            add_count: 0.0,
            max_len: 30,
            threshold: 50.0,
        }
    }
}

impl Segmenter for Morfessor {
    fn segment(&self, message: &str) -> Vec<(usize, usize)> {
        let (segments, score) = viterbi_segment(&self.morfessor, message, self.config.add_count, self.config.max_len);

        for segment in segments.iter() {
            if segment.chars().count() == 1 {
                return vec![(0, message.len())];
            }
        }

        if score > self.config.threshold {
            return vec![(0, message.len())];
        }

        to_byte_offsets(message, collect_grapheme_offsets(segments))
    }
}
