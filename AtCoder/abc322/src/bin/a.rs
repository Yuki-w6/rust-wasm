use proconio::input;
use regex::Regex;

fn main() {
    input!{
        n: u32,
        s: String
    }

    let re = Regex::new("ABC").unwrap();
    match re.find(&s) {
        Some(m) => println!("{}", m.start() + 1),
        None => println!("{}", -1),
    }
}
