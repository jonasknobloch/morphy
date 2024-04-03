use morphy::pre_tokenizers::sequence::Sequence;
use morphy::pre_tokenizers::morfessor::Morfessor;
use morphy::pre_tokenizers::PreTokenizerWrapper;
use tokenizers::decoders::byte_level::ByteLevel;
use tokenizers::models::bpe::BpeTrainerBuilder;
use tokenizers::{AddedToken, DecoderWrapper, Model, ModelWrapper, NormalizerWrapper, PostProcessorWrapper, Result, Tokenizer, tokenizer, TokenizerBuilder};
use tokenizers::models::TrainerWrapper;

fn main() -> Result<()> {
    let gpt2_tokenizer = Tokenizer::from_pretrained("gpt2", None)?;

    let pre_tokenizer = Sequence::new(vec![
        PreTokenizerWrapper::from(Morfessor::new()),
        PreTokenizerWrapper::from(ByteLevel::new(false, true, false)),
    ]);

    // recreate gpt2 tokenizer since we can't replace pre_tokenizer because of type constraints

    let mut tokenizer = TokenizerBuilder::<
        ModelWrapper,
        NormalizerWrapper,
        PreTokenizerWrapper,
        PostProcessorWrapper,
        DecoderWrapper,
    >::default()
    .with_model(gpt2_tokenizer.get_model().clone())
    .with_pre_tokenizer(Some(PreTokenizerWrapper::from(pre_tokenizer)))
    .with_post_processor(gpt2_tokenizer.get_post_processor().cloned())
    .with_decoder(gpt2_tokenizer.get_decoder().cloned())
    .build()
    .unwrap();

    let mut trainer = TrainerWrapper::from(BpeTrainerBuilder::new()
        .show_progress(true)
        .vocab_size(50256)
        .min_frequency(0)
        .build());

    tokenizer
        .train_from_files(
            &mut trainer,
            vec!["data/tiny_shakespeare.txt".to_string()],
            // vec!["/Users/jonas/Developer/czech-gpt/en_part_00000.txt".to_string()],
        )?;


    // add special end_of_text token

    let mut end_of_text = AddedToken::from(String::from("<|endoftext|>"), true);

    end_of_text.normalized = true; // match gpt2 (no clue why this is done)

    // tokenizer.add_special_tokens(&[end_of_text]);

    tokenizer.add_special_tokens(&[end_of_text]);

    // replace pre_tokenizer before saving

    tokenizer.with_pre_tokenizer(PreTokenizerWrapper::from(gpt2_tokenizer.get_pre_tokenizer().cloned().unwrap()));

    tokenizer.save("tokenizer_gpt2+morf_cx-en_00000-00000_50k.json", false)?;

    Ok(())
}

