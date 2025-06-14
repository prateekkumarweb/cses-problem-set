fn main() {
    let mut lines = std::io::stdin().lines();
    let _ = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
    let mut nums = lines
        .next()
        .unwrap()
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    nums.sort_unstable();
    let median = nums[nums.len() / 2];
    let sum = nums.iter().map(|&x| (x - median).abs()).sum::<i64>();
    println!("{}", sum);
}
