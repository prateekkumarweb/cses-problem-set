use std::collections::HashMap;

fn main() {
    let n = std::io::stdin()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let mut memo = HashMap::new();
    println!("{}", count(n, &mut memo));
}

fn count(n: usize, memo: &mut HashMap<usize, usize>) -> usize {
    if n == 0 {
        return 0;
    }
    if n <= 9 {
        return 1;
    }
    if let Some(&result) = memo.get(&n) {
        return result;
    }
    let mut min = usize::MAX;
    for c in n.to_string().chars() {
        let digit = c.to_digit(10).unwrap();
        if digit > 0 {
            min = min.min(count(n - digit as usize, memo) + 1);
        }
    }
    memo.insert(n, min);
    min
}
