use proconio::input;

fn main() {
    input!{
        q: usize,
        query: [(u32, u32); q]
    }

    let mut a: Vec<u32> = Vec::new();

    for (t, num) in query.iter() {
        match *t {
            1 => a.push(*num),
            _ => {
                let index = *num as usize;
                if index <= a.len() && index > 0 {
                    println!("{}", a[a.len() - index]);
                } else {
                }
            }
        }
    }
}
