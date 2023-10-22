use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

fn main() {
    input! {
        h: usize,
        w: usize,
        grid: [Chars; h],
    }

    let mut visited = vec![vec![false; w]; h];
    let dirs = vec![(0, 1), (0, -1), (1, 0), (-1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1)];
    let mut count = 0;

    for i in 0..h {
        for j in 0..w {
            if grid[i][j] == '#' && !visited[i][j] {
                count += 1;
                let mut queue = VecDeque::new();
                queue.push_back((i, j));
                visited[i][j] = true;

                while let Some((x, y)) = queue.pop_front() {
                    for &(dx, dy) in dirs.iter() {
                        let new_x = x as i32 + dx;
                        let new_y = y as i32 + dy;

                        if new_x >= 0 && new_x < h as i32 && new_y >= 0 && new_y < w as i32 {
                            let (new_x, new_y) = (new_x as usize, new_y as usize);
                            if grid[new_x][new_y] == '#' && !visited[new_x][new_y] {
                                queue.push_back((new_x, new_y));
                                visited[new_x][new_y] = true;
                            }
                        }
                    }
                }
            }
        }
    }

    println!("{}", count);
}
