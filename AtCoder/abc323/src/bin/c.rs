use proconio::input;

fn main() {
    input!{
        n: usize,
        m: u32,
        a: [u32; m],
        s: [String; n]
    }

    let scores: Vec::<u32> = s.iter().map(|si| {
        si.chars().enumerate().filter(|&(_, c)| c == 'o').map(|(idx, _)| a[idx]).sum()
    }).collect();
    let scores_with_index: Vec<u32> = scores.iter().enumerate().map(|(i, &score)| score + (i as u32) + 1).collect();


    for i in 0..n {
        let mut notans: Vec<u32> = s[i].chars().enumerate().filter(|&(_, c)| c == 'x').map(|(idx, _)| a[idx]).collect();
        notans.sort_by(|a, b| b.cmp(a));
        
        let mut needed = 0;
        let mut total_score = scores_with_index[i];
        for &score in &notans {
            if total_score >= *scores_with_index.iter().max().unwrap() {
                break;
            }
            total_score += score;
            needed += 1;
        }
        println!("{}", needed);
    }
}
