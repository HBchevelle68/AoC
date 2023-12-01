use std::collections::HashMap;
use std::fs;

// 1st Column
// A - Rock
// B - Paper
// C - Scissors
//
// 2nd Column
// X - Rock - 1pt
// Y - Paper - 2pts
// Z - Scissors - 3pts
//
// Lose - 0 pts
// Draw - 3 pts
// Win  - 6 pts
fn part1(data: &String) {
    let mut statemap: HashMap<&str, u32> = HashMap::new();
    statemap.insert("A X", 4);
    statemap.insert("A Y", 8);
    statemap.insert("A Z", 3);

    statemap.insert("B X", 1);
    statemap.insert("B Y", 5);
    statemap.insert("B Z", 9);

    statemap.insert("C X", 7);
    statemap.insert("C Y", 2);
    statemap.insert("C Z", 6);

    let total: u32 = data.lines().map(|l| statemap.get(l).unwrap()).sum();
    println!("Part 1 Total: {}", total);
}

// 1st Column
// A - Rock
// B - Paper
// C - Scissors
//
// 2nd Column
// X - need to lose
// Y - need to draw
// Z - need to win
//
// Rock - 1pt
// Paper - 2pts
// Scissors - 3pts
//
// Lose - 0 pts
// Draw - 3 pts
// Win  - 6 pts
fn part2(data: &String) {
    let mut statemap: HashMap<&str, u32> = HashMap::new();
    statemap.insert("A X", 3);
    statemap.insert("A Y", 4);
    statemap.insert("A Z", 8);

    statemap.insert("B X", 1);
    statemap.insert("B Y", 5);
    statemap.insert("B Z", 9);

    statemap.insert("C X", 2);
    statemap.insert("C Y", 6);
    statemap.insert("C Z", 7);

    let total: u32 = data.lines().map(|l| statemap.get(l).unwrap()).sum();
    println!("Part 2 Total: {}", total);
}

fn main() {
    println!("Day 2 AoC22 !");

    let data = fs::read_to_string("day2_input.txt").expect("Failed to open file");

    part1(&data);
    part2(&data);
}
