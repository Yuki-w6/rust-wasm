use proconio::input;
use permutohedron::Heap;
use std::cmp;

fn main() {
    input!{
        n: usize,
        s: String,
    }

    let mut chars: Vec<char> = s.chars().collect();

    let max_value = 10_usize.pow(n as u32);
    let mut ans = 0;

    for i in 0..max_value { // 31624 is approximately sqrt(10^9)
        let mut t: Vec<char> = (i * i).to_string().chars().collect();
        while t.len() < n {
            t.insert(0, '0'); // 桁数を揃える
        }
        t.sort();
        if chars == t {
            ans += 1; // 一致するか判定
        }
    }

    println!("{}", ans);
}
