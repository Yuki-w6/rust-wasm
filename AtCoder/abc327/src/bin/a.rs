use proconio::input;

fn main() {
    input!{
        _n: u32,
        s: String
    }

    if s.contains("ab") || s.contains("ba"){
        println!("Yes");
    } else {
        println!("No")
    }
}

