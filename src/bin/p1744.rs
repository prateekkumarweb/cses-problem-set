fn main() {
    let mut lines = std::io::stdin().lines();
    let (a, b) = {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        (
            parts.next().unwrap().parse::<usize>().unwrap(),
            parts.next().unwrap().parse::<usize>().unwrap(),
        )
    };

    let mut dp = vec![vec![0; b + 1]; a + 1];

    for i in 0..=a {
        for j in 0..=b {
            if i == j {
                dp[i][j] = 0;
            } else {
                dp[i][j] = usize::MAX;
                for k in 1..i {
                    dp[i][j] = dp[i][j].min(dp[k][j] + dp[i - k][j] + 1);
                }
                for k in 1..j {
                    dp[i][j] = dp[i][j].min(dp[i][k] + dp[i][j - k] + 1);
                }
            }
        }
    }

    println!("{}", dp[a][b]);
}
