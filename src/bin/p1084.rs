fn main() {
    let mut lines = std::io::stdin().lines();
    let (n, _, k) = {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        (
            parts.next().unwrap().parse::<usize>().unwrap(),
            parts.next().unwrap().parse::<usize>().unwrap(),
            parts.next().unwrap().parse::<i64>().unwrap(),
        )
    };
    let mut applicants = lines
        .next()
        .unwrap()
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .map(|x| (x, 0))
        .collect::<Vec<_>>();
    applicants.sort_unstable_by_key(|x| x.0);
    let mut apartments = lines
        .next()
        .unwrap()
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    apartments.sort_unstable();
    let mut index = 0;
    for apt in apartments {
        while index < n && apt - k > applicants[index].0 {
            index += 1;
        }
        if index >= n {
            break;
        }
        let cv = applicants[index].0;
        if apt - k <= cv && cv <= apt + k {
            applicants[index].1 = apt;
            index += 1;
        }
    }
    println!("{}", applicants.iter().filter(|x| x.1 != 0).count());
}
