use std::collections::{BTreeMap, BTreeSet};

fn main() {
    let mut lines = std::io::stdin().lines();
    let (x, _) = {
        let first_line = lines.next().unwrap().unwrap();
        let mut parts = first_line.split_ascii_whitespace();
        (
            parts.next().unwrap().parse::<i64>().unwrap(),
            parts.next().unwrap().parse::<usize>().unwrap(),
        )
    };
    let line = lines.next().unwrap().unwrap();
    let nums = line
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().unwrap());
    let mut pairs = BTreeSet::new();
    let mut lengths = BTreeMap::new();
    pairs.insert((0, x));
    lengths.insert(x, 1);
    for num in nums {
        let upper_bound = *pairs.range(..(num, 0)).last().unwrap();
        let len = upper_bound.1 - upper_bound.0;
        pairs.remove(&upper_bound);
        let len_c = *lengths.get(&len).unwrap();
        if len_c == 1 {
            lengths.remove(&len);
        } else {
            lengths.insert(len, len_c - 1);
        }
        let p1 = (upper_bound.0, num);
        let p2 = (num, upper_bound.1);
        pairs.insert(p1);
        pairs.insert(p2);
        *lengths.entry(num - upper_bound.0).or_insert(0) += 1;
        *lengths.entry(upper_bound.1 - num).or_insert(0) += 1;
        let max = lengths.last_key_value().unwrap().0;
        print!("{} ", max);
    }
    println!();
}
