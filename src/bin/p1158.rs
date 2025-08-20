fn main() {
    let mut lines = std::io::stdin().lines();
    let (n, x) = {
        let line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        (
            iter.next().unwrap().parse::<usize>().unwrap(),
            iter.next().unwrap().parse::<usize>().unwrap(),
        )
    };
    let prices = {
        let line = lines.next().unwrap().unwrap();
        line.split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
    };
    let pages = {
        let line = lines.next().unwrap().unwrap();
        line.split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
    };

    let mut dp = vec![0; x + 1];
    for i in 0..n {
        for j in (prices[i]..=x).rev() {
            dp[j] = dp[j].max(dp[j - prices[i]] + pages[i]);
        }
    }
    println!("{}", dp[x]);
}
