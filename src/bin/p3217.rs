use std::collections::VecDeque;

fn main() {
    let n = std::io::stdin()
        .lines()
        .next()
        .unwrap()
        .map(|x| x.parse::<usize>().unwrap())
        .unwrap();
    let mut grid = vec![vec![-1; n]; n];
    grid[0][0] = 0;
    let mut queue = VecDeque::from([(0, 0)]);
    while let Some(p) = queue.pop_front() {
        let positions = knight_pos(n, p);
        let c = grid[p.0][p.1];
        for p in positions {
            if grid[p.0][p.1] == -1 {
                grid[p.0][p.1] = c + 1;
                queue.push_back(p);
            }
        }
    }
    for row in grid {
        for x in row {
            print!("{} ", x);
        }
        println!();
    }
}

fn knight_pos(n: usize, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
    let mut positions = vec![];
    let moves = [
        (2, 1),
        (2, -1),
        (-2, 1),
        (-2, -1),
        (1, 2),
        (1, -2),
        (-1, 2),
        (-1, -2),
    ];
    for (dx, dy) in moves.iter() {
        let new_x = x as isize + dx;
        let new_y = y as isize + dy;
        if new_x >= 0 && new_x < n as isize && new_y >= 0 && new_y < n as isize {
            positions.push((new_x as usize, new_y as usize));
        }
    }
    positions
}
