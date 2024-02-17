use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
        st: [(u64, u64); n-1],
    }

    let mut currencies = a;
    for i in 0..n-1 {
        let (s, t) = st[i];
        let exchanges = currencies[i] / s;
        currencies[i] -= exchanges * s;
        currencies[i+1] += exchanges * t;
    }

    println!("{}", currencies[n-1]);
}