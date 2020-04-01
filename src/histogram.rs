use std::collections::HashMap;
use std::hash::Hash;
use std::iter::IntoIterator;

const PRUNE_OVERFILL_THRESHOLD: usize = 8;

pub type Count = u32;
pub type CountMap<T> = HashMap<T, Count>;
pub type CountIterator<T> = <CountMap<T> as IntoIterator>::IntoIter;
pub type CountIteratorItem<T> = <CountMap<T> as IntoIterator>::Item;

pub struct Histogram<T: Hash + Eq> {
    size: usize,
    count_map: CountMap<T>,
}

impl<T: Hash + Eq> Histogram<T> {
    pub fn new(size: usize) -> Self {
        if size == 0 {
            panic!("size must be non-zero");
        }

        Self {
            size,
            count_map: HashMap::new(),
        }
    }

    pub fn count_mut(&mut self, key: T) -> &mut Count {
        if self.count_map.len() >= self.size * PRUNE_OVERFILL_THRESHOLD {
            self.prune();
        }

        self.count_map.entry(key).or_insert(0)
    }

    pub fn add(&mut self, key: T) {
        *self.count_mut(key) += 1;
    }

    pub fn merge(mut self, other: Self) -> Self {
        for (key, count) in other {
            *self.count_mut(key) += count;
        }

        self
    }

    fn prune(&mut self) {
        let len = self.count_map.len();

        if len < self.size {
            return;
        }

        let mut counts = 
            self.count_map
            .values()
            .map(|count| *count)
            .collect::<Vec<Count>>();

        counts.sort_by_key(|count| std::cmp::Reverse(*count));
        counts.truncate(self.size);

        let cuttoff = counts[self.size - 1];

        let mut remaining =
            counts
            .into_iter()
            .filter(|count| *count == cuttoff)
            .count();

        self.count_map.retain(|_word, count| {
            if *count > cuttoff {
                return true;
            }

            if *count == cuttoff && remaining > 0 {
                remaining -= 1;

                return true;
            }

            false
        });
    }
}

impl<T: Hash + Eq> IntoIterator for Histogram<T> {
    type Item = CountIteratorItem<T>;
    type IntoIter = CountIterator<T>;

    fn into_iter(mut self) -> Self::IntoIter {
        self.prune();

        self.count_map.into_iter()
    }
}
