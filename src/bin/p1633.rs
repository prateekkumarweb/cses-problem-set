use std::collections::HashMap;

const MOD: usize = 1_000_000_007;

fn main() {
    let n = std::io::stdin()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let mut memo = HashMap::new();
    println!("{}", count(n, &mut memo));
}

fn count(n: usize, memo: &mut HashMap<usize, usize>) -> usize {
    if n == 0 {
        return 1;
    }
    if let Some(&result) = memo.get(&n) {
        return result;
    }
    let mut total = 0;
    for i in 1..=n.min(6) {
        total += count(n - i, memo);
    }
    total %= MOD;
    memo.insert(n, total);
    total
}
