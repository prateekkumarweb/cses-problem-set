use std::collections::HashMap;

const MOD: i64 = 1_000_000_007;

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
    let mut count_map = HashMap::new();
    for &n in nums.iter() {
        let current_count = count_map.get(&n).cloned().unwrap_or(0);
        if current_count == 0 {
            count = count * 2 + 1;
            count %= MOD;
        } else {
            let prod = count_map
                .iter()
                .filter(|(&k, _)| k != n)
                .map(|(_, &v)| v + 1)
                .fold(1, |a, b| (a * b) % MOD);
            count = (count + prod) % MOD;
        }
        count_map.entry(n).and_modify(|e| *e += 1).or_insert(1i64);
    }
    println!("{count}");
}
