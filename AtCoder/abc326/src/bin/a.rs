use proconio::input;

fn main() {
    input!{
        x: u32,
        y: u32
    }

    if x < y {
        if y - x > 2 {
            println!("{}", "No");
        } else {
            println!("{}", "Yes");
        }
    } else {
        if x - y > 3 {
            println!("{}", "No");
        } else {
            println!("{}", "Yes");
        }
    }    
}
