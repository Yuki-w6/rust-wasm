use proconio::input;

fn main() {
    input!{
        _n: usize,
        _m: usize,
        s: String,
        t: String,
    }

    if t.starts_with(&s) && t.ends_with(&s) {
        println!("{}", 0);
    } else if t.starts_with(&s) {
        println!("{}", 1);
    } else if t.ends_with(&s) {
        println!("{}", 2);
    } else {
        println!("{}", 3);
    }
}
