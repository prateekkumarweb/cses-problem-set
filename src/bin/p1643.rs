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
    let mut max_sum = nums[0];
    let mut running_sum = nums[0];
    for num in nums.iter().skip(1) {
        if running_sum < 0 {
            running_sum = *num;
        } else {
            running_sum += num;
        }
        max_sum = max_sum.max(running_sum);
    }
    println!("{}", max_sum);
}
