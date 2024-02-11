use proconio::input;

fn main() {
    input!{
        n: u64
    }

    let mut target = n;
    let mut count = n;

    while target > 1 {
        count += target;
        target /= 2;
    }

    println!("{}", count);
}
