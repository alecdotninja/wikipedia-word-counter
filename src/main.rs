mod histogram;
mod word_counter;
mod wikitext;

use word_counter::ParWordCounter;
use wikitext::wikitext_in_mediawiki_dump;

const HISTOGRAM_SIZE: usize = 100_000;

pub fn main() {
    let par_word_counter = ParWordCounter::new(HISTOGRAM_SIZE);

    for wikitext in wikitext_in_mediawiki_dump(buffered_stdin()) {
        par_word_counter.push_wikitext(wikitext);        
    }

    let mut csv_writer = csv::Writer::from_writer(buffered_stdin());

    for (word, count) in par_word_counter.into_histogram() {
        csv_writer.write_record(&[word, count.to_string()]).unwrap();
    }
}

use std::io::{
    stdin,
    stdout,
    Stdin,
    Stdout,
    BufReader,
    BufWriter,
};

pub fn buffered_stdin() -> BufReader<Stdin> {
    BufReader::new(stdin())
}

pub fn buffered_stdout() -> BufWriter<Stdout> {
    BufWriter::new(stdout())
}