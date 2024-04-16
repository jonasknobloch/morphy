use crate::morfessor::morfessor;
use crate::morfessor::morfessor::morfessor::BaselineModel;
use crate::morfessor::morfessor::viterbi_segment;
use crate::pre_tokenizers::pre_byte_level::PreByteLevel;
use crate::pre_tokenizers::segmenter::{Segmenter, SegmenterWrapper};
use crate::utils::offsets::{collect_scalar_offsets, scalar_to_byte_offsets};
use serde::{Deserialize, Serialize};

pub fn new_pre_tokenizer(
    add_prefix_space: bool,
    use_regex: bool,
    model_path: &str,
    model_config: MorfessorConfig,
) -> PreByteLevel {
    let model = match morfessor::decode_model(model_path) {
        Ok(model) => model,
        Err(_) => panic!("Failed to decode model"),
    };

    let segmenter = Morfessor {
        config: model_config,
        morfessor: model,
    };

    PreByteLevel::new(
        add_prefix_space,
        use_regex,
        SegmenterWrapper::Morfessor(segmenter),
    )
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Morfessor {
    pub config: MorfessorConfig,
    #[serde(skip_deserializing, skip_serializing)]
    pub morfessor: BaselineModel,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct MorfessorConfig {
    pub viterbi_smoothing: f64,
    pub viterbi_max_len: usize,
    pub rejection_threshold: f64,
    pub reject_single_char_sequences: bool,
}

impl Default for MorfessorConfig {
    fn default() -> Self {
        MorfessorConfig {
            viterbi_smoothing: 0.0, // 0.0 command-line arg default / 1.0 viterbi_segment arg default
            viterbi_max_len: 30,
            rejection_threshold: 50.0,
            reject_single_char_sequences: false,
        }
    }
}

impl Segmenter for Morfessor {
    fn segment(&self, message: &str) -> Vec<(usize, usize)> {
        let (segments, score) = viterbi_segment(
            &self.morfessor,
            message,
            self.config.viterbi_smoothing,
            self.config.viterbi_max_len,
        );

        if score > self.config.rejection_threshold {
            return vec![(0, message.len())];
        }

        if self.config.reject_single_char_sequences {
            let mut prev = 0;

            for segment in segments.iter() {
                let count = segment.chars().count();

                if prev == 1 && count == 1 {
                    return vec![(0, message.len())];
                }

                prev = count;
            }
        }

        scalar_to_byte_offsets(message, collect_scalar_offsets(segments))
    }
}
