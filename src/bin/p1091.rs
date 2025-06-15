use std::collections::BTreeMap;

fn main() {
    let mut lines = std::io::stdin().lines();
    let (_, _) = {
        let first_line = lines.next().unwrap().unwrap();
        let mut parts = first_line.split_whitespace();
        (
            parts.next().unwrap().parse::<usize>().unwrap(),
            parts.next().unwrap().parse::<usize>().unwrap(),
        )
    };
    let mut tickets = {
        let second_line = lines.next().unwrap().unwrap();
        let list = second_line
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap());
        let mut btree = BTreeMap::new();
        for ticket in list {
            *btree.entry(ticket).or_insert(0) += 1;
        }
        btree
    };
    let customers = {
        let third_line = lines.next().unwrap().unwrap();
        third_line
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
    };
    for customer in customers {
        let entry = tickets.range_mut(..=customer).next_back();
        if entry.is_none() {
            println!("-1");
            continue;
        }
        let (&ticket, count) = entry.unwrap();
        if *count == 1 {
            tickets.remove(&ticket);
        } else {
            *count -= 1;
        }
        println!("{}", ticket);
    }
}
