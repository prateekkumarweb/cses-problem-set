const MOD: usize = 1_000_000_007;

struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    is_end: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: Default::default(),
            is_end: false,
        }
    }

    fn insert(&mut self, word: &str) {
        let mut node = self;
        for &b in word.as_bytes() {
            let idx = (b - b'a') as usize;
            node = node.children[idx].get_or_insert_with(|| Box::new(TrieNode::new()));
        }
        node.is_end = true;
    }
}

fn main() {
    let mut lines = std::io::stdin().lines();
    let s = lines.next().unwrap().unwrap();
    let n = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
    let words: Vec<String> = lines.take(n).map(|line| line.unwrap()).collect();

    let mut trie = TrieNode::new();
    for word in &words {
        trie.insert(word);
    }

    let len = s.len();
    let s_bytes = s.as_bytes();
    let mut dp = vec![0usize; len + 1];
    dp[0] = 1;

    for i in 0..len {
        if dp[i] == 0 {
            continue;
        }
        let mut node = &trie;
        for j in i..len {
            let idx = (s_bytes[j] - b'a') as usize;
            if let Some(child) = &node.children[idx] {
                node = child;
                if node.is_end {
                    dp[j + 1] = (dp[j + 1] + dp[i]) % MOD;
                }
            } else {
                break;
            }
        }
    }

    println!("{}", dp[len]);
}
