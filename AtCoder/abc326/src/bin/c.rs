use proconio::input;

fn main() {
    input! {
        n: usize,
        m: i64,
        mut a: [i64; n],
    }

    a.sort_unstable();

    let mut start = 0;
    let mut max_presents = 0;

    for end in 0..n {
        while a[end] - a[start] >= m {
            start += 1;
        }

        let num_presents = end - start + 1;
        max_presents = max_presents.max(num_presents);
    }

    println!("{}", max_presents);
}
