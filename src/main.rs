#![forbid(unsafe_code)]

use csv;
use serde_derive::{Deserialize, Serialize};
use structopt::{self, StructOpt};

mod pinyin;

use std::collections::HashSet;
use std::error::Error;
use std::path::PathBuf;

#[derive(StructOpt, Debug)]
#[structopt()]
struct Opt {
    /// File to process
    #[structopt(name = "INPUT", parse(from_os_str))]
    input: PathBuf,

    /// Output file
    #[structopt(name = "OUTPUT", parse(from_os_str))]
    output: PathBuf,
}

#[derive(Debug, Deserialize, Serialize)]
struct Line {
    #[serde(rename = "Mandarin")]
    mandarin: String,
    #[serde(rename = "Pinyin")]
    pinyin: String,
    #[serde(rename = "German")]
    german: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();

    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b',')
        .from_path(opt.input)?;

    let mut wtr = csv::WriterBuilder::new()
        .has_headers(false)
        .from_path(opt.output)?;

    let mut words = HashSet::new();
    let mut count = 0;
    for result in rdr.deserialize() {
        count += 1;
        let mut line: Line = result?;
        let marks = pinyin::numbers_to_marks(&line.pinyin);
        line.pinyin = marks;
        if !words.insert(line.mandarin.to_owned()) {
            eprintln!("Duplicate word: {}", &line.mandarin);
        }
        wtr.serialize(&line)?;
    }
    eprintln!("Processed {} words", count);
    wtr.flush()?;
    Ok(())
}
