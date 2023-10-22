use proconio::input;
use std::collections::BinaryHeap;

// 正解できてない
fn main() {
    input! {
        n: usize,
        mut events: [(i64, i64); n],
    }

    // イベントのベクターを、各商品が印字機の範囲に入る時間でソートします。
    // 同じ時間に複数の商品が到着する場合、印字機の範囲を後に離れる商品から処理します。
    events.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| b.1.cmp(&a.1)));

    let mut heap = BinaryHeap::new();
    let mut ans = 0;

    for (arrive, duration) in events {
        let leave = arrive + duration; // 商品が印字機の範囲を離れる時間

        // 商品が印字機の範囲に入る前に、印字可能な前の商品をチェックします。
        while let Some(&end_time) = heap.peek() {
            if end_time <= arrive { // 印字機が利用可能であれば印字を行い、heapから取り除きます。
                heap.pop();
            } else {
                break;
            }
        }

        heap.push(leave); // 現在の商品の出発時間をheapに追加します。
        ans = ans.max(heap.len()); // 最大同時印字数を更新します。
    }

    println!("{}", ans);
}
