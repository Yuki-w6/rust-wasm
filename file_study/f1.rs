fn main () {
    let f = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .append(true)
    .open(path)?;
}