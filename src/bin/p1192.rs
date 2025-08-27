fn main() {
    let mut lines = std::io::stdin().lines();
    let (n, m) = {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        (
            parts.next().unwrap().parse::<usize>().unwrap(),
            parts.next().unwrap().parse::<usize>().unwrap(),
        )
    };
    let grid = {
        let mut g = vec![];
        for _ in 0..n {
            g.push(lines.next().unwrap().unwrap().chars().collect::<Vec<_>>());
        }
        g
    };

    let mut visited = vec![vec![false; m]; n];
    let mut rooms = 0;

    fn dfs(
        x: usize,
        y: usize,
        grid: &Vec<Vec<char>>,
        visited: &mut Vec<Vec<bool>>,
        n: usize,
        m: usize,
    ) {
        visited[x][y] = true;
        let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        for (dx, dy) in dirs.iter() {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx >= 0 && nx < n as isize && ny >= 0 && ny < m as isize {
                let nx = nx as usize;
                let ny = ny as usize;
                if grid[nx][ny] == '.' && !visited[nx][ny] {
                    dfs(nx, ny, grid, visited, n, m);
                }
            }
        }
    }

    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == '.' && !visited[i][j] {
                rooms += 1;
                dfs(i, j, &grid, &mut visited, n, m);
            }
        }
    }

    println!("{}", rooms);
}
