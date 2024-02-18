use proconio::input;

fn main() {
    input!{
        s: String
    }
    let count = s.chars().filter(|&c| c == '1').count();
    println!("{}", count);
}
