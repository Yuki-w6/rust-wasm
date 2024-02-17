use proconio::input;

fn main() {
    input!{
        n: usize
    }
    let mut string = String::new();
    for _ in 0..n {
        string += "10";
    }
    string += "1";
    println!("{}", string);
}
