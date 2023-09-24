use proconio::input;

fn main() {
    input!{
        m: u32,
    }

    let mut vv: String = "".to_string();
    if m < 100 {
        vv = "00".to_string();
    } else if 100 <= m && m <= 5000 {
        let n : u32 = m * 10 / 1000;  
            if count_digits(n) == 1 {
                vv = String::from("0") +  &n.to_string();
            } else {
                vv = n.to_string();
            }
    } else if 6000 <= m && m <= 30000 {
        vv = (m / 1000 + 50).to_string();
    } else if 35000 <= m && m <= 70000 {
        vv = ((m / 1000 - 30) / 5 + 80).to_string();
    } else if 70000 < m {
        vv = "89".to_string();
    }
    println!("{}", vv);
}

fn count_digits(mut n: u32) -> usize {
    let mut count = 0;
    while n != 0 {
        count += 1;
        n /= 10;
    }
    count
}
