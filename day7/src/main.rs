use std::{fs, string};

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
    parent: Option<&'a FileInfo<'a>>,
    subfiles: Vec<FileInfo<'a>>,
}

impl<'a> FileInfo<'a> {
    fn new(name: String, size: u64, ftype: FileType, parent: Option<&'a FileInfo<'a>>) -> FileInfo {
        FileInfo {
            name: name,
            size: size,
            ftype: ftype,
            parent: parent,
            subfiles: vec![],
        }
    }
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
