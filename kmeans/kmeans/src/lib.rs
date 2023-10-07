use rand::Rng;

/// 各クラスタの平均ベクトルを求める
fn calc_means(data: &Vec<Vec<f64>>, k: usize, cluster_indices: &[usize]) -> Vec<Vec<f64>> {
    let n = data.len();
    let d = data[0].len();
    let mut means = vec![vec![0.0; d]; k];
    let mut counts = vec![0; k];
    for i in 0..n {
        let j = cluster_indices[i];
        for l in 0..d {
            means[j][l] += data[i][l];
        }
        counts[j] += 1;
    }
    for j in 0..k {
        if counts[j] == 0 {
            continue;
        }
        for l in 0..d {
            means[j][l] /= counts[j] as f64;
        }
    }
    means
}

/// 2つのベクトル間のユークリッド距離の2乗を計算する
fn calc_dist(x: &[f64], y: &[f64]) -> f64 {
    x.iter()
        .zip(y.iter())
        .map(|(xi, yi)| (xi - yi).powi(2))
        .sum()
}

/// ベクトルxに最も近い平均ベクトルのインデックスを取得する
fn get_min_index(x: &[f64], means: &[Vec<f64>]) -> usize {
    let mut min_dist = std::f64::MAX;
    let mut min_index = 0;
    for (j, m) in means.iter().enumerate() {
        let dist = calc_dist(x, m);
        if dist < min_dist {
            min_dist = dist;
            min_index = j;
        }
    }
    min_index
}

/// kmeans algorithm
fn _kmeans(data: Vec<Vec<f64>>, k: usize, num_steps: usize) -> (Vec<usize>, Vec<Vec<f64>>) {
    // init clusters
    let n = data.len();
    let mut rng = rand::thread_rng();
    let mut cluster_indices: Vec<usize> = (0..n).map(|_| rng.gen_range(0..k)).collect();
    let mut means = calc_means(&data, k, &cluster_indices);

    // traininig
    for _ in 0..num_steps {
        cluster_indices = data.iter().map(|x| get_min_index(x, &means)).collect();
        means = calc_means(&data, k, &cluster_indices);
    }
    (cluster_indices, means)
}

/// kmeansの結果をPythonに返すための構造体
#[repr(C)]
pub struct KmeansResult {
    pub cluster_indices: *const usize,
    pub means: *const f64,
}

/// # Safety
/// Pythonからデータを受け取ってkmeansを実行する
#[no_mangle]
pub unsafe extern "C" fn kmeans(
    array: *const f64,
    rows: usize,
    cols: usize,
    k: usize,
    num_steps: usize,
) -> *const KmeansResult {
    let a = unsafe { std::slice::from_raw_parts(array, rows * cols) };
    let data: Vec<Vec<f64>> = a.chunks(cols).map(|x| x.to_vec()).collect();
    let (cluster_indices, means) = _kmeans(data, k, num_steps);
    let means = means.concat();
    let kmeans_result = KmeansResult {
        cluster_indices: cluster_indices.as_ptr(),
        means: means.as_ptr(),
    };
    std::mem::forget(cluster_indices);
    std::mem::forget(means);
    Box::into_raw(Box::new(kmeans_result))
}

/// # Safety
/// KmeansResultのメモリを解放する
#[no_mangle]
pub unsafe extern "C" fn free_kmeans_result(kmeans_result: *mut KmeansResult) {
    let _ = Box::from_raw(kmeans_result);
}

/// # Safety
/// *const f64 のメモリを解放する
#[no_mangle]
pub unsafe extern "C" fn free_f64_array(array: *mut f64, len: usize) {
    let _ = std::slice::from_raw_parts(array, len);
}

/// # Safety
/// *const usize のメモリを解放する
#[no_mangle]
pub unsafe extern "C" fn free_usize_array(array: *mut usize, len: usize) {
    let _ = std::slice::from_raw_parts(array, len);
}