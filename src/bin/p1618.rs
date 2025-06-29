fn main() {
    let n = std::io::stdin()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let mut res = 0;
    for c in 1.. {
        let f = 5usize.pow(c);
        let a = n / f;
        if a == 0 {
            break;
        }
        res += a;
    }
    println!("{res}");
}
