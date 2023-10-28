use proconio::input;

fn main() {
    input!{
        n: u32
    }

    let result = find_326_like_number(n);

    println!("{}", result);
}

fn find_326_like_number(mut n: u32) -> u32 {
    loop {
        let h = n / 100;
        let t = (n / 10) % 10;
        let o = n % 10;

        if h * t == o {
            return n;
        }

        n += 1;
    }
}