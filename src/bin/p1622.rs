fn main() {
    let input = std::io::stdin().lines().next().unwrap().unwrap();
    let mut combs = vec![];
    let mut current = input.chars().collect::<Vec<char>>();
    current.sort_unstable();
    loop {
        combs.push(current.iter().collect::<String>());
        if !lex_next(&mut current) {
            break;
        }
    }
    println!("{}", combs.len());
    for comb in combs {
        println!("{}", comb);
    }
}

fn lex_next(s: &mut Vec<char>) -> bool {
    let n = s.len();
    if n == 0 {
        return false;
    }
    let mut i = n - 1;
    while i > 0 && s[i - 1] >= s[i] {
        i -= 1;
    }
    if i == 0 {
        return false;
    }
    let mut j = n - 1;
    while s[j] <= s[i - 1] {
        j -= 1;
    }
    s.swap(i - 1, j);
    s[i..].reverse();
    true
}
