fn main() {
    let mut lines = std::io::stdin().lines();
    let n = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
    let boss = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let mut tree = vec![vec![]; n + 1];
    for (i, &b) in boss.iter().enumerate() {
        tree[b].push(i + 2);
    }
    let mut subordinates = vec![0; n];
    dfs(1, &tree, &mut subordinates);
    for v in subordinates {
        print!("{v} ");
    }
    println!();
}

fn dfs(node: usize, tree: &Vec<Vec<usize>>, subordinates: &mut Vec<usize>) -> usize {
    let mut count = 0;
    for &child in &tree[node] {
        count += dfs(child, tree, subordinates) + 1;
    }
    subordinates[node - 1] = count;
    count
}
