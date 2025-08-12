#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Range {
    low: u32,
    high: u32,
    index: usize,
}

impl Ord for Range {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.low == other.low {
            self.high.cmp(&other.high).reverse()
        } else {
            self.low.cmp(&other.low)
        }
    }
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut lines = std::io::stdin().lines();
    let n = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
    let mut pairs = lines
        .take(n)
        .map(|line| {
            let line = line.unwrap();
            let mut parts = line.split_whitespace();
            (
                parts.next().unwrap().parse::<u32>().unwrap(),
                parts.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .enumerate()
        .map(|(i, (x, y))| Range {
            low: x,
            high: y,
            index: i,
        })
        .collect::<Vec<_>>();
    let mut contains = vec![0; n];
    let mut contained = vec![0; n];

    pairs.sort();

    let mut min_end = u32::MAX;
    for i in (0..n).rev() {
        if pairs[i].high >= min_end {
            contains[pairs[i].index] = 1;
        }
        min_end = min_end.min(pairs[i].high);
    }

    let mut max_end = 0;
    for i in 0..n {
        if pairs[i].high <= max_end {
            contained[pairs[i].index] = 1;
        }
        max_end = max_end.max(pairs[i].high);
    }

    println!(
        "{}",
        contains
            .iter()
            .fold(String::new(), |mut acc, &x| {
                acc.push_str(if x > 0 { "1 " } else { "0 " });
                acc
            })
            .trim_end()
    );
    println!(
        "{}",
        contained
            .iter()
            .fold(String::new(), |mut acc, &x| {
                acc.push_str(if x > 0 { "1 " } else { "0 " });
                acc
            })
            .trim_end()
    );
}
