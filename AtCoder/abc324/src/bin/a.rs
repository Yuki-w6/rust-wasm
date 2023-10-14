use proconio::input;

fn main() {
    input!{
        n: usize,
        a: [u32; n],
    }

    let num: u32 = a[0];

    let mut ans: String = "Yes".to_string();
    for x in a {
        if x != num {
            ans = "No".to_string();
        }
    }

println!("{}", ans);
}
