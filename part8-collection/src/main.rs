use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    // let mut primes = Vec::new();
    // primes.push(2);
    // primes.push(3);
    // primes.push(5);
    // println!("{:?}", primes)

    // let mut primes = vec![1, 2, 3, 5];
    // primes.remove(0);
    // println!("{:?}", primes);
    // println!("The second prime number is {}", primes[1]);

    let primes = vec![2, 3, 5];
    // let mut i = 0;
    // while i < primes.len() {
    //     println!("Prime #{}: {}", i + 1, primes[i]);
    //     i += 1;
    // }

    // let primes_iter = primes.iter();
    // for p in primes_iter {
    //     println!("Prime: {}", p);
    // }

    // let mut primes_iter = primes.iter();
    // while let Some(p) = primes_iter.next() {
    //     println!("Prime: {}", p);
    // }

    let mut primes_iter = primes.iter().enumerate();
    while let Some((i, p)) = primes_iter.next() {
        println!("Prime: #{}: {}", i + 1, p);
    }

    println!("--------");

    for (i, p) in primes.iter().enumerate() {
        println!("Prime: #{}: {}", i + 1, p);
    }

    println!("--------");

    for p in &primes {
        println!("Prime: {}", p);
    }

    println!("--------  VecDeque  --------");

    let mut primes2 = VecDeque::new();
    primes2.push_back(3);
    primes2.push_back(5);
    primes2.push_back(7);
    primes2.push_front(2);

    for p in &mut primes2 {
        println!("Prime: {}", p);
    }

    println!("--------  HashMap  --------");
    let mut grid = HashMap::new();
    grid.insert((2, 3), "tree");
    grid.insert((4, 7), "rock");
    *grid.entry((-3, 1)).or_insert("emtpy") = "bird";
    grid.remove(&(4, 7));

    let coords = (2, 3);
    // let cell = grid.get(&coords).unwrap_or(&"empty");
    if let Some(cell) = grid.get(&coords) {
        println!("{} at  {:?}", cell, &coords);
    } else {
        println!("Empty at {:?}", &coords);
    }

    for (key, value) in &grid {
        println!("({}, {}): {}", key.0, key.1, value);
        println!("{} at  {:?}", value, key);
    }

    println!("--------  BTreeMap  --------");

    let mut grid1 = BTreeMap::new();
    grid1.insert((2, 3), "tree");
    grid1.insert((4, 7), "rock");
    *grid1.entry((-3, 1)).or_insert("emtpy") = "bird";
    grid1.remove(&(4, 7));

    let coords = (2, 3);
    // let cell = grid1.get(&coords).unwrap_or(&"empty");
    if let Some(cell) = grid1.get(&coords) {
        println!("{} at  {:?}", cell, &coords);
    } else {
        println!("Empty at {:?}", &coords);
    }

    for (key, value) in &grid1 {
        println!("({}, {}): {}", key.0, key.1, value);
        println!("{} at  {:?}", value, key);
    }

    println!("--------  Set  --------");

    let mut set = HashSet::new();
    set.insert(0);
    for i in 1..=10 {
        set.insert(i);
    }

    for n in &set {
        println!(" Set  Value {}", n);
    }

    for (i, n) in set.iter().enumerate() {
        println!(" Set index {}: Value {}", i, n);
    }

    println!("--------  BinaryHeap  --------");
    let mut primes4 = BinaryHeap::new();
    primes4.push(2);
    primes4.push(3);
    primes4.push(5);
    println!("{:?}", primes4);

    for p in &primes4 {
        // not sort in for
        println!("Prime: {}", p);
    }

    while let Some(p) = primes4.pop() {
        // sort in pop
        println!("Prime: {}", p);
    }
}
