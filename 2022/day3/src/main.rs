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

fn part2(data: &String) {
    let mut vec_lines: Vec<&str> = data.lines().filter(|line| !line.is_empty()).collect();

    let mut total = 0;
    while !vec_lines.is_empty() {
        let e1 = vec_lines.pop().unwrap();
        let e2 = vec_lines.pop().unwrap();
        let e3 = vec_lines.pop().unwrap();

        for c in e1.chars() {
            if e2.contains(c) && e3.contains(c) {
                total += match c.is_lowercase() {
                    true => c as u32 - LOWERADJ,
                    false => c as u32 - UPPERADJ,
                };
                break;
            }
        }
    }
    println!("Part 2 total: {}", total);
}

fn main() {
    println!("Day 2 AoC22 !");

    let data = fs::read_to_string("day3_input.txt").expect("Failed to open file");

    part1(&data);

    let data = fs::read_to_string("day3_input2.txt").expect("Failed to open file");

    part2(&data);
}
