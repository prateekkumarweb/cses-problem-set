fn main() {
    let mut lines = std::io::stdin().lines();
    let t = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
    for _ in 0..t {
        let (a, b) = {
            let line = lines.next().unwrap().unwrap();
            let mut parts = line.split_whitespace();
            (
                parts.next().unwrap().parse::<usize>().unwrap(),
                parts.next().unwrap().parse::<usize>().unwrap(),
            )
        };
        if 2 * a >= b && 2 * b >= a {
            if (2 * a - b) % 3 == 0 && (2 * b - a) % 3 == 0 {
                println!("YES");
            } else {
                println!("NO");
            }
        } else {
            println!("NO");
        }
    }
}
