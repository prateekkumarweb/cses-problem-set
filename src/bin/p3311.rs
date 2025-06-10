#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    A,
    B,
    C,
    D,
}

fn main() {
    let mut lines = std::io::stdin().lines();
    let (n, m) = lines
        .next()
        .unwrap()
        .unwrap()
        .split_once(' ')
        .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
        .unwrap();
    let grid = lines
        .take(n)
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| match c {
                    'A' => Color::A,
                    'B' => Color::B,
                    'C' => Color::C,
                    'D' => Color::D,
                    _ => panic!("Invalid color"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut answer: Vec<Vec<Color>> = vec![];
    for i in 0..n {
        let mut row = vec![];
        for j in 0..m {
            let mut diff = vec![grid[i][j]];
            if i != 0 {
                diff.push(answer[i - 1][j]);
            }
            if j != 0 {
                diff.push(row[j - 1]);
            }
            let c = pick_diff(&diff).unwrap();
            row.push(c);
        }
        answer.push(row);
    }
    for row in answer {
        for c in row {
            print!(
                "{}",
                match c {
                    Color::A => 'A',
                    Color::B => 'B',
                    Color::C => 'C',
                    Color::D => 'D',
                }
            );
        }
        println!();
    }
}

fn pick_diff(a: &[Color]) -> Option<Color> {
    [Color::A, Color::B, Color::C, Color::D]
        .into_iter()
        .find(|&c| !a.contains(&c))
}
