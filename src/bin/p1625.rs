const N: usize = 7;

fn main() {
    let input = std::io::stdin()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .collect::<Vec<_>>();
    let mut count = 0;
    // Visisted contains bounding box
    let mut visited = VisitedGrid::new();
    for i in 0..=N + 1 {
        visited.set(0, i);
        visited.set(N + 1, i);
        visited.set(i, 0);
        visited.set(i, N + 1);
    }
    search(&input, 0, (1, 1), &mut visited, &mut count);
    println!("{count}");
}

fn search(
    input: &[char],
    step: usize,
    (x, y): (usize, usize),
    visited: &mut VisitedGrid,
    count: &mut usize,
) {
    if x < 1 || y < 1 || x > N || y > N || visited.is_set(x, y) {
        return;
    }

    if x == 1 && y == N {
        if step == N * N - 1 {
            *count += 1;
        }
        return;
    }

    // Pruning search trees
    if (!visited.is_set(x - 1, y)
        && !visited.is_set(x + 1, y)
        && visited.is_set(x, y + 1)
        && visited.is_set(x, y - 1))
        || (visited.is_set(x - 1, y)
            && visited.is_set(x + 1, y)
            && !visited.is_set(x, y + 1)
            && !visited.is_set(x, y - 1))
    {
        return;
    }

    visited.set(x, y);

    // If we have a few cells to travel to and one of the cell is surrounded by 3 walls (including the current),
    // then we must travel to that one, otherwise we won't be able to travel it later on.
    if input[step] == '?' {
        if y > 2
            && visited.is_set(x, y - 2)
            && (visited.is_set(x - 1, y - 1) || visited.is_set(x + 1, y - 1))
            && !visited.is_set(x, y - 1)
        {
            search(input, step + 1, (x, y - 1), visited, count);
            visited.unset(x, y);
            return;
        } else if y < 6
            && visited.is_set(x, y + 2)
            && (visited.is_set(x - 1, y + 1) || visited.is_set(x + 1, y + 1))
            && !visited.is_set(x, y + 1)
        {
            search(input, step + 1, (x, y + 1), visited, count);
            visited.unset(x, y);
            return;
        } else if x > 2
            && visited.is_set(x - 2, y)
            && (visited.is_set(x - 1, y - 1) || visited.is_set(x - 1, y + 1))
            && !visited.is_set(x - 1, y)
        {
            search(input, step + 1, (x - 1, y), visited, count);
            visited.unset(x, y);
            return;
        } else if x < 6
            && visited.is_set(x + 2, y)
            && (visited.is_set(x + 1, y - 1) || visited.is_set(x + 1, y + 1))
            && !visited.is_set(x + 1, y)
        {
            search(input, step + 1, (x + 1, y), visited, count);
            visited.unset(x, y);
            return;
        }
    }

    if input[step] == '?' || input[step] == 'L' {
        search(input, step + 1, (x - 1, y), visited, count);
    }
    if input[step] == '?' || input[step] == 'R' {
        search(input, step + 1, (x + 1, y), visited, count);
    }
    if input[step] == '?' || input[step] == 'U' {
        search(input, step + 1, (x, y - 1), visited, count);
    }
    if input[step] == '?' || input[step] == 'D' {
        search(input, step + 1, (x, y + 1), visited, count);
    }

    visited.unset(x, y);
}

struct VisitedGrid(u128);

impl VisitedGrid {
    #[inline(always)]
    fn new() -> Self {
        Self(0)
    }

    #[inline(always)]
    fn set(&mut self, x: usize, y: usize) {
        self.0 |= 1 << (x * (N + 2) + y);
    }

    #[inline(always)]
    fn unset(&mut self, x: usize, y: usize) {
        self.0 &= !(1 << (x * (N + 2) + y));
    }

    #[inline(always)]
    fn is_set(&self, x: usize, y: usize) -> bool {
        (self.0 & (1 << (x * (N + 2) + y))) != 0
    }
}
