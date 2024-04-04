use crate::pre_tokenizers::tree_split::TreeSplit;
use crate::pre_tokenizers::morfessor::Morfessor;

use serde::{Deserialize, Serialize};

pub trait Segmenter {
    fn segment(&self, message: &str) -> Vec<(usize, usize)>;
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum SegmenterWrapper {
    TreeSplit(TreeSplit),
    Morfessor(Morfessor),
}

impl Segmenter for SegmenterWrapper {
    fn segment(&self, message: &str) -> Vec<(usize, usize)> {
        match self {
            SegmenterWrapper::TreeSplit(ts) => ts.segment(message),
            SegmenterWrapper::Morfessor(mf) => mf.segment(message),
        }
    }
}