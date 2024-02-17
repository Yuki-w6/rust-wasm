use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        t: Chars,
        s: [Chars; h],
    }

    let mut possible_positions = vec![vec![true; w]; h];

    for i in 0..h {
        possible_positions[i][0] = false;
        possible_positions[i][w-1] = false;
    }
    for i in 0..w {
        possible_positions[0][i] = false;
        possible_positions[h-1][i] = false;
    }

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                possible_positions[i][j] = false;
            }
        }
    }

    for dir in t.iter().rev() {
        let mut new_possible_positions = vec![vec![false; w]; h];
        for i in 0..h {
            for j in 0..w {
                if !possible_positions[i][j] {
                    continue;
                }
                match dir {
                    'L' => if j > 0 { new_possible_positions[i][j-1] = true; },
                    'R' => if j < w-1 { new_possible_positions[i][j+1] = true; },
                    'U' => if i > 0 { new_possible_positions[i-1][j] = true; },
                    'D' => if i < h-1 { new_possible_positions[i+1][j] = true; },
                    _ => unreachable!(),
                }
            }
        }
        possible_positions = new_possible_positions;
    }

    let mut count = 0;
    for i in 0..h {
        for j in 0..w {
            if possible_positions[i][j] {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
