fn main () {
    println!("{}", f('a', 37, 41))
}

fn f(ch: char, num1: i16, num2:i16) -> i16 {
    if ch == 'a' { num1 }
    else {num2 }
}