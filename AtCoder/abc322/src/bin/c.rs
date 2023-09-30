use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    }

    let mut days = vec![n; n];
    let mut last_firework = 0;

    for &ai in &a {
        for j in last_firework..ai {
            days[j] = ai - j -1;
        }
        last_firework = ai;
    }

    for &day in &days {
        println!("{}", day);
    }
}