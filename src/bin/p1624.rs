fn main() {
    let chess = std::io::stdin()
        .lines()
        .take(8)
        .map(|line| line.unwrap().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut count = 0;
    'outer: for perm in permutations(&[0, 1, 2, 3, 4, 5, 6, 7]) {
        for i in 0..8 {
            for j in (i + 1)..8 {
                let p1 = (i, perm[i]);
                if chess[p1.0][p1.1] == '*' {
                    continue 'outer;
                }
                let p2 = (j, perm[j]);
                if chess[p2.0][p2.1] == '*' {
                    continue 'outer;
                }
                if p1.0.abs_diff(p2.0) == p1.1.abs_diff(p2.1) {
                    continue 'outer;
                }
            }
        }
        count += 1;
    }
    println!("{count}");
}

fn permutations(input: &[usize]) -> Vec<Vec<usize>> {
    if input.len() == 1 {
        return vec![input.to_vec()];
    }
    let mut result = Vec::new();
    for i in 0..input.len() {
        let mut rest = input.to_vec();
        let current = rest.remove(i);
        for mut perm in permutations(&rest) {
            perm.insert(0, current);
            result.push(perm);
        }
    }
    result
}
