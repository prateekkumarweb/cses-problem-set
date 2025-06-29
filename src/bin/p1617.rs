const MOD: usize = 1_000_000_007;

fn main() {
    let n = std::io::stdin()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let mut power = 1;
    let mut pow_2 = 2;
    for i in 0..32 {
        let mask = 1 << i;
        let bit_is_set = (mask & n) != 0;
        if bit_is_set {
            power = (power * pow_2) % MOD;
        }
        pow_2 = (pow_2 * pow_2) % MOD;
    }
    println!("{power}");
}
