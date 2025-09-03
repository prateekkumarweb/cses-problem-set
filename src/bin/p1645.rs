struct Entry {
    value: i64,
    index: usize,
}

fn main() {
    let mut lines = std::io::stdin().lines();
    let n = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<usize>()
        .unwrap();
    let numbers = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut stack: Vec<Entry> = Vec::new();
    let mut result = vec![0; n];

    for (i, &num) in numbers.iter().enumerate() {
        while let Some(top) = stack.last() {
            if top.value < num {
                result[i] = top.index + 1;
                break;
            } else {
                stack.pop();
            }
        }
        if stack.is_empty() {
            result[i] = 0;
        }
        stack.push(Entry {
            value: num,
            index: i,
        });
    }

    println!(
        "{}",
        result
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
