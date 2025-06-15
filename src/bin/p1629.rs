fn main() {
    let mut lines = std::io::stdin().lines();
    let n = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
    let mut movies = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<_> = line.split_whitespace().collect();
        let start = parts[0].parse::<usize>().unwrap();
        let end = parts[1].parse::<usize>().unwrap();
        movies.push((start, end));
    }
    movies.sort_by_key(|&(_, end)| end);
    let mut count = 0;
    let mut last_end = 0;

    for (start, end) in movies {
        if start >= last_end {
            count += 1;
            last_end = end;
        }
    }

    println!("{}", count);
}
