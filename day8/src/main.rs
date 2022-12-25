use std::{
    collections::{HashMap, HashSet},
    fs,
};

const TESTLEN: usize = 5;

fn testinput(data: &str) {
    let mut trees: [[u8; TESTLEN]; TESTLEN] = [[0; TESTLEN]; TESTLEN];
    let mut visable_trees: HashSet<(u8, u8)> = HashSet::new();

    for (r, line) in data.lines().map(|l| l.trim()).enumerate() {
        dbg!(&line);
        for (c, num) in line
            .chars()
            .map(|cell| cell.to_digit(10).unwrap() as u8)
            .enumerate()
        {
            trees[r][c] = num;
        }
    }
    let mut total = (TESTLEN * 2) + ((TESTLEN - 2) * 2);
    dbg!(&total);

    for r in 1..(TESTLEN - 1) {
        for c in 1..(TESTLEN - 1) {

            
            // Left to right
            let vis = false;
            let tmp_tree_pos = (r, c);
            let tmp_tree_val = trees[r][c];

            for tc in 0..TESTLEN {

                if (r,tc) == tmp_tree_pos{
                    break;
                }
                if trees[r][tc] < tmp_tree_val
            }
            match vis {
                true => {_ = visable_trees.insert(tmp_tree_pos);}
            }
        }
    }
}
fn main() {
    println!("Day 8 AoC22!");

    let data = fs::read_to_string("test_input.txt").expect("Failed to open file");

    testinput(&data);
}
