use proconio::input;


fn get_digits(n: u32) -> Vec<u32> {
    n.to_string()
        .chars()
        .filter_map(|ch| ch.to_digit(10))
        .collect()
}

fn main() {
    input!{
        n: u32,
    }

    let digit = get_digits(n);
    
    let mut pre_digit: u32 = 0;
    for (i, num) in digit.iter().enumerate() {
        if digit.len() == 1 {
            println!("{}", "Yes");
            break;
        }
        if i == 0 {
            pre_digit = *num;
            continue;
        }
        if num >= &pre_digit {
            println!("{}", "No");
            break;
        } else {
            pre_digit = *num;
            if i == digit.len() - 1  {
                println!("{}", "Yes");
            }
        }
    }
}
