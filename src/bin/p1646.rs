fn main() {
    let mut lines = std::io::stdin().lines();
    let (n, q) = {
        let first_line = lines.next().unwrap().unwrap();
        let mut parts = first_line.split_whitespace();
        (
            parts.next().unwrap().parse::<usize>().unwrap(),
            parts.next().unwrap().parse::<usize>().unwrap(),
        )
    };
    let nums = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let mut cummulative = vec![0; n + 1];
    for i in 0..n {
        cummulative[i + 1] = cummulative[i] + nums[i];
    }
    for _ in 0..q {
        let query = lines.next().unwrap().unwrap();
        let mut parts = query.split_whitespace();
        let l = parts.next().unwrap().parse::<usize>().unwrap();
        let r = parts.next().unwrap().parse::<usize>().unwrap();
        let sum = cummulative[r] - cummulative[l - 1];
        println!("{sum}");
    }
}
