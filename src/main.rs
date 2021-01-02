#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use structopt::{self, StructOpt};

mod pinyin;

use std::collections::HashSet;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(StructOpt, Debug)]
#[structopt()]
enum Opt {
    /// Reads a word list as CSV and replaces numbers with tone marks for Pinyin text
    #[structopt(name = "create-word-list")]
    CreateWordList {
        /// File to process
        #[structopt(name = "INPUT", parse(from_os_str))]
        input: PathBuf,

        /// Output file
        #[structopt(name = "OUTPUT", parse(from_os_str))]
        output: PathBuf,
    },
    /// Replaces numbered sillables with tone marks
    #[structopt(name = "numbers-to-tone-marks")]
    NumbersToToneMarks {
        /// File to process
        #[structopt(name = "INPUT", parse(from_os_str))]
        input: PathBuf,

        /// Output file
        #[structopt(name = "OUTPUT", parse(from_os_str))]
        output: PathBuf,
    },
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
    match Opt::from_args() {
        Opt::CreateWordList { input, output } => {
            create_word_list(&input, &output)?;
        }
        Opt::NumbersToToneMarks { input, output } => {
            numbers_to_marks(&input, &output)?;
        }
    };
    Ok(())
}

fn create_word_list(input: &Path, output: &Path) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new().delimiter(b',').from_path(input)?;

    let mut wtr = csv::WriterBuilder::new()
        .has_headers(false)
        .from_path(output)?;

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

fn numbers_to_marks(input: &Path, output: &Path) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(input)?;
    let transformed = pinyin::numbers_to_marks(&contents);
    print!("{}", &transformed);
    let mut out = File::create(output)?;
    write!(out, "{}", transformed)?;
    Ok(())
}
