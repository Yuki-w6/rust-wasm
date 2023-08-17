fn use_value(_val: i32) {

}

fn use_value2(_val: Demo) {

}

struct Demo {
    a: i32
}

fn main() {
    let demo = Demo{a :123};
    use_value2(demo);

    println!("{}", demo.a);
}