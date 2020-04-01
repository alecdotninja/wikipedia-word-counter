use std::thread::{ self, JoinHandle };
use crossbeam_channel::{ bounded, Sender, Receiver };

use super::histogram::Histogram;
use super::wikitext::words_in_wikitext;

const THREAD_COUNT: usize = 8;
const MAX_QUEUE_SIZE: usize = 256;

pub struct ParWordCounter {
    size: usize,
    sender: Sender<String>,
    join_handles: Vec<JoinHandle<WordCounter>>,
}

impl ParWordCounter {
    pub fn new(size: usize) -> Self {
        let (sender, receiver) = bounded(MAX_QUEUE_SIZE);

        Self {
            size,
            sender,
            join_handles: Self::par_recv_into_histogram(size, receiver),
        }
    }

    fn par_recv_into_histogram(size: usize, receiver: Receiver<String>) -> Vec<JoinHandle<WordCounter>> {
        (0..THREAD_COUNT)
            .map(|_| {
                let receiver = receiver.clone();

                thread::spawn(move || {
                    Self::recv_into_histogram(size, receiver)
                })
            })
            .collect()
    }

    fn recv_into_histogram(size: usize, receiver: Receiver<String>) -> WordCounter {
        let mut word_counter = WordCounter::new(size);

        while let Ok(wikitext) = receiver.recv() {
            word_counter.push_wikitext(wikitext);
        }

        word_counter
    }

    pub fn push_wikitext(&self, wikitext: String) {
        self.sender.send(wikitext).unwrap();
    }

    pub fn into_histogram(self) -> Histogram<String> {
        let Self { size, sender, join_handles } = self;
        
        drop(sender);

        join_handles
            .into_iter()
            .map(|thread| thread.join().unwrap().into_histogram())
            .fold(
                Histogram::new(size),
                |accumulator, element| accumulator.merge(element),
            )
    }
}

pub struct WordCounter {
    histogram: Histogram<String>,
}

impl WordCounter {
    pub fn new(size: usize) -> Self {
        Self {
            histogram: Histogram::new(size),
        }
    }

    pub fn push_wikitext(&mut self, wikitext: String) {
        for word in words_in_wikitext(&wikitext) {
            self.histogram.add(word.to_uppercase());
        }
    }

    pub fn into_histogram(self) -> Histogram<String> {
        self.histogram
    }
}
