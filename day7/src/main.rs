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

    subfiles: Vec<FileInfo>,
}

impl FileInfo {
    fn new(name: String, size: u64, ftype: FileType) -> FileInfo {
        FileInfo {
            name,
            size,
            ftype,
            //parent,
            subfiles: vec![],
        }
    }
    pub fn change_directory(&mut self, dir: &str) {
        for (i, d) in self.subfiles.iter().enumerate() {
            if d.name == dir {}
        }
    }
    //pub fn add_file(&mut self) {}
}

// fn parse_cd(cmd: &str) {}

fn parse_input(data: &str, &mut file_sys: &mut FileInfo) {
    let split_data: Vec<&str> = data
        .split('$')
        .filter(|l| !l.is_empty())
        .map(|s| s.trim())
        .collect();

    dbg!(&split_data);
    for cmd in &split_data[1..] {
        if cmd.starts_with("cd") {
            let newdir = cmd.strip_prefix("cd").unwrap();

            file_sys.change_directory(&newdir);
        }
    }
}

fn part1(data: &str) {
    let mut part1_fs = FileInfo::new("/".into(), 0, FileType::Directory);
    parse_input(&data, &mut part1_fs);
}

fn main() {
    println!("Day 7 AoC22!");

    let data = fs::read_to_string("test_input.txt").expect("Failed to open file");

    //dbg!(&data);

    part1(&data);
}
