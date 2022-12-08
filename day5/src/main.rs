use std::fs;
#[derive(Debug)]
struct Move {
    mov: u8,
    from: u8,
    to: u8,
}

fn parse_instructions(data: &str) -> Vec<Move> {
    let mut instructions: Vec<Move> = vec![];
    for line in data.split('\n') {
        let nums: Vec<u8> = line
            .replace("move", "")
            .replace("from", "")
            .replace("to", "")
            .trim()
            .split(' ')
            .filter(|n| !n.is_empty())
            .map(|n| n.parse().unwrap())
            .collect();

        instructions.push(Move {
            mov: nums[0],
            from: nums[1],
            to: nums[2],
        })
    }
    instructions
}
fn main() {
    println!("Day 5 AoC22!");

    let data = fs::read_to_string("test_input.txt").expect("Failed to open file");

    let split_data: Vec<String> = data.split("\n\n").map(|s| s.to_string()).collect();
    let crates = split_data[0].to_owned();
    let instructions = split_data[1].to_owned();

    dbg!(crates);
    dbg!(&instructions);

    let instructions = parse_instructions(&instructions);
    dbg!(instructions);
}
