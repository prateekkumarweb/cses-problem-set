fn main() {
    let mut lines = std::io::stdin().lines();
    let n = lines.next().unwrap().unwrap().parse::<i64>().unwrap();
    let nums = lines
        .next()
        .unwrap()
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let indices_map = nums
        .iter()
        .enumerate()
        .map(|(i, &num)| (num, i + 1))
        .collect::<std::collections::HashMap<_, _>>();
    let mut result = 1;
    for i in 1i64..n {
        if indices_map[&(i + 1)] < indices_map[&i] {
            result += 1;
        }
    }
    println!("{}", result);
}
