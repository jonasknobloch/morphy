use morphy::pre_tokenizers::sequence::Sequence;
use morphy::pre_tokenizers::tree_split::TreeSplit;
use morphy::pre_tokenizers::PreTokenizerWrapper;
use tokenizers::decoders::byte_level::ByteLevel;
use tokenizers::models::bpe::{BPE, BpeTrainerBuilder};
use tokenizers::{AddedToken, DecoderWrapper, Model, ModelWrapper, NormalizerWrapper, PostProcessorWrapper, Result, Tokenizer, tokenizer, TokenizerBuilder};
use tokenizers::models::TrainerWrapper;

fn main() -> Result<()> {
    // let mut tokenizer = TokenizerBuilder::<
    //     ModelWrapper,
    //     NormalizerWrapper,
    //     PreTokenizerWrapper,
    //     PostProcessorWrapper,
    //     DecoderWrapper,
    // >::default()
    // .with_model(ModelWrapper::from(BPE::default()))
    // .with_pre_tokenizer(Some(PreTokenizerWrapper::from(Sequence::new(vec![
    //     PreTokenizerWrapper::from(TreeSplit::new(false, true)),
    //     PreTokenizerWrapper::from(ByteLevel::new(false, true, false)),
    // ]))))
    // .with_post_processor(Some(PostProcessorWrapper::from(ByteLevel::default())))
    // .with_decoder(Some(DecoderWrapper::from(ByteLevel::default())))
    // .build()
    // .unwrap();

    let gpt2_tokenizer = Tokenizer::from_pretrained("gpt2", None)?;

    let pre_tokenizer = Sequence::new(vec![
        PreTokenizerWrapper::from(TreeSplit::new(false, true)),
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
    // .with_pre_tokenizer(Option::from(PreTokenizerWrapper::from(gpt2_tokenizer.get_pre_tokenizer().cloned().unwrap())))
    .with_post_processor(gpt2_tokenizer.get_post_processor().cloned())
    .with_decoder(gpt2_tokenizer.get_decoder().cloned())
    .build()
    .unwrap();

    // let mut gpt2_trainer = tokenizer.get_model().get_trainer();

    let mut trainer = TrainerWrapper::from(BpeTrainerBuilder::new()
        .show_progress(true)
        .vocab_size(50256)
        .min_frequency(0)
        .build());

    // tokenizer
    //     .train_from_files(
    //         &mut trainer,
    //         // vec!["data/tiny_shakespeare.txt".to_string()],
    //         // vec!["/Users/jonas/Developer/czech-gpt/en_part_00000.txt".to_string()],
    //         // vec!["/Users/jonas/Developer/czech-gpt/cs_part_00000.txt".to_string()],
    //         vec![
    //             "/Users/jonas/Developer/czech-gpt/cs_part_00000.txt".to_string(),
    //             "/Users/jonas/Developer/czech-gpt/cs_part_00001.txt".to_string(),
    //         ],
    //     )?;

    let mut import = Tokenizer::from_file("tokenizer_gpt2+ts_cx-cs_00000-00001_50k+no-eot.json")?;

    // tokenizer.train_from_files(&mut trainer, vec!["data/tiny_shakespeare.txt".to_string()])?;

    // add special end_of_text token

    let mut end_of_text = AddedToken::from(String::from("<|endoftext|>"), true);

    end_of_text.normalized = true; // match gpt2 (no clue why this is done)

    // tokenizer.add_special_tokens(&[end_of_text]);

    import.add_special_tokens(&[end_of_text]);

    import.save("tokenizer_gpt2+ts_cx-cs_00000-00001_50k.json", false)?;

    // replace pre_tokenizer before saving

    // tokenizer.with_pre_tokenizer(PreTokenizerWrapper::from(gpt2_tokenizer.get_pre_tokenizer().cloned().unwrap()));

    //tokenizer.save("tokenizer_gpt2+ts_cx-cs_00000-00001_50k+no-eot.json", false)?;

    Ok(())
}

// [01:07:35] Pre-processing files (3932 Mo) ███████████████████████████████████████████████████████████████████████████████                100%
// [00:00:52] Tokenize words                 ███████████████████████████████████████████████████████████████████████████████ 4923123  /  4923123
// [00:22:14] Count pairs                    ███████████████████████████████████████████████████████████████████████████████ 4923123  /  4923123
// [00:02:37] Compute merges                 ███████████████████████████████████████████████████████████████████████████████ 29766    /    29766

// tokenizer_gpt2+ts_cx-en_00000-00000_50k+no-eot.json
// [01:27:44] Pre-processing files (3932 Mo) ██████████████████████████████████████████                100%
// [00:00:53] Tokenize words                 ██████████████████████████████████████████ 4923123  /  4923123
// [00:14:18] Count pairs                    ██████████████████████████████████████████ 4923123  /  4923123
// [00:02:39] Compute merges                 ██████████████████████████████████████████ 50022    /    50022

// tokenizer_gpt2_cx-en_00000-00000_50k+no-eot.json
// [00:44:28] Pre-processing files (3932 Mo) ██████████████████████████████████████████                100%
// [00:00:53] Tokenize words                 ██████████████████████████████████████████ 4992469  /  4992469
// [00:16:22] Count pairs                    ██████████████████████████████████████████ 4992469  /  4992469
// [00:02:44] Compute merges                 ██████████████████████████████████████████ 50022    /    50022

// tokenizer_gpt2_cx-cs_00000-00001_50k+no-eot.json
// [00:53:59] Pre-processing files (4551 Mo) ██████████████████████████████████████████                100%
// [00:01:34] Tokenize words                 ██████████████████████████████████████████ 7411386  /  7411386
// [00:41:21] Count pairs                    ██████████████████████████████████████████ 7411386  /  7411386
// [00:09:31] Compute merges                 ██████████████████████████████████████████ 50027    /    50027

// tokenizer_gpt2+ts_cx-cs_00000-00001_50k+no-eot.json
// [02:02:46] Pre-processing files (4551 Mo) ██████████████████████████████████████████                100%
// [00:02:31] Tokenize words                 ██████████████████████████████████████████ 7389790  /  7389790
// [01:25:43] Count pairs                    ██████████████████████████████████████████ 7389790  /  7389790
// [00:06:08] Compute merges                 ██████████████████████████████████████████ 50027    /    50027