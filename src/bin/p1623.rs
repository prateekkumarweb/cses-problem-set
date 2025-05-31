fn main() {
    let mut lines = std::io::stdin().lines();
    let _ = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
    let input = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let result = solve(&input, 0, 0);
    println!("{}", result);
}

fn solve(input: &[usize], group1: usize, group2: usize) -> usize {
    if input.is_empty() {
        return group1.abs_diff(group2);
    }
    let (first, rest) = input.split_first().unwrap();
    let a = solve(rest, group1 + first, group2);
    let b = solve(rest, group1, group2 + first);
    a.min(b)
}
