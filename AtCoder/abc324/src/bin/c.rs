use proconio::input;

fn main() {
    input! {
        n: usize,
        t: String,
        s: [String; n]
    }

    let mut ans = Vec::new();
    let t_chars: Vec<char> = t.chars().collect();

    for (j, x) in s.iter().enumerate() {
        let x_chars: Vec<char> = x.chars().collect();

        if x == &t {
            ans.push(j + 1);
            continue;
        }

        if x.len() == t.len() + 1 {
            let mut is_match = true;
            let mut skipped = false;
            let mut t_idx = 0;
            
            for &ch in &x_chars {
                if t_idx >= t.len() || ch != t_chars[t_idx] {
                    if skipped {
                        is_match = false;
                        break;
                    }
                    skipped = true;
                } else {
                    t_idx += 1;
                }
            }

            if is_match {
                ans.push(j + 1);
            }
        } else if x.len() == t.len() - 1 {
            let mut is_match = true;
            let mut skipped = false;
            let mut x_idx = 0;
            
            for &ch in &t_chars {
                if x_idx >= x.len() || ch != x_chars[x_idx] {
                    if skipped {
                        is_match = false;
                        break;
                    }
                    skipped = true;
                } else {
                    x_idx += 1;
                }
            }

            if is_match {
                ans.push(j + 1);
            }
        } else if x.len() == t.len() {
            let diffs: usize = x.chars().zip(t.chars()).filter(|&(a, b)| a != b).count();
            if diffs == 1 {
                ans.push(j + 1);
            }
        }
    }

    println!("{}", ans.len());
    if !ans.is_empty() {
        println!("{}", ans.iter().map(ToString::to_string).collect::<Vec<String>>().join(" "));
    } else {
        println!();
    }
}
