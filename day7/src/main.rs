use std::fs;

#[derive(Debug)]
enum FileType {
    File,
    Directory,
}

#[derive(Debug)]
struct FileInfo {
    name: String,
    size: u64,
    ftype: FileType,
    parent:
    subfiles: Vec<FileInfo>,

}


impl FileInfo {
    fn change_dir(&self, dir: &str) -> FileInfo{}
}


fn main() {
    println!("Day 7 AoC22!");

    let data = fs::read_to_string("test_input.txt").expect("Failed to open file");

    dbg!(&data);
}
