fn greet_world() {
    println!("Hello, world!");
    
    let japan = "ハロー・ワールド";
    let regions = [japan];
    for region in regions.iter() {
        println!("{}", &region);
    }
}

fn main() {
    greet_world();
}