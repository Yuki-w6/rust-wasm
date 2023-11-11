use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut stack: Vec<char> = Vec::new();

    for ch in s.chars() {
        if ch == 'C' && stack.len() >= 2 && stack[stack.len() - 1] == 'B' && stack[stack.len() - 2] == 'A' {
            stack.pop();
            stack.pop();
        } else {
            stack.push(ch);
        }
    }

    let result: String = stack.into_iter().collect();
    if !result.is_empty() {
        println!("{}", result);
    }
}