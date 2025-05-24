fn main() {
    let mut lines = std::io::stdin().lines();
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut sum = n * (n + 1) / 2;
    let line = lines.next().unwrap().unwrap();
    let nums = line.split_ascii_whitespace();
    for x in nums {
        let x: usize = x.parse().unwrap();
        sum -= x;
    }
    println!("{}", sum);
}
