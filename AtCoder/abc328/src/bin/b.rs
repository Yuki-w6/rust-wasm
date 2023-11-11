use proconio::input;

fn main() {
    input!{
        n: usize,
        d: [usize; n]
    }

    let mut zorome_days = 0;

    for month in 1..=n {
        for day in 1..=d[month - 1] {
            if is_zorome(month, day) {
                zorome_days += 1;
            }
        }
    }

    println!("{}", zorome_days);
}

fn is_zorome(month: usize, day: usize) -> bool {
    let month_digits = get_digits(month);
    let day_digits = get_digits(day);

    let num = month_digits[0];
    for &digit in &month_digits {
        if digit != num {
            return false;
        }
    }
    for &digit in &day_digits {
        if digit != num {
            return false;
        }
    }
    true
}

fn get_digits(mut number: usize) -> Vec<usize> {
    let mut digits = Vec::new();
    while number > 0 {
        digits.push(number % 10);
        number /= 10;
    }
    digits.reverse();
    digits
}
