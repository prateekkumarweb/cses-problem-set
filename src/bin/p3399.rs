fn main() {
    let mut lines = std::io::stdin().lines();
    let t = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
    for _ in 0..t {
        let (n, a, b) = {
            let line = lines.next().unwrap().unwrap();
            let mut parts = line.split_whitespace();
            (
                parts.next().unwrap().parse::<usize>().unwrap(),
                parts.next().unwrap().parse::<usize>().unwrap(),
                parts.next().unwrap().parse::<usize>().unwrap(),
            )
        };
        if n < a + b || (a * b == 0 && a.max(b) != 0) {
            println!("NO");
            continue;
        }
        println!("YES");
        let c = n - a - b;
        let mut result = vec![0; n];
        for x in 1..=n {
            if x > a && x <= a + c {
                result[x - 1] = x;
            }
        }
        let index_to_fill = (1..=n - c).map(|x| (x + a + c - 1) % n);
        let no_to_fill = (1..=n).filter(|&x| !(x > a && x <= a + c));
        for (i, x) in index_to_fill.zip(no_to_fill) {
            result[i] = x;
        }
        println!(
            "{}",
            result
                .iter()
                .fold(String::new(), |mut a, x| {
                    a.push_str(&x.to_string());
                    a.push(' ');
                    a
                })
                .trim()
        );
        let p = (1..=n).fold(String::new(), |mut a, x| {
            a.push_str(&x.to_string());
            a.push(' ');
            a
        });
        println!("{}", p.trim());
    }
}
