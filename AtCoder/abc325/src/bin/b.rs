use proconio::input;

fn main() {
    input! {
        n: usize,
        offices: [(usize, i32); n],
    }

    let mut work_hours = vec![];
    for &(w, x) in &offices {
        let start = (9 - x + 24) % 24;
        let end = (18 - x + 24) % 24;
        work_hours.push((w, start, end));
    }

    let mut max_attendance = 0;
    for hour in 0..24 {
        let mut attendance = 0;
        for &(w, start, end) in &work_hours {
            if start <= end {
                if hour >= start && hour + 1 <= end {
                    attendance += w;
                }
            } else {
                if hour >= start || hour + 1 <= end {
                    attendance += w;
                }
            }
        }
        max_attendance = max_attendance.max(attendance);
    }

    // 最大の参加者数を出力
    println!("{}", max_attendance);
}
