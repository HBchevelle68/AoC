use std::fs;

fn main() {
    println!("Day 5 AoC22!");

    let data = fs::read_to_string("test_input.txt").expect("Failed to open file");

    dbg!(data);
}
