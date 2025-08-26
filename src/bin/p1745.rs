fn main() {
    let mut lines = std::io::stdin().lines();
    let _n = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<usize>()
        .unwrap();
    let xs = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let sum: usize = xs.iter().sum();
    let mut possible = vec![false; sum + 1];
    possible[0] = true;

    for &coin in &xs {
        for s in (coin..=sum).rev() {
            if possible[s - coin] {
                possible[s] = true;
            }
        }
    }

    let results: Vec<_> = (1..=sum).filter(|&s| possible[s]).collect();

    println!("{}", results.len());
    println!(
        "{}",
        results
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
