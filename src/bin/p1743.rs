fn main() {
    let input = std::io::stdin()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .collect::<Vec<_>>();
    let n = input.len();
    let mut frequencies = [0; 26];
    for c in input {
        frequencies[c as usize - 'A' as usize] += 1;
    }
    let mut answer = vec![' '; n];
    let mut last_inserted = ' ';
    for a in answer.iter_mut() {
        let mut found = false;
        for j in 0..26 {
            let c = (j as u8 + b'A') as char;
            if frequencies[j] > 0 && c != last_inserted {
                frequencies[j] -= 1;
                if is_possible(&frequencies, j) {
                    *a = c;
                    last_inserted = c;
                    found = true;
                    break;
                } else {
                    frequencies[j] += 1;
                }
            }
        }
        if !found {
            println!("-1");
            return;
        }
    }
    println!("{}", answer.iter().collect::<String>());
}

fn is_possible(frequencies: &[usize; 26], index: usize) -> bool {
    let mut mode = 0;
    let mut total = 0;
    for i in 0..26 {
        if frequencies[i] > frequencies[mode] {
            mode = i;
        }
        total += frequencies[i];
    }
    frequencies[mode] <= total.div_ceil(2) && frequencies[index] <= total.div_ceil(2)
}
