use proconio::input;

fn main() {
    input!{
        n: usize,
        s: [String; n]
    }

    let mut results = Vec::new();
    for i in 0..n {
        results.push((s[i].chars().filter(|&c| c == 'o').count(), n - i, i + 1));
    }

    results.sort_by(|a, b| (b.0, b.1).cmp(&(a.0, a.1)));

    for i in 0..results.len() {
        if i == results.len() - 1 {
            println!("{}", results[i].2);
        } else {
            print!("{} ", results[i].2);
        }
    }
}
