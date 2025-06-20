use std::collections::BTreeSet;

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
    let mut towers = BTreeSet::new();
    for (index, num) in nums.into_iter().enumerate() {
        let smallest_greater = towers.range((num + 1, 0)..).next();
        if let Some(&next) = smallest_greater {
            towers.remove(&next);
            towers.insert((num, index));
        } else {
            towers.insert((num, index));
        }
    }
    let result = towers.len();
    println!("{}", result);
}
