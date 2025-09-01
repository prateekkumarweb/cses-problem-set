fn main() {
    let mut lines = std::io::stdin().lines();
    let (n, x) = {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        (
            parts.next().unwrap().parse::<usize>().unwrap(),
            parts.next().unwrap().parse::<usize>().unwrap(),
        )
    };
    let numbers = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut arr: Vec<(usize, usize)> = numbers
        .into_iter()
        .enumerate()
        .map(|(i, v)| (v, i + 1))
        .collect();
    arr.sort();

    for i in 0..n {
        let a = arr[i].0;
        let mut left = i + 1;
        let mut right = n - 1;
        while left < right {
            let sum = a + arr[left].0 + arr[right].0;
            if sum == x {
                println!("{} {} {}", arr[i].1, arr[left].1, arr[right].1);
                return;
            } else if sum < x {
                left += 1;
            } else {
                right -= 1;
            }
        }
    }
    println!("IMPOSSIBLE");
}
