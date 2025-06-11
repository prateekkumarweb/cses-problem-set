#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Event {
    Start,
    End,
}

fn main() {
    let mut lines = std::io::stdin().lines();
    let n = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
    let mut queue = vec![(0, Event::Start); 2 * n];
    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut iter = line.split_ascii_whitespace();
        let start = iter.next().unwrap().parse::<usize>().unwrap();
        let end = iter.next().unwrap().parse::<usize>().unwrap();
        queue[2 * i] = (start, Event::Start);
        queue[2 * i + 1] = (end, Event::End);
    }
    queue.sort_unstable_by_key(|&(time, event)| (time, event == Event::End));
    let mut customers = 0;
    let mut max_customers = 0;
    for (_, event) in queue {
        match event {
            Event::Start => {
                customers += 1;
            }
            Event::End => {
                customers -= 1;
            }
        }
        max_customers = max_customers.max(customers);
    }
    println!("{}", max_customers);
}
