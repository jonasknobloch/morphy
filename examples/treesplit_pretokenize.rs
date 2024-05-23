use mbpe::pre_tokenizers::sequence::Sequence;
use mbpe::pre_tokenizers::tree_split;
use mbpe::pre_tokenizers::PreTokenizerWrapper;
use tokenizers::pre_tokenizers::byte_level::ByteLevel;
use tokenizers::{OffsetReferential, OffsetType, PreTokenizedString, PreTokenizer};

pub fn main() {
    let tree_split = tree_split::new_pre_tokenizer(false, true, ".unimorph/eng/eng");

    let pre_tokenizer = Sequence::new(vec![
        PreTokenizerWrapper::from(tree_split),
        PreTokenizerWrapper::from(ByteLevel::new(false, true, false)),
    ]);

    let mut pre_tokenized = PreTokenizedString::from("That's some impressive retrofitting!");

    let _ = pre_tokenizer.pre_tokenize(&mut pre_tokenized);

    println!(
        "{:?}",
        pre_tokenized.get_splits(OffsetReferential::Original, OffsetType::Byte)
    );
}
