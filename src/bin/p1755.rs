use std::collections::HashMap;

fn main() {
    let input = std::io::stdin().lines().next().unwrap().unwrap();
    let mut chars_count: HashMap<char, u32> = HashMap::new();
    for c in input.chars() {
        let entry = chars_count.entry(c).or_default();
        *entry += 1;
    }
    let (even_counts, odd_counts): (Vec<_>, Vec<_>) = chars_count
        .into_iter()
        .partition(|(_, count)| count % 2 == 0);

    if odd_counts.len() > 1 {
        println!("NO SOLUTION");
    } else {
        let mut result = String::new();

        for (c, count) in even_counts {
            let half_count = count / 2;
            result.push_str(&c.to_string().repeat(half_count as usize));
        }

        let middle = if let Some((c, count)) = odd_counts.into_iter().next() {
            c.to_string().repeat(count as usize)
        } else {
            String::new()
        };

        println!(
            "{}{}{}",
            result,
            middle,
            result.chars().rev().collect::<String>()
        );
    }
}
