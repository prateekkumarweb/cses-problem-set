use std::collections::HashSet;

fn main() {
    let mut lines = std::io::stdin().lines();
    let _ = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
    let nums = lines
        .next()
        .unwrap()
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<HashSet<_>>();
    println!("{}", nums.len());
}
