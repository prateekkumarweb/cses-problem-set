fn main() {
    let mut lines = std::io::stdin().lines();
    let str1 = lines.next().unwrap().unwrap();
    let str2 = lines.next().unwrap().unwrap();

    let n1 = str1.len();
    let n2 = str2.len();
    let s1 = str1.as_bytes();
    let s2 = str2.as_bytes();

    let mut dp = vec![vec![0; n2 + 1]; n1 + 1];

    #[allow(clippy::needless_range_loop)]
    for i in 0..=n1 {
        dp[i][0] = i;
    }
    for j in 0..=n2 {
        dp[0][j] = j;
    }

    for i in 1..=n1 {
        for j in 1..=n2 {
            if s1[i - 1] == s2[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = 1 + dp[i - 1][j - 1].min(dp[i][j - 1].min(dp[i - 1][j]));
            }
        }
    }

    println!("{}", dp[n1][n2]);
}
