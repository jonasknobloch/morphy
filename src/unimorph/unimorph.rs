use csv::ReaderBuilder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Unimorph {
    language: String,
    inflect: HashMap<String, HashMap<String, ()>>,
    analyze: HashMap<String, HashMap<String, ()>>,
    features: HashMap<String, HashMap<String, Vec<String>>>,
}

impl Unimorph {
    pub fn new() -> Self {
        Unimorph {
            language: String::new(),
            inflect: HashMap::new(),
            analyze: HashMap::new(),
            features: HashMap::new(),
        }
    }

    pub fn init(&mut self, dict: &str) -> Result<(), std::io::Error> {
        let file = File::open(dict)?;

        let mut rdr = ReaderBuilder::new().delimiter(b'\t').from_reader(file);

        for result in rdr.records() {
            let record = result?;

            let lemma = record[0].to_owned();
            let form = record[1].to_string();
            let features = record[2].to_string();

            self.inflect
                .entry(lemma.clone())
                .or_insert_with(HashMap::new)
                .insert(form.clone(), ());
            self.analyze
                .entry(form.clone())
                .or_insert_with(HashMap::new)
                .insert(lemma.clone(), ());
            self.features
                .entry(lemma.clone())
                .or_insert_with(HashMap::new)
                .entry(form.clone())
                .or_insert_with(Vec::new)
                .push(features);
        }

        Ok(())
    }

    pub fn inflect(&self, lemma: &str, features: &str) -> Vec<String> {
        let forms = match self.inflect.get(lemma) {
            Some(forms) => forms,
            None => return Vec::new(),
        };

        let mut result = Vec::new();

        for form in forms.keys() {
            if features.is_empty() {
                result.push(form.clone());

                continue;
            }

            let fss = self.features.get(lemma).unwrap().get(form).unwrap();

            for fs in fss {
                if fs == features {
                    result.push(form.clone());
                }
            }
        }

        return result;
    }

    pub fn analyze(&self, form: &str) -> Vec<String> {
        let lemmas = match self.analyze.get(form) {
            Some(lemmas) => lemmas,
            None => return Vec::new(),
        };

        let mut result = Vec::new();

        for lemma in lemmas.keys() {
            result.push(lemma.clone());
        }

        return result;
    }

    pub fn features(&self, lemma: &str, form: &str) -> Vec<String> {
        return match self.features.get(lemma) {
            Some(forms) => match forms.get(form) {
                Some(features) => features.clone(),
                None => Vec::new(),
            },
            None => Vec::new(),
        };
    }
}

#[test]
fn test_inflect() {
    let mut unimorph = Unimorph::new();

    unimorph.init("ces_afghansky.tsv").unwrap();

    let mut forms = unimorph.inflect("afghánský", "");

    forms.sort();

    assert_eq!(
        forms,
        vec![
            "afghánskou",
            "afghánská",
            "afghánské",
            "afghánského",
            "afghánském",
            "afghánskému",
            "afghánský",
            "afghánských",
            "afghánským",
            "afghánskými",
            "afghánští"
        ]
    );
}

#[test]
fn test_inflect_with_features() {
    let mut unimporph = Unimorph::new();

    unimporph.init("ces_afghansky.tsv").unwrap();

    let mut forms = unimporph.inflect("afghánský", "ADJ;GEN;FEM;SG");

    forms.sort();

    assert_eq!(forms, vec!["afghánské"])
}

#[test]
fn test_analyze() {
    let mut unimorph = Unimorph::new();

    unimorph.init("ces_afghansky.tsv").unwrap();

    let mut lemmas = unimorph.analyze("afghánskou");

    lemmas.sort();

    assert_eq!(lemmas, vec!["afghánský"]);
}

#[test]
fn test_features() {
    let mut unimorph = Unimorph::new();

    unimorph.init("ces_afghansky.tsv").unwrap();

    let mut features = unimorph.features("afghánský", "afghánskou");

    features.sort();

    assert_eq!(features, vec!["ADJ;ACC;FEM;SG"]);
}
