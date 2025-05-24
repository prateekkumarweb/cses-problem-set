fn main() {
    let n = std::io::stdin()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let r = n % 4;
    match r {
        1 | 2 => println!("NO"),
        0 => {
            println!("YES");
            println!("{}", n / 2);
            for i in 1..=n {
                if i % 4 == 0 || i % 4 == 1 {
                    print!("{} ", i);
                }
            }
            println!();
            println!("{}", n / 2);
            for i in 1..=n {
                if i % 4 == 2 || i % 4 == 3 {
                    print!("{} ", i);
                }
            }
            println!();
        }
        3 => {
            println!("YES");
            println!("{}", n / 2 + 1);
            for i in 1..=n {
                if i % 4 == 1 || i % 4 == 2 {
                    print!("{} ", i);
                }
            }
            println!();
            println!("{}", n / 2);
            for i in 1..=n {
                if i % 4 == 0 || i % 4 == 3 {
                    print!("{} ", i);
                }
            }
            println!();
        }
        _ => unreachable!(),
    };
}
