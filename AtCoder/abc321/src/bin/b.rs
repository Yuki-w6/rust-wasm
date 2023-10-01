use proconio::input;

fn main() {
    input!{
        n: u32,
        x: u32,
        mut a: [u32; n-1]
    }

    a.sort();

    if a.len() <= 1 {
        println!("-1");
        return;
    }

    let sum_except_first: u32 = a[1..].iter().sum();
    let sum_except_end: u32 = a[..a.len() -1].iter().sum();
    let sum_except_first_end: u32 = a[1..a.len() -1].iter().sum();

    if sum_except_end == x {
        println!("{}", 0);
    } else if sum_except_first_end == x {
        println!("{}", a[0]);
    } else if sum_except_first == x {
        println!("{}", a[a.len() - 1]);
    }  else if x - sum_except_first_end >= a[0] && x - sum_except_first_end <= a[a.len() -1] {
        println!("{}", x - sum_except_first_end);
    } else {
        println!("{}", -1);
    }
}
