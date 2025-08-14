fn main() {
    let mut lines = std::io::stdin().lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut first_iter = first_line.split_whitespace();
    let _n: usize = first_iter.next().unwrap().parse().unwrap();
    let x: usize = first_iter.next().unwrap().parse().unwrap();

    let second_line = lines.next().unwrap().unwrap();
    let coins: Vec<usize> = second_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut dp = vec![0usize; x + 1];
    dp[0] = 1;

    for &coin in &coins {
        for i in coin..=x {
            dp[i] = (dp[i] + dp[i - coin]) % 1_000_000_007;
        }
    }

    println!("{}", dp[x]);
}
