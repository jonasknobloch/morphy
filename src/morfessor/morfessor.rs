use std::fs;
use std::path::Path;

use bytes::Bytes;
use prost::{DecodeError, Message};

use serde::{Deserialize, Serialize};

use crate::utils::offsets::unicode_scalar_bounds;

pub mod morfessor {
    include!(concat!(env!("OUT_DIR"), "/morfessor.rs"));
}

pub fn decode_model<P: AsRef<Path>>(path: P) -> Result<morfessor::BaselineModel, DecodeError> {
    morfessor::BaselineModel::decode(Bytes::from(fs::read(path).unwrap()))
}

pub fn viterbi_segment(
    model: &morfessor::BaselineModel,
    compound: &str,
    add_count: f64,
    max_len: usize,
) -> (Vec<String>, f64) {
    let compound_length = unicode_scalar_bounds(compound).len();

    let mut grid: Vec<(f64, Option<usize>)> = vec![(0.0, None)];

    let corpus_tokens = model.corpus_coding.as_ref().unwrap().tokens as f64;
    let corpus_boundaries = model.corpus_coding.as_ref().unwrap().boundaries as f64;

    let log_tokens: f64 = if corpus_tokens + corpus_boundaries + add_count > 0.0 {
        (corpus_tokens + corpus_boundaries + add_count).ln()
    } else {
        0.0
    };

    let bad_likelihood = compound_length as f64 * log_tokens + 1.0;

    let bounds = unicode_scalar_bounds(compound);

    let mut bounds_upper = bounds.clone();
    let mut bounds_lower = bounds.clone();

    bounds_lower.pop();
    bounds_lower.insert(0, 0);

    for t in bounds_upper {
        let mut best_path: Option<usize> = None;
        let mut best_cost: Option<f64> = None;

        let mut eval_path = |path: usize, cost: f64| {
            if best_cost.is_none() || cost < best_cost.unwrap() {
                best_path = Some(path);
                best_cost = Some(cost);
            }
        };

        // TODO implement nosplit_re

        for pt in bounds_lower.clone() {
            if pt >= t {
                break; // up to but not including t
            }

            let construction = &compound[pt..t];

            if construction.chars().count() > max_len {
                continue;
            }

            let mut cost = grid[pt].0;

            // if cost.is_nan() { // TODO this might be unnecessary
            //     continue;
            // }

            if let Some(analyses) = model.analyses.get(construction) {
                if analyses.splitloc.is_empty() || analyses.splitloc[0] == 0 {
                    if analyses.count <= 0 {
                        panic!(
                            "Construction count of '{}' is {}",
                            construction, analyses.count
                        );
                    }

                    cost += log_tokens - (analyses.count as f64 + add_count).ln();

                    eval_path(pt, cost);

                    continue;
                }
            }

            if add_count == 0.0 {
                if unicode_scalar_bounds(construction).len() == 1 {
                    cost += bad_likelihood;

                    eval_path(pt, cost);
                }

                continue;
            }

            if add_count > 0.0 {
                let lexicon_coding = model.lexicon_coding.as_ref().unwrap();
                let corpus_coding = model.corpus_coding.as_ref().unwrap();

                let lexicon_boundaries = lexicon_coding.boundaries as f64;
                let corpus_weight = corpus_coding.weight as f64;

                if model.corpus_coding.as_ref().unwrap().tokens == 0 {
                    cost += add_count * add_count.ln()
                        + get_code_length(lexicon_coding, construction) / corpus_weight;
                } else {
                    cost += log_tokens - add_count.ln()
                        + (((lexicon_boundaries + add_count)
                            * (lexicon_boundaries + add_count).ln())
                            - (lexicon_boundaries * lexicon_boundaries.ln())
                            + get_code_length(lexicon_coding, construction))
                            / corpus_weight;
                }

                eval_path(pt, cost);

                continue;
            }

            // if false { // TODO handle regex
            //     cost += construction.len() as f64 * bad_likelihood; eval_path(pt, cost); continue;
            // }

            eval_path(pt, cost);
        }

        if best_path.is_none() {
            panic!("No best path");
        }

        // pad grid to account for multibyte characters
        // note that grid is later iterated in reverse

        while grid.len() < t {
            grid.push((f64::NAN, None));
        }

        grid.push((best_cost.unwrap(), best_path));
    }

    let mut constructions: Vec<String> = Vec::new();

    if grid.len() != compound.as_bytes().len() + 1 {
        panic!("Invalid grid length");
    }

    let mut cost = grid[grid.len() - 1].0;
    let mut path = grid[grid.len() - 1].1;

    let mut last_t = compound.as_bytes().len();

    while let Some(t) = path {
        constructions.push(compound[t..last_t].to_string());
        path = grid[t].1;
        last_t = t;
    }

    constructions.reverse();

    cost += (corpus_tokens + corpus_boundaries).ln() - corpus_boundaries.ln();

    if constructions.len() == 0 {
        panic!("No constructions");
    }

    return (constructions, cost);
}

