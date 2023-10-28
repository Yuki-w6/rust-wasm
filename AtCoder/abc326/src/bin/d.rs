use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        r: String,
        c: String,
    }

    let r: Vec<char> = r.chars().collect();
    let c: Vec<char> = c.chars().collect();

    // 各行と各列に 'A', 'B', 'C' のどれかがあることを保証するためのセット
    let letters = vec!['A', 'B', 'C'];
    let mut sets = vec![HashSet::new(); n];

    // 初期マトリックスの構築。'.' で初期化された n x n のマトリックスを作成します。
    let mut matrix = vec![vec!['.'; n]; n];

    // RとCに基づいてマトリックスに文字を配置します。
    for i in 0..n {
        matrix[i][i] = r[i];
        sets[i].insert(r[i]);
    }

    // R と C の制約を満たすためにマトリックスを更新します。
    for i in 0..n {
        for j in 0..n {
            if r[i] == c[j] && matrix[i][j] == '.' {
                matrix[i][j] = r[i];
                sets[i].insert(r[i]);
                break;
            }
        }
    }

    // 各行と各列に'A', 'B', 'C'がちょうど1回ずつ現れるようにマトリックスを更新します。
    for i in 0..n {
        for j in 0..n {
            if matrix[i][j] == '.' {
                for &letter in &letters {
                    if !sets[i].contains(&letter) {
                        matrix[i][j] = letter;
                        sets[i].insert(letter);
                        break;
                    }
                }
            }
        }
    }

    // 最終的な検証
    if validate_matrix(&matrix, &r, &c, n) {
        println!("Yes");
        for row in &matrix {
            let line: String = row.iter().collect();
            println!("{}", line);
        }
    } else {
        println!("No");
    }
}

// マトリックスがすべての条件を満たしているかどうかを検証する関数
fn validate_matrix(matrix: &[Vec<char>], r: &[char], c: &[char], n: usize) -> bool {
    // 行ごとの検証
    for i in 0..n {
        let mut occurrences = HashSet::new();
        for j in 0..n {
            let ch = matrix[i][j];
            occurrences.insert(ch);
            if j == 0 && ch != r[i] {
                return false;
            }
        }
        if occurrences.len() != 3 {
            return false;
        }
    }

    // 列ごとの検証
    for i in 0..n {
        let mut occurrences = HashSet::new();
        for j in 0..n {
            let ch = matrix[j][i];
            occurrences.insert(ch);
            if j == 0 && ch != c[i] {
                return false;
            }
        }
        if occurrences.len() != 3 {
            return false;
        }
    }

    true
}
