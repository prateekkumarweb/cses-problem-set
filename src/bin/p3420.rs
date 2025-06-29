use std::collections::HashMap;

fn main() {
    let mut lines = std::io::stdin().lines();
    let _ = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
    let nums = lines
        .next()
        .unwrap()
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let mut count = 0;
    let mut index_map = HashMap::new();
    let mut start = 0;
    for (end, n) in nums.iter().enumerate() {
        let index = index_map.get(n);
        if let Some(&i) = index {
            if i >= start {
                start = i + 1;
            }
        }
        index_map.insert(*n, end);
        count += end - start + 1;
    }
    println!("{count}");
}
