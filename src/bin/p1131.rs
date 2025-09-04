fn main() {
    let mut lines = std::io::stdin().lines();
    let n = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<usize>()
        .unwrap();
    let edges: Vec<(usize, usize)> = lines
        .take(n - 1)
        .map(|line| {
            let line = line.unwrap();
            let mut parts = line.split_whitespace();
            (
                parts.next().unwrap().parse::<usize>().unwrap() - 1,
                parts.next().unwrap().parse::<usize>().unwrap() - 1,
            )
        })
        .collect();

    let mut tree = vec![Vec::new(); n];
    for (u, v) in edges {
        tree[u].push(v);
        tree[v].push(u);
    }

    fn bfs(start: usize, tree: &[Vec<usize>]) -> (usize, usize) {
        let mut dist = vec![None; tree.len()];
        let mut queue = std::collections::VecDeque::new();
        dist[start] = Some(0);
        queue.push_back(start);
        let mut farthest = start;
        while let Some(u) = queue.pop_front() {
            for &v in &tree[u] {
                if dist[v].is_none() {
                    dist[v] = Some(dist[u].unwrap() + 1);
                    queue.push_back(v);
                    if dist[v].unwrap() > dist[farthest].unwrap() {
                        farthest = v;
                    }
                }
            }
        }
        (farthest, dist[farthest].unwrap())
    }

    let (u, _) = bfs(0, &tree);
    let (_, diameter) = bfs(u, &tree);
    println!("{}", diameter);
}
