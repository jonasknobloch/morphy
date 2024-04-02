use std::io::Error;
use morphy::morfessor::morfessor;

fn main() -> Result<(), Error>{
    let model = match morfessor::decode_model("scripts/unsup_model.proto") {
        Ok(model) => model,
        Err(_) => return Err(Error::from(Error::new(std::io::ErrorKind::Other, "Failed to decode model"))),
    };

    let corpus = vec![
        "unsupervised",
        "incredible",
        "impressive",
        "McLaren",
        "unfoobared",
        "YOOOO",
        "WHATUP",
        "1317281738",
    ];

    for compound in corpus {
        let (segments, score) = morfessor::viterbi_segment(&model, &compound, 0.0, 30);

        if score < 50.0 {
            println!("Segments: {:?}, Score: {}", segments, score);
        } else {
            println!("Segements: {:?}, Score: {}", vec![compound], score);
        }
    }

    Ok(())
}