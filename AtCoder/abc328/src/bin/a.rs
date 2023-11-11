use proconio::input;

fn main() {
    input!{
        n: usize,
        x: u32,
        s: [u32; n]
    }

    let mut sum: u32 = 0;
    for num in s {
        if num <= x {
            sum += num;
        }
    }

    println!("{}", sum);
}
