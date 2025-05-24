fn main() {
    let n = std::io::stdin()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    for i in 1..=n {
        match i {
            1 => println!("0"),
            2 => println!("6"),
            3 => println!("28"),
            4 => println!("96"),
            n => {
                let n2 = n * n;
                let p1 = n2 * (n2 - 1) / 2;
                let p2 = 4 * n2 - 12 * n + 8;
                println!("{}", p1 - p2);
            }
        }
    }
}
