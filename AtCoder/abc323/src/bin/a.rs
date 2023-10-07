use proconio::input;

fn main() {
    input!{
        s : String
    }

    let mut r = "Yes";
    for (i, char) in s.chars().enumerate() {
        if i >= 1 && (i + 1) % 2 == 0 {
            if char == '1' {
                r = "No";
            }
        }
    }
    println!("{}", r);
}
