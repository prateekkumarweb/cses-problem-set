fn main() {
    let mut lines = std::io::stdin().lines();
    lines.next();
    let line = lines.next().unwrap().unwrap();
    let mut count = 0;
    let mut prev = 0;
    for x in line.split_ascii_whitespace() {
        let x: usize = x.parse().unwrap();
        if x < prev {
            count += prev - x;
        } else {
            prev = x;
        }
    }
    println!("{count}");
}
