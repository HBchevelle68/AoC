use std::fs;

const LOWERADJ: u32 = 96;
const UPPERADJ: u32 = 38;

fn part1(data: &String) {
    let mut total: u32 = 0;
    for line in data.lines() {
        let p1 = &line[0..(line.len() / 2)];
        let p2 = &line[(line.len() / 2)..];

        for c in p1.chars() {
            if p2.contains(c) {
                total += match c.is_lowercase() {
                    true => c as u32 - LOWERADJ,
                    false => c as u32 - UPPERADJ,
                };
                break;
            }
        }
    }

    println!("Part 1 total: {}", total);
}

fn main() {
    println!("Day 2 AoC22 !");

    let data = fs::read_to_string("day3_input.txt").expect("Failed to open file");

    part1(&data);

    let data = fs::read_to_string("test2_input.txt").expect("Failed to open file");

    let vec_lines: Vec<&str> = data.split('\n').filter(|line| !line.is_empty()).collect();

    dbg!(vec_lines);
}
