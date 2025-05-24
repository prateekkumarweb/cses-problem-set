fn main() {
    let mut lines = std::io::stdin().lines();
    let n = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut iter = line.split_ascii_whitespace();
        let y = iter.next().unwrap().parse::<usize>().unwrap();
        let x = iter.next().unwrap().parse::<usize>().unwrap();
        println!("{}", spiral_num(y, x));
    }
}

fn spiral_num(mut y: usize, mut x: usize) -> usize {
    let max = y.max(x);
    let layer = max - 1;
    let layer_start = layer * layer + 1;
    let layer_end = (layer + 1) * (layer + 1);
    if max % 2 == 1 {
        std::mem::swap(&mut y, &mut x);
    }
    if x <= y {
        layer_end - x + 1
    } else {
        layer_start + y - 1
    }
}
