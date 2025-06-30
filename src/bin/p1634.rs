fn main() {
    let mut lines = std::io::stdin().lines();
    let (_, x) = {
        let first_line = lines.next().unwrap().unwrap();
        let mut parts = first_line.split_whitespace();
        (
            parts.next().unwrap().parse::<usize>().unwrap(),
            parts.next().unwrap().parse::<usize>().unwrap(),
        )
    };
    let mut coins = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    coins.sort_unstable();

    let mut memo = vec![None; x + 1];
    memo[0] = Some(0);

    for x in 1..=x {
        for &coin in &coins {
            if coin <= x {
                if let Some(prev_count) = memo[x - coin] {
                    let new_count = prev_count + 1;
                    memo[x] = match memo[x] {
                        Some(current_count) => Some(current_count.min(new_count)),
                        None => Some(new_count),
                    };
                }
            } else {
                break;
            }
        }
    }

    match memo[x] {
        Some(count) => println!("{count}"),
        None => println!("-1"),
    }
}
