use proconio::input;

fn main() {
    input!{
        a: usize,
        b: usize,
        d: usize
    }

    let mut num = &a;
    for (n, _i) in (a..=b).enumerate() {
        print!("{}", num + (n * d));
        if num + (n * d) == b {
            println!();
            break;
        } else {
            print!("{}", ' ');
        }
    }
}
