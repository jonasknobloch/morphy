use tokenizers::normalizers::{Prepend, Sequence, Strip, NFC};
use tokenizers::{NormalizedString, Normalizer, NormalizerWrapper};

pub fn main() {
    let normalizer = Sequence::new(vec![
        NormalizerWrapper::from(Strip::new(true, true)),
        NormalizerWrapper::from(Prepend::new(" ".to_string())), // same as BPE add_prefix_space
        NormalizerWrapper::from(NFC::default()),
    ]);

    let mut normalized = NormalizedString::from("   That's some impressive retrofitting!  ");

    let _ = normalizer.normalize(&mut normalized);

    println!("{:?}", normalized.get());
}
