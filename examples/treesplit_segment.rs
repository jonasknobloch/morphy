use mbpe::pre_tokenizers::segmenter::Segmenter;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Error, Write};

fn main() -> Result<(), Error> {
    let mut unimorph = mbpe::unimorph::unimorph::Unimorph::new();

    unimorph.init("/Users/jonas/.unimorph/eng/eng")?;

    let segmenter = mbpe::pre_tokenizers::tree_split::TreeSplit { unimorph };

    let reader = BufReader::new(File::open("data/goldstd_trainset.segmentation.eng")?);
    let mut writer = BufWriter::new(File::create("tree_split.eng")?);

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
