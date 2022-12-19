use std::{collections::HashMap, fs, path::PathBuf};

const TOTALDISKSPACE: u32 = 70000000;
const NEEDEDMIN: u32 = 30000000;

fn part1(data: &str) {
    let mut dir_sizes: HashMap<PathBuf, u32> = HashMap::new();
    let mut dir_hist: Vec<&str> = vec![];

    for line in data.lines() {
        if line.starts_with("$ ls") || line.starts_with("dir") {
            continue;
        }

        let split_line: Vec<_> = line.split_whitespace().collect();
        match split_line[..] {
            ["$", "cd", ".."] => {
                dir_hist.pop();
            }

            ["$", "cd", fname] => {
                dir_hist.push(fname);
            }

            [size, _name] => {
                let sz: u32 = size.parse().unwrap();
                for index in 0..dir_hist.len() {
                    let path = PathBuf::from_iter(&dir_hist[..=index]);
                    *dir_sizes.entry(path).or_insert(0) += sz;
                }
            }

            _ => {}
        }
    }
    println!(
        "Part 1: {}",
        &dir_sizes
            .into_values()
            .filter(|sz| *sz <= 100000)
            .sum::<u32>()
    );
}

fn part2(data: &str) {
    let mut dir_sizes: HashMap<PathBuf, u32> = HashMap::new();
    let mut dir_hist: Vec<&str> = vec![];

    for line in data.lines() {
        if line.starts_with("$ ls") || line.starts_with("dir") {
            continue;
        }

        let split_line: Vec<_> = line.split_whitespace().collect();
        match split_line[..] {
            ["$", "cd", ".."] => {
                dir_hist.pop();
            }

            ["$", "cd", fname] => {
                dir_hist.push(fname);
            }

            [size, _name] => {
                let sz: u32 = size.parse().unwrap();
                for index in 0..dir_hist.len() {
                    let path = PathBuf::from_iter(&dir_hist[..=index]);
                    *dir_sizes.entry(path).or_insert(0) += sz;
                }
            }

            _ => {}
        }
    }

    let available = TOTALDISKSPACE - dir_sizes.get(&PathBuf::from("/")).unwrap();

    println!(
        "Part 2: {}",
        &dir_sizes
            .into_values()
            .filter(|sz| available + sz >= NEEDEDMIN)
            .min()
            .unwrap()
    );
}
fn main() {
    println!("Day 7 AoC22!");

    let data = fs::read_to_string("day7_input.txt").expect("Failed to open file");

    //dbg!(&data);

    part1(&data);
    part2(&data);
}
