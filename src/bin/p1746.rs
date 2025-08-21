fn main() {
    let mut lines = std::io::stdin().lines();
    let (n, m) = {
        let line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        (
            iter.next().unwrap().parse::<usize>().unwrap(),
            iter.next().unwrap().parse::<usize>().unwrap(),
        )
    };
    let nums = {
        let line = lines.next().unwrap().unwrap();
        line.split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
    };

    const MOD: usize = 1_000_000_007;
    // dp[i][v] = number of ways to fill the array up to position i with value v at position i.
    let mut dp = vec![vec![0; m + 2]; n];

    if nums[0] == 0 {
        for v in 1..=m {
            dp[0][v] = 1;
        }
    } else {
        dp[0][nums[0]] = 1;
    }

    for i in 1..n {
        for v in 1..=m {
            if nums[i] == 0 || nums[i] == v {
                dp[i][v] = (dp[i - 1][v] + dp[i - 1][v - 1] + dp[i - 1][v + 1]) % MOD;
            }
        }
    }

    let result = (1..=m).fold(0, |acc, v| (acc + dp[n - 1][v]) % MOD);
    println!("{}", result);
}
