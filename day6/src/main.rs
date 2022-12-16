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

fn find_msg(data: &String, num: usize) {
    let len = data.len();
    let mut i = 0;
    while (i + num) < len {
        let chunk = &data[i..=(i + num)];
        match is_unique(&chunk) {
            None => break,
            Some((_, _, _)) => i += 1,
        }
    }
    if num == 3 {
        println!("Part 1 -- first marker {}", i + 4);
    } else {
        println!("Part 2 -- Num processed before msg {}", i + 14);
    }
}

fn main() {
    println!("Day 6 AoC22!");

    let data = fs::read_to_string("day6_input.txt").expect("Failed to open file");
    find_msg(&data, 4 - 1);
    find_msg(&data, 14 - 1);
}
