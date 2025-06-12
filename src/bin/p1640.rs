use std::collections::HashMap;

fn main() {
    let mut lines = std::io::stdin().lines();
    let (_, x) = {
        let first_line = lines.next().unwrap().unwrap();
        let mut parts = first_line.split_whitespace();
        (
            parts.next().unwrap().parse::<usize>().unwrap(),
            parts.next().unwrap().parse::<i64>().unwrap(),
        )
    };
    let numbers = {
        let second_line = lines.next().unwrap().unwrap();
        second_line
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<_>>()
    };
    let mut complement_map = HashMap::new();
    for (i, num) in numbers.iter().enumerate() {
        let complement = x - num;
        if complement_map.contains_key(&complement) {
            let complement_index = complement_map[&complement];
            println!("{} {}", complement_index + 1, i + 1);
            return;
        }
        complement_map.insert(num, i);
    }
    println!("IMPOSSIBLE");
}
