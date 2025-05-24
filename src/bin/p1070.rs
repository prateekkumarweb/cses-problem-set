fn main() {
    let n = {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().parse::<usize>().unwrap()
    };
    match n {
        1 => println!("1"),
        2 | 3 => println!("NO SOLUTION"),
        _ => {
            print!("2");
            for i in 2..=n / 2 {
                print!(" {}", i * 2);
            }
            for i in 1..=n.div_ceil(2) {
                print!(" {}", i * 2 - 1);
            }
            println!();
        }
    }
}
