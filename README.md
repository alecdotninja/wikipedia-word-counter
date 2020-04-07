# Wikipedia Word Counter

Wikipedia Word Counter is a small utility to get the top 100,000 words in a
[Wikipedia dump file](https://dumps.wikimedia.org/).


## Usage

  1. Download the latest version of [the Rust toolchain](https://rustup.rs/) and compile the program _in release mode_
  (`cargo build --release`). You may wish to tweak some of the static variables in the source code like
  [`HISTOGRAM_SIZE`](src/main.rs#L8) (the number of top words to keep), [`THREAD_COUNT`](src/word_counter.rs#L7) (ideally
  the number of logical cores that your computer has), and [`PRUNE_OVERFILL_THRESHOLD`](src/histogram.rs#L5) (the amount of
  extra memory the program can use to defer work while counting).

  1. Download the latest full Wikipedia dump from the
  [Wikimeida Downloads page](https://dumps.wikimedia.org/backup-index.html). You probably want to find the latest `enwiki`
  dump. At the time of this writing, the dump that includes the text of the current version of all articles ends with
  `pages-meta-current`.

  1. Run the dump through the counter (the dump goes on stdin and a CSV of the top words is written to stdout):
  
  ```bash
  $ bzcat enwiki-20200301-pages-meta-current.xml.bz2 | target/release/wikipedia-word-counter > top-words.csv
  ```
     
  On my machine with the dump from March 1st, 2020, this takes approximately an hour to run and uses around 1Gb of RAM.
  You can get some sense of progress with the [`pv` utility](https://linux.die.net/man/1/pv):
     
  ```bash
  $ pv enwiki-20200301-pages-meta-current.xml.bz2 | bzcat | target/release/wikipedia-word-counter > top-words.csv
  ```


## Contributing

Bug reports and pull requests are welcome on [GitHub](https://github.com/alecdotninja/wikipeida-word-counter).


## License

The gem is available as open source under the terms of the [MIT License](http://opensource.org/licenses/MIT).
