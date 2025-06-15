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
    let mut smallest_sum = 1;
    for num in nums {
        if num > smallest_sum {
            break;
        }
        smallest_sum += num;
    }
    println!("{}", smallest_sum);
}
