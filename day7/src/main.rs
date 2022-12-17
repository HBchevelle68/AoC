use std::{fs, string};

#[derive(Debug)]
enum FileType {
    File,
    Directory,
}

#[derive(Debug)]
struct FileInfo<'a> {
    name: &'a str,
    size: u64,
    ftype: FileType,
    parent: Option<&'a FileInfo<'a>>,
    subfiles: Vec<FileInfo<'a>>,
}

impl<'a> FileInfo<'a> {
    fn new(
        name: &str,
        size: u64,
        ftype: FileType,
        parent: Option<&'a FileInfo<'a>>,
    ) -> FileInfo<'a> {
        FileInfo {
            name: 'a name,
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

fn parse_command(cmd: &str) {}

fn parse_input(data: &str) {
    let split_data: Vec<&str> = data.split("$").filter(|l| !l.is_empty()).collect();

    for cmd in split_data {
        dbg!(cmd.trim());
    }
}

fn main() {
    println!("Day 7 AoC22!");

    let data = fs::read_to_string("test_input.txt").expect("Failed to open file");

    dbg!(&data);
    let part1 = FileInfo::new("/", 0, FileType::Directory, None);
    parse_input(&data);
}
