fn main() {
    let mut lines = std::io::stdin().lines();
    let q = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
    let count = [
        9usize,
        90 * 2,
        900 * 3,
        9000 * 4,
        90000 * 5,
        900000 * 6,
        9000000 * 7,
        90000000 * 8,
        900000000 * 9,
        9000000000 * 10,
        90000000000 * 11,
        900000000000 * 12,
        9000000000000 * 13,
        90000000000000 * 14,
        900000000000000 * 15,
        9000000000000000 * 16,
        90000000000000000 * 17,
        900000000000000000 * 18,
    ];
    let mut cumm_count = [0usize; 19];
    for i in 1..19 {
        cumm_count[i] = cumm_count[i - 1] + count[i - 1];
    }

    for _ in 0..q {
        let k = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
        let i = cumm_count.binary_search(&k).unwrap_or_else(|x| x);
        let l = cumm_count[i - 1];
        let k = k - l - 1;
        let d = k / i + 10usize.pow(i as u32 - 1);
        let r = k % i;
        println!("{}", d.to_string().chars().nth(r).unwrap());
    }
}
