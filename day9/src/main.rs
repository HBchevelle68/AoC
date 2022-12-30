use std::{collections::HashSet, fs};

macro_rules! add_unique {
    ($map:expr, $t:expr) => {
        _ = $map.insert($t);
    };
}

macro_rules! to_isize {
    ($i:expr) => {
        $i.trim().parse::<isize>().unwrap()
    };
}

fn part1(data: &str) {
    let mut unique_pos: HashSet<(isize, isize)> = HashSet::new();

    let mut _H: (isize, isize) = (0, 0);
    let mut _T: (isize, isize) = (0, 0);

    // Insert first position
    _ = unique_pos.insert(_T);

    add_unique!(unique_pos, _T);

    dbg!(&unique_pos);

    for h_mvmt in data.lines().map(|line| line.trim().split_at(1)) {
        match h_mvmt.0 {
            "R" => {
                // loop for h_mvmt.1 # of times
                // each step, check if T needs to move
                // each T movement, call add_unique!()
                //println!("TEST: {}", to_isize!(h_mvmt.1));

                for m in 0..to_isize!(h_mvmt.1) {
                    _H.1 += 1;

                    if _T.1.abs() - _H.1.abs() > 1 {
                        // move T right??
                    }
                }
            }
            "L" => {}
            "U" => {}
            "D" => {}
            _ => {}
        }
    }
}

fn main() {
    println!("Day 9 AoC22!");

    let data = fs::read_to_string("test_input.txt").expect("Failed to open file");

    part1(&data);
}
