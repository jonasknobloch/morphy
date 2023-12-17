use morphy::pre_tokenizers::isolate_lemmas::IsolateLemmas;
use tokenizers::{OffsetReferential, OffsetType, PreTokenizedString, PreTokenizer};

pub fn main() {
    let pre_tokenizer = IsolateLemmas{
        add_prefix_space: true,
        trim_offsets: true,
        use_regex: true,
    };

    let mut pre_tokenized = PreTokenizedString::from("That's some impressive retrofitting!");

    let _ = pre_tokenizer.pre_tokenize(&mut pre_tokenized);

    println!(
        "{:?}",
        pre_tokenized.get_splits(OffsetReferential::Original, OffsetType::Byte)
    );
}
