use mbpe::pre_tokenizers::sequence::Sequence;
use mbpe::pre_tokenizers::PreTokenizerWrapper;
use tokenizers::pre_tokenizers::byte_level::ByteLevel;
use tokenizers::{OffsetReferential, OffsetType, PreTokenizedString, PreTokenizer};
use mbpe::pre_tokenizers::morfessor::{MorfessorConfig, new_pre_tokenizer};

pub fn main() {
    let morfessor = new_pre_tokenizer(false, true, "scripts/unsup_model.proto", MorfessorConfig::default());

    let pre_tokenizer = Sequence::new(vec![
        PreTokenizerWrapper::from(morfessor),
        PreTokenizerWrapper::from(ByteLevel::new(false, true, false)),
    ]);

    let mut pre_tokenized = PreTokenizedString::from("That's some impressive retrofitting!");

    let _ = pre_tokenizer.pre_tokenize(&mut pre_tokenized);

    println!(
        "{:?}",
        pre_tokenized.get_splits(OffsetReferential::Original, OffsetType::Byte)
    );
}
