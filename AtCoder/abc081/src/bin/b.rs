use proconio::input;

fn main() {
    input!{
        n: usize,
        a: [u64; n]
    }
    let mut target = a;
    let mut count = 0;
    while isAllEven(target) {
        target = devide(target)
        count += 1;
    }
    println!({}, count);
}

fn isAllEven(array: Vec) {
    let mut result = true;
    for i in array {
        if i % 2 != 0 {
            result = false;
            break;
        }
    }
    result
}

fn devide(array: Vec) {
    let mut result: Vec;
    for i, num in result.enumerate() {
        result[i] = num / 2;
    }
    result
}
 