fn main() {
    let n = std::io::stdin()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let mut result = vec!["0".to_owned(), "1".to_owned()];
    for _ in 2..=n {
        let mut next = vec![];
        for j in 0..result.len() {
            let mut s = result[j].to_string();
            s.push('0');
            next.push(s);
        }
        for j in (0..result.len()).rev() {
            let mut s = result[j].to_string();
            s.push('1');
            next.push(s);
        }
        result = next;
    }
    for r in result {
        println!("{}", r);
    }
}
