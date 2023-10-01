use proconio::input;

fn rotate(p: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut res = vec![vec!['.'; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            res[j][3 - i] = p[i][j];
        }
    }
    res
}

fn can_put(grid: &mut Vec<Vec<char>>, p: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    for i in 0..4 {
        for j in 0..4 {
            if p[i][j] == '#' && (x + i >= 4 || y + j >= 4 || grid[x + i][y + j] == '#') {
                return false;
            }
        }
    }
    true
}

fn put(grid: &mut Vec<Vec<char>>, p: &Vec<Vec<char>>, x: usize, y: usize) {
    for i in 0..4 {
        for j in 0..4 {
            if p[i][j] == '#' {
                grid[x + i][y + j] = '#';
            }
        }
    }
}

fn main() {
    input! {
        p: [[char; 4]; 12]
    }

    let mut ps = vec![];
    for i in 0..3 {
        let mut tmp = p[i * 4..(i + 1) * 4].to_vec();
        let mut set = vec![];
        for _ in 0..4 {
            set.push(tmp.clone());
            tmp = rotate(&tmp);
        }
        ps.push(set);
    }

    'outer: for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                let mut grid = vec![vec!['.'; 4]; 4];
                for x in 0..4 {
                    for y in 0..4 {
                        if can_put(&mut grid, &ps[0][i], x, y) {
                            put(&mut grid, &ps[0][i], x, y);
                            for x2 in 0..4 {
                                for y2 in 0..4 {
                                    let mut grid2 = grid.clone();
                                    if can_put(&mut grid2, &ps[1][j], x2, y2) {
                                        put(&mut grid2, &ps[1][j], x2, y2);
                                        for x3 in 0..4 {
                                            for y3 in 0..4 {
                                                let mut grid3 = grid2.clone();
                                                if can_put(&mut grid3, &ps[2][k], x3, y3) {
                                                    put(&mut grid3, &ps[2][k], x3, y3);
                                                    if grid3.iter().flatten().all(|&c| c == '#') {
                                                        println!("Yes");
                                                        return;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("No");
}
