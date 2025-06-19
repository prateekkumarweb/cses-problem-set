use std::collections::HashMap;

fn main() {
    let mut lines = std::io::stdin().lines();
    let _ = lines.next().unwrap().unwrap().parse::<i64>().unwrap();
    let nums = lines
        .next()
        .unwrap()
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let mut indices_map = HashMap::new();
    let mut start = 0;
    let mut max_len = 0;
    for (i, &num) in nums.iter().enumerate() {
        if indices_map.contains_key(&num) && indices_map[&num] >= start {
            start = indices_map[&num] + 1;
        }
        indices_map.insert(num, i);
        let end = i;
        max_len = max_len.max(end - start + 1);
    }
    println!("{}", max_len);
}
