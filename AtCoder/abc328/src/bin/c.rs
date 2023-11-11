use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
    }

    let mut same_char_count = vec![0; n + 1];
    for i in 0..n - 1 {
        same_char_count[i + 1] = same_char_count[i] + if s[i] == s[i + 1] { 1 } else { 0 };
    }

    for _ in 0..q {
        input! {
            l: usize,
            r: usize,
        }

        let count = same_char_count[r - 1] - same_char_count[l - 1];
        println!("{}", count);
    }
}
