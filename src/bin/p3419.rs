use std::collections::HashSet;

fn main() {
    let n = std::io::stdin()
        .lines()
        .next()
        .unwrap()
        .map(|x| x.parse::<usize>())
        .unwrap()
        .unwrap();
    let mut grid = vec![vec![0; n]; n];
    for m in 1..(2 * n - 1) {
        for x in 0..=m {
            let y = m - x;
            if y < n && x <= y {
                let mut set = HashSet::new();
                for i in grid[x][..y].iter().copied() {
                    set.insert(i);
                }
                for i in grid[..x].iter().map(|row| row[y]) {
                    set.insert(i);
                }
                let mut set = set.into_iter().collect::<Vec<_>>();
                set.sort_unstable();
                let found = set
                    .iter()
                    .enumerate()
                    .find(|&(i, x)| i != *x)
                    .map(|x| x.0)
                    .unwrap_or(set.len());
                grid[x][y] = found;
                if x != y {
                    grid[y][x] = grid[x][y];
                }
            }
        }
    }
    for row in grid {
        for x in row {
            print!("{x} ");
        }
        println!();
    }
}
