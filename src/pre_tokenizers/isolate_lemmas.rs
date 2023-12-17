use std::collections::HashMap;

use crate::unimorph::socket::split;

use serde::{Serialize, Deserialize};

use tokenizers::{PreTokenizedString, PreTokenizer, SplitDelimiterBehavior};
use tokenizers::utils::SysRegex;

use lazy_static::lazy_static;

fn bytes_char() -> HashMap<u8, char> {
    let mut bs: Vec<u8> = vec![];
    bs.extend(b'!'..=b'~');
    bs.extend(b'\xA1'..=b'\xAC');
    bs.extend(b'\xAE'..=b'\xFF');

    let mut cs: Vec<u32> = bs.iter().map(|i| *i as u32).collect();
    let mut n = 0;

    for b in 0..=255u8 {
        if !bs.contains(&b) {
            bs.push(b);
            cs.push(u32::pow(2, 8) + n);
            n += 1;
        }
    }

    bs.into_iter()
        .zip(cs)
        .map(|(f, t)| (f, unsafe { std::char::from_u32_unchecked(t) }))
        .collect()
}

lazy_static! {
    static ref RE: SysRegex = SysRegex::new(
        r"'s|'t|'re|'ve|'m|'ll|'d| ?\p{L}+| ?\p{N}+| ?[^\s\p{L}\p{N}]+|\s+(?!\S)|\s+"
    )
    .unwrap();
    static ref BYTES_CHAR: HashMap<u8, char> = bytes_char();
    static ref CHAR_BYTES: HashMap<char, u8> =
        bytes_char().into_iter().map(|(c, b)| (b, c)).collect();
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[derive(Default)]
#[derive(Serialize, Deserialize)]
pub struct IsolateLemmas {
    pub add_prefix_space: bool,
    pub trim_offsets: bool,
    pub use_regex: bool,
}

impl PreTokenizer for IsolateLemmas {
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
        pretokenized.split(|_, mut normalized| {
            let split = split(normalized.get())?;
            let mut new_chars: Vec<(char, isize)> = vec![];
            split.chars().for_each(|c| {
                if c == '#' {
                    new_chars.push((c, 1));
                } else {
                    new_chars.push((c, 0));
                }
            });
            normalized.transform(new_chars, 0);
            normalized.split("#", SplitDelimiterBehavior::Removed)
        })?;
        pretokenized.normalize(|normalized| {
            let s = normalized.get();
            let mut transformations: Vec<(char, isize)> = Vec::with_capacity(s.len());
            let mut i = 0;
            for cur_char in s.chars() {
                let size = cur_char.len_utf8();
                let bytes = s[i..i + size].as_bytes();
                i += size;
                transformations.extend(
                    bytes
                        .iter()
                        .enumerate()
                        .map(|(i, b)| (BYTES_CHAR[b], isize::from(i > 0))),
                );
            }
            normalized.transform(transformations, 0);
            Ok(())
        })
    }
}
