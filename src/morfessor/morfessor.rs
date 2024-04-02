use std::{cmp, fs};
use std::path::Path;

use bytes::Bytes;
use prost::{DecodeError, Message};

pub mod morfessor {
    include!(concat!(env!("OUT_DIR"), "/morfessor.rs"));
}

pub fn decode_model<P: AsRef<Path>>(path: P) -> Result<morfessor::BaselineModel, DecodeError> {
    morfessor::BaselineModel::decode(Bytes::from(fs::read(path).unwrap()))
}

pub fn viterbi_segment(model: &morfessor::BaselineModel, compound: &str, add_count: f64, max_len: usize) -> (Vec<String>, f64) {
    let compound_length = compound.len();

    let mut grid: Vec<(f64, Option<usize>)> = vec![(0.0, None)];

    let corpus_tokens = model.corpus_coding.as_ref().unwrap().tokens as f64;
    let corpus_boundaries = model.corpus_coding.as_ref().unwrap().boundaries as f64;

    let log_tokens: f64 = if corpus_tokens + corpus_boundaries + add_count > 0.0 {
        (corpus_tokens + corpus_boundaries + add_count).ln() }
    else {
        0.0
    };

    let bad_likelihood = compound_length as f64 * log_tokens + 1.0;

    for t in 1..=compound_length {
        let mut best_path: Option<usize> = None;
        let mut best_cost: Option<f64> = None;

        let mut eval_path = |path: usize, cost: f64| {
            if best_cost.is_none() || cost < best_cost.unwrap() {
                best_path = Some(path);
                best_cost = Some(cost);
            }
        };

        // TODO implement nosplit_re

        for pt in cmp::max(0, t as isize - max_len as isize) as usize..t {
            // if grid[pt].0.is_nan() { // TODO this might be unnecessary
            //     continue;
            // }

            let mut cost = grid[pt].0;
            let construction = &compound[pt..t];

            if let Some(analyses) = model.analyses.get(construction) {
                if analyses.splitloc.is_empty() || analyses.splitloc[0] == 0 {
                    if analyses.count <= 0 {
                        panic!("Construction count of '{}' is {}", construction, analyses.count);
                    }

                    cost += log_tokens - (analyses.count as f64 + add_count).ln();

                    eval_path(pt, cost);

                    continue;
                }
            }

            if add_count == 0.0 {
                if construction.len() == 1 {
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
                    cost += add_count * add_count.ln() + get_code_length(lexicon_coding, construction) / corpus_weight;
                } else {
                    cost += log_tokens - add_count.ln() + (((lexicon_boundaries + add_count) * (lexicon_boundaries + add_count).ln()) - (lexicon_boundaries * lexicon_boundaries.ln()) + get_code_length(lexicon_coding, construction)) / corpus_weight;
                }

                eval_path(pt, cost);

                continue;
            }

            // if false { // TODO handle regex
            //     cost += construction.len() as f64 * bad_likelihood; eval_path(pt, cost); continue;
            // }

            eval_path(pt, cost);
        }

        grid.push((best_cost.unwrap_or(f64::NAN), best_path));
    }

    let mut constructions: Vec<String> = Vec::new();

    let mut cost = grid[compound_length].0; // TODO grid[grid.len() - 1] equiv?
    let mut path = grid[compound_length].1;

    let mut last_t = compound_length; // TODO clen + 1 in python code

    while let Some(t) = path {
        constructions.push(compound[t..last_t].to_string());
        path = grid[t].1;
        last_t = t;
    }

    constructions.reverse();

    cost += (corpus_tokens + corpus_boundaries).ln() - corpus_boundaries.ln();

    return (constructions, cost);
}

pub fn get_code_length(lexicon_encoding: &morfessor::LexiconEncoding, construction: &str) -> f64 {
    let l = construction.len() as f64 + 1.0;

    let mut cost = l * (lexicon_encoding.tokens as f64 + l).ln();

    cost -= (lexicon_encoding.boundaries as f64 + 1.0).ln();

    for atom in construction.chars() {
        let c = match lexicon_encoding.atoms.as_ref().unwrap().counts.get(&atom.to_string()) {
            Some(c) => *c,
            None => 1,
        };

        cost -= (c as f64).ln();
    }

    cost
}