pub fn get_code_length(lexicon_encoding: &morfessor::LexiconEncoding, construction: &str) -> f64 {
    let l = construction.chars().count() as f64 + 1.0;

    let mut cost = l * (lexicon_encoding.tokens as f64 + l).ln();

    cost -= (lexicon_encoding.boundaries as f64 + 1.0).ln();

    for atom in construction.chars() {
        let c = match lexicon_encoding
            .atoms
            .as_ref()
            .unwrap()
            .counts
            .get(&atom.to_string())
        {
            Some(c) => *c,
            None => 1,
        };

        cost -= (c as f64).ln();
    }

    cost
}

mod tests {
    use super::*;

    #[test]
    fn test_get_code_length_composed() {
        let model = decode_model("scripts/unsup_model.proto").unwrap();

        let lexicon_encoding = model.lexicon_coding.as_ref().unwrap();

        let cost = get_code_length(lexicon_encoding, "\u{00E9}");

        assert_eq!(cost, 14.375011301554892);
    }

    #[test]
    fn test_get_code_length_decomposed() {
        let model = decode_model("scripts/unsup_model.proto").unwrap();

        let lexicon_encoding = model.lexicon_coding.as_ref().unwrap();

        let cost = get_code_length(lexicon_encoding, "\u{0065}\u{0301}");

        assert_eq!(cost, 16.65337147997293);
    }

    #[test]
    fn test_viterbi_segment() {
        let model = decode_model("scripts/unsup_model.proto").unwrap();

        let (segments, score) = viterbi_segment(&model, "unfoobared", 0.0, 30);

        assert_eq!(segments, vec!["un", "foo", "bar", "ed"]);
        assert_eq!(score, 32.684465337620665);
    }

    #[test]
    fn test_viterbi_segment_composed() {
        let model = decode_model("scripts/unsup_model.proto").unwrap();

        let (segments, score) = viterbi_segment(&model, "brul\u{00E9}e", 0.0, 30);

        assert_eq!(segments, vec!["bru", "l", "\u{00E9}", "e"]);
        assert_eq!(score, 109.47779723820601);
    }

    #[test]
    fn test_viterbi_segment_decomposed() {
        let model = decode_model("scripts/unsup_model.proto").unwrap();

        let (segments, score) = viterbi_segment(&model, "brul\u{0065}\u{0301}e", 0.0, 30);

        assert_eq!(segments, vec!["brul\u{0065}", "\u{0301}", "e"]);
        assert_eq!(score, 118.92118396646775);
    }

    #[test]
    fn test_viterbi_segment_max_len() {
        let model = decode_model("scripts/unsup_model.proto").unwrap();

        let (segments, score) = viterbi_segment(&model, "unsupervised", 0.0, 5);

        assert_eq!(segments, vec!["un", "super", "vis", "ed"]);
        assert_eq!(score, 29.684031672881893);
    }
}
