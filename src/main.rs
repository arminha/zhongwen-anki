#![forbid(unsafe_code)]

#[macro_use]
extern crate lazy_static;
extern crate csv;
extern crate unicode_segmentation;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate structopt;

mod pinyin;

use std::collections::HashSet;
use std::error::Error;
use std::path::PathBuf;

use structopt::StructOpt;

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

fn main() -> Result<(), Box<Error>> {
    let opt = Opt::from_args();

    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b',')
        .from_path(opt.input)?;

    let mut wtr = csv::WriterBuilder::new()
        .has_headers(false)
        .from_path(opt.output)?;

    let mut words = HashSet::new();
    for result in rdr.deserialize() {
        let mut line: Line = result?;
        let marks = pinyin::numbers_to_marks(&line.pinyin);
        line.pinyin = marks;
        if !words.insert(line.mandarin.to_owned()) {
            eprintln!("Duplicate word: {}", &line.mandarin);
        }
        wtr.serialize(&line)?;
    }
    wtr.flush()?;
    Ok(())
}
