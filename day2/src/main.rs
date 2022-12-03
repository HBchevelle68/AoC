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

use std::collections::HashMap;
use std::fs;

fn main() {
    println!("Day 2 AoC22 !");

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

    let data = fs::read_to_string("/home/ap/MyRepos/AoC22/day2/day2_input.txt")
        .expect("Failed to open file");

    let mut total: u32 = 0;
    for line in data.lines() {
        //println!("{}", line);
        total += statemap.get(line).unwrap();
    }
    println!("Total: {}", total);
}
