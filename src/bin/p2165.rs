fn main() {
    let n = std::io::stdin()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .parse::<u8>()
        .unwrap();
    let mut result = Vec::new();
    tower_of_hanoi(n, 1, 3, 2, &mut result);
    println!("{}", result.len());
    for (left, right) in result {
        println!("{left} {right}");
    }
}

fn tower_of_hanoi(n: u8, left: u8, right: u8, middle: u8, result: &mut Vec<(u8, u8)>) {
    if n == 0 {
        return;
    }
    tower_of_hanoi(n - 1, left, middle, right, result);
    result.push((left, right));
    tower_of_hanoi(n - 1, middle, right, left, result);
}
