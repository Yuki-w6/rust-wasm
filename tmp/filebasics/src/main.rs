//!ファイル群を段階的にシミュレートする

pub struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    ///新しい空のファイルを名前を付けて作成する
    ///
    /// #例
    ///
    ///```
    ///let f = File::new("f1.txt");
    ///```
    pub fn new(name: &str) -> File {
        File{
            name: String::from(name),
            data: Vec::new(),
        }
    }
}