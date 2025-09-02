fn main() {
    let mut lines = std::io::stdin().lines();
    let (n, x) = {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        (
            parts.next().unwrap().parse::<usize>().unwrap(),
            parts.next().unwrap().parse::<i64>().unwrap(),
        )
    };
    let numbers = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let arr: Vec<(i64, usize)> = numbers
        .into_iter()
        .enumerate()
        .map(|(i, v)| (v, i + 1))
        .collect();

    use std::collections::HashMap;
    let mut pair_sum = HashMap::new();

    for i in 0..n {
        for j in i + 1..n {
            let sum = arr[i].0 + arr[j].0;
            pair_sum.entry(sum).or_insert(Vec::new()).push((i, j));
        }
    }

    for k in 0..n {
        for l in k + 1..n {
            let target = x - arr[k].0 - arr[l].0;
            if let Some(pairs) = pair_sum.get(&target) {
                for &(i, j) in pairs {
                    if i != k && i != l && j != k && j != l {
                        println!("{} {} {} {}", arr[i].1, arr[j].1, arr[k].1, arr[l].1);
                        return;
                    }
                }
            }
        }
    }

    println!("IMPOSSIBLE");
}
