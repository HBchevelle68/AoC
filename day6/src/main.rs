use std::{fs, string};

fn is_unique(s: &str) -> Option<(usize, usize, char)> {
    s.chars().enumerate().find_map(|(i, c)| {
        s.chars()
            .enumerate()
            .skip(i + 1)
            .find(|(_, other)| c == *other)
            .map(|(j, _)| (i, j, c))
    })
}

fn part1(data: &String) {
    let len = data.len();
    let mut i = 0;
    while (i + 3) < len {
        let chunk = &data[i..=(i + 3)];
        match is_unique(&chunk) {
            None => break,
            Some((_, _, _)) => i += 1,
        }
    }
    println!("Part 1 -- first marker {}", i + 4);
}

fn main() {
    println!("Day 5 AoC22!");

    let data = fs::read_to_string("day6_input.txt").expect("Failed to open file");
    part1(&data);
}
