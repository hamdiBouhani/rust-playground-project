use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

// Bloom filter data structure
struct BloomFilter {
    bitset: Vec<bool>,
    size: usize,
    hash_count: usize,
}

impl BloomFilter {
    fn new(size: usize, hash_count: usize) -> BloomFilter {
        BloomFilter {
            bitset: vec![false; size],
            size: size,
            hash_count: hash_count,
        }
    }

    fn insert(&mut self, item: &str) {
        // repeating this process for each hash function, the insert method ensures that multiple bits in the bitset are set for the given item,
        // reducing the chance of false positives when checking for membership.
        for i in 0..self.hash_count {
            let index = self.hash(item, i);
            self.bitset[index] = true;
        }
    }

    fn hash(&self, item: &str, i: usize) -> usize {
        let mut hasher = DefaultHasher::new();
        //The salt is appended to the item to create a unique input for the hash function.
        let salt = i.to_string();
        let salted_item = format!("{}{}", item, salt);
        salted_item.hash(&mut hasher); //This updates the internal state of the hasher with the hash of the salted_item.
        let hash = hasher.finish();
        hash as usize % self.size
    }

    fn contains(&self, item: &str) -> bool {
        for i in 0..self.hash_count {
            let index = self.hash(item, i);
            if !self.bitset[index] {
                return false;
            }
        }
        true
    }
}

fn main() {
    let mut filter = BloomFilter::new(1000, 3);

    filter.insert("apple");
    filter.insert("banana");
    filter.insert("orange");

    println!("Contains 'apple': {}", filter.contains("apple")); // true
    println!("Contains 'grape': {}", filter.contains("grape")); // false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_bloom_filter() {
        let mut filter = BloomFilter::new(1000, 3);

        filter.insert("apple");
        filter.insert("banana");
        filter.insert("orange");

        assert_eq!(filter.contains("apple"), true);
        assert_eq!(filter.contains("grape"), false);
    }
}
