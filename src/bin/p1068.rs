fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut n: usize = buf.trim().parse().unwrap();
    print!("{}", n);
    while n != 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        print!(" {}", n);
    }
    println!();
}
