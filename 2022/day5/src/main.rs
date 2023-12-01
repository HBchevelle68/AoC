use std::fs;
#[derive(Debug)]
struct Move {
    mov: usize,
    from: usize,
    to: usize,
}

fn parse_crates(data: &str) -> Vec<Vec<char>> {
    let mut split_data: Vec<&str> = data.rsplit('\n').collect();
    let num_col: usize = split_data[0].split(' ').filter(|c| !c.is_empty()).count();

    split_data.remove(0);
    let mut crates: Vec<Vec<char>> = vec![vec![]; num_col];

    // Add crates to proper vec
    for row in split_data {
        for (idx, col) in (1..((num_col * 3) + (num_col - 1))).step_by(4).enumerate() {
            let c = row.chars().nth(col).unwrap();

            if c.is_ascii_alphabetic() {
                crates[idx].push(c);
            }
        }
    }
    crates
}

fn parse_instructions(data: &str) -> Vec<Move> {
    let mut instructions: Vec<Move> = vec![];
    for line in data.split('\n') {
        let nums: Vec<usize> = line
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

fn perform_part1_instructions(inst: &Vec<Move>, mut crates: Vec<Vec<char>>) {
    // Execute instructions
    for instruction in inst {
        for _ in 0..instruction.mov {
            let tmp = crates[instruction.from - 1].pop().unwrap();
            crates[instruction.to - 1].push(tmp);
        }
    }

    println!(
        "Part 1 -- {}",
        crates
            .iter()
            .map(|inner| inner.last().unwrap())
            .collect::<String>()
    );
}

fn perform_part2_instructions(inst: &Vec<Move>, mut crates: Vec<Vec<char>>) {
    // Execute instructions
    for instruction in inst {
        // There is probably a better way to do this...
        let mut tmp: Vec<char> = vec![];
        for _ in 0..instruction.mov {
            tmp.push(crates[instruction.from - 1].pop().unwrap());
        }
        for x in tmp.iter().rev() {
            crates[instruction.to - 1].push(x.to_owned());
        }
    }

    println!(
        "Part 2 -- {}",
        crates
            .iter()
            .map(|inner| inner.last().unwrap())
            .collect::<String>()
    );
}
fn main() {
    println!("Day 5 AoC22!");

    let data = fs::read_to_string("day5_input.txt").expect("Failed to open file");

    let split_data: Vec<String> = data.split("\n\n").map(|s| s.to_string()).collect();
    let crates = split_data[0].to_owned();
    let instructions = split_data[1].to_owned();

    let crates_p1 = parse_crates(&crates);
    let instructions_p1 = parse_instructions(&instructions);

    perform_part1_instructions(&instructions_p1, crates_p1);

    let crates_p2 = parse_crates(&crates);
    let instructions_p2 = parse_instructions(&instructions);
    perform_part2_instructions(&instructions_p2, crates_p2);
}
