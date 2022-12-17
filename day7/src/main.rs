use std::fs;

#[derive(Debug)]
enum FileType {
    File,
    Directory,
}

#[derive(Debug)]
struct FileInfo<'a> {
    name: String,
    size: u64,
    ftype: FileType,
    parent: &'a FileInfo<'a>,
    subfiles: Vec<FileInfo<'a>>,
}

impl<'a> FileInfo<'a> {
    fn change_directory(self, dir: &str) -> Option<&FileInfo> {
        None
    }
    fn add_file(&mut self) {}
}

fn main() {
    println!("Day 7 AoC22!");

    let data = fs::read_to_string("test_input.txt").expect("Failed to open file");

    dbg!(&data);
}
