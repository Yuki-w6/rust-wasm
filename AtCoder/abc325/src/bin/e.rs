use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

// 正解できてない
fn main() {
    input! {
        n: usize,
        a: i128, // i64からi128に変更
        b: i128, // i64からi128に変更
        c: i128, // i64からi128に変更
        d: [[i128; n]; n], // i64からi128に変更
    }

    let mut min_time = vec![vec![std::i128::MAX; n]; 2]; // i64からi128に変更
    min_time[0][0] = 0; // 初期位置の時間は0

    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, 0, true))); // (cost, position, is_using_car)の形式で初期位置と状態をpush

    while let Some(Reverse((time, position, is_using_car))) = heap.pop() {
        if time > min_time[is_using_car as usize][position] {
            continue; // すでにより短いルートがある場合はスキップ
        }

        for next_position in 0..n {
            if next_position == position {
                continue; // 同じ都市への移動はスキップ
            }

            // 車での移動
            let by_car = time + d[position][next_position] * a;
            if by_car < min_time[true as usize][next_position] {
                min_time[true as usize][next_position] = by_car;
                heap.push(Reverse((by_car, next_position, true)));
            }

            // 車で移動している場合のみ、電車に乗り換え可能
            if is_using_car {
                let by_train = time + d[position][next_position] * b + c; // 電車での移動にはcのコストが必要
                if by_train < min_time[false as usize][next_position] {
                    min_time[false as usize][next_position] = by_train;
                    heap.push(Reverse((by_train, next_position, false))); // 車を使っていない状態でpush
                }
            }
        }
    }

    // 最終目的地までの最短時間を計算
    let answer = std::cmp::min(min_time[0][n-1], min_time[1][n-1]);
    println!("{}", answer);
}
