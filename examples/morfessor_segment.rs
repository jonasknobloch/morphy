use mbpe::pre_tokenizers::segmenter::Segmenter;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Error, Write};

fn main() -> Result<(), Error> {
    let model = match mbpe::morfessor::morfessor::decode_model("scripts/semisup_model.proto") {
        Ok(model) => model,
        Err(_) => {
            return Err(Error::from(Error::new(
                std::io::ErrorKind::Other,
                "Failed to decode model",
            )))
        }
    };

    let config = mbpe::pre_tokenizers::morfessor::MorfessorConfig {
        viterbi_smoothing: 0.0,
        viterbi_max_len: 30,
        rejection_threshold: 0.0,
        reject_single_char_ngrams: 2,
    };

    let segmenter = mbpe::pre_tokenizers::morfessor::Morfessor {
        config: config,
        morfessor: model,
    };

    let reader = BufReader::new(File::open("data/goldstd_trainset.segmentation.eng")?);
    let mut writer = BufWriter::new(File::create("s0_30_x_2.eng")?);

    if let lines = reader.lines() {
        for line in lines.flatten() {
            let compound = line.split("\t").collect::<Vec<&str>>()[0];

            let offsets = segmenter.segment(&compound);

            let mut parts = vec![];

            for (start, end) in offsets {
                parts.push(&compound[start..end])
            }

            let out = compound.to_string() + "\t" + &parts.join(" ") + "\n";

            writer.write(out.as_bytes())?;
        }
    }

    writer.flush()
}
