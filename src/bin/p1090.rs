fn main() {
    let mut lines = std::io::stdin().lines();
    let (n, x) = {
        let first_line = lines.next().unwrap().unwrap();
        let mut parts = first_line.split_whitespace();
        (
            parts.next().unwrap().parse::<usize>().unwrap(),
            parts.next().unwrap().parse::<i64>().unwrap(),
        )
    };
    let mut weights = {
        let second_line = lines.next().unwrap().unwrap();
        second_line
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<_>>()
    };
    weights.sort_unstable();
    let mut taken = vec![false; n];
    let mut index = 0;
    let mut back_index = n - 1;
    let mut count = 0;
    while index < back_index {
        if taken[index] {
            index += 1;
            continue;
        }
        if taken[back_index] {
            back_index -= 1;
            continue;
        }
        if weights[index] + weights[back_index] <= x {
            taken[index] = true;
            taken[back_index] = true;
            count += 1;
            index += 1;
            back_index -= 1;
        } else {
            taken[back_index] = true;
            count += 1;
            back_index -= 1;
        }
    }
    if index == back_index && !taken[index] {
        count += 1;
    }
    println!("{}", count);
}
