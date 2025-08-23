const MOD: usize = 1_000_000_007;

fn main() {
    let mut lines = std::io::stdin().lines();
    let n = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<usize>()
        .unwrap();
    let mut grid = vec![];
    for _ in 0..n {
        grid.push(lines.next().unwrap().unwrap().chars().collect::<Vec<_>>());
    }

    let mut dp = vec![vec![0usize; n]; n];

    for i in 0..n {
        for j in 0..n {
            if grid[i][j] == '*' {
                dp[i][j] = 0;
            } else if i == 0 && j == 0 {
                dp[i][j] = 1;
            } else {
                if i > 0 {
                    dp[i][j] = (dp[i][j] + dp[i - 1][j]) % MOD;
                }
                if j > 0 {
                    dp[i][j] = (dp[i][j] + dp[i][j - 1]) % MOD;
                }
            }
        }
    }

    println!("{}", dp[n - 1][n - 1]);
}
