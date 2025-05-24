fn main() {
    let seq = std::io::stdin().lines().next().unwrap().unwrap();
    let mut max = 0;
    let mut prev = ' ';
    let mut prev_count = 0;
    for c in seq.chars() {
        if c == prev {
            prev_count += 1;
        } else {
            max = max.max(prev_count);
            prev = c;
            prev_count = 1;
        }
    }
    println!("{}", max.max(prev_count));
}
