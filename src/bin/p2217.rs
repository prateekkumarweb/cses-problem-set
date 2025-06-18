use std::collections::{HashMap, HashSet};

fn main() {
    let mut lines = std::io::stdin().lines();
    let (n, m) = {
        let first_line = lines.next().unwrap().unwrap();
        let mut parts = first_line.split_ascii_whitespace();
        (
            parts.next().unwrap().parse::<i64>().unwrap(),
            parts.next().unwrap().parse::<i64>().unwrap(),
        )
    };
    let mut nums = lines
        .next()
        .unwrap()
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let mut indices_map = nums
        .iter()
        .enumerate()
        .map(|(i, &num)| (num, i + 1))
        .collect::<HashMap<_, _>>();
    let mut result = 1;
    for i in 1i64..n {
        if indices_map[&(i + 1)] < indices_map[&i] {
            result += 1;
        }
    }
    for _ in 0..m {
        let line = lines.next().unwrap().unwrap();
        let (a, b) = {
            let mut parts = line.split_ascii_whitespace();
            (
                parts.next().unwrap().parse::<usize>().unwrap(),
                parts.next().unwrap().parse::<usize>().unwrap(),
            )
        };
        let mut pairs = HashSet::new();
        if nums[a - 1] < n {
            pairs.insert((nums[a - 1], nums[a - 1] + 1));
        }
        if nums[a - 1] > 1 {
            pairs.insert((nums[a - 1] - 1, nums[a - 1]));
        }
        if nums[b - 1] < n {
            pairs.insert((nums[b - 1], nums[b - 1] + 1));
        }
        if nums[b - 1] > 1 {
            pairs.insert((nums[b - 1] - 1, nums[b - 1]));
        }
        for (x, y) in &pairs {
            result -= if indices_map[x] > indices_map[y] {
                1
            } else {
                0
            };
        }
        nums.swap(a - 1, b - 1);
        indices_map.insert(nums[a - 1], a);
        indices_map.insert(nums[b - 1], b);
        for (x, y) in &pairs {
            result += if indices_map[x] > indices_map[y] {
                1
            } else {
                0
            };
        }
        println!("{}", result);
    }
}
