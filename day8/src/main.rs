use std::{collections::HashSet, fs};

const TESTLEN: usize = 5;

fn testinput(data: &str) {
    let mut trees: [[u8; TESTLEN]; TESTLEN] = [[0; TESTLEN]; TESTLEN];
    let mut visable_trees: HashSet<(usize, usize)> = HashSet::new();

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

    // Start at [1][1] since all outside trees are
    // considered, visable
    for row in 1..(TESTLEN - 1) {
        for col in 1..(TESTLEN - 1) {
            // Left to right
            let mut vis = false;
            let tmp_tree_val = trees[row][col];
            let mut range_iter = 0..TESTLEN;

            println!("Evaluating TREE: [{}][{}] = {}", row, col, tmp_tree_val);

            while let Some(tmpcol) = range_iter.next() {
                println!(
                    "[{}][{}] > [{}][{}] ---- {} > {}",
                    row, col, row, tmpcol, tmp_tree_val, trees[row][tmpcol]
                );
                if tmpcol == col {
                    // Same index
                    match vis {
                        true => {
                            // Getting to a true means all to the left
                            // are smaller and the tree is visable

                            // Nothing to to, fall through to insert
                            println!("TREE AT [{}][{}] VISABLE!", row, col);
                            break;
                        }
                        false => {
                            println!("not visable from left");
                            continue;
                        }
                    }
                } else if tmp_tree_val > trees[row][tmpcol] {
                    // Tree visable, continue on
                    vis = true;
                } else {
                    // Tree at [r][c] not visable
                    // set false and skip to c+1 to check on the right side
                    vis = false;
                    if tmpcol < col {
                        continue;
                    } else {
                        // We are on the right, which can only occur
                        // after we have already checked the left and
                        // did not have a true evaluation. time to move on
                        break;
                    }
                } // if
            } // while

            if vis == true {
                println!("Inserting [{}][{}]...", row, col);
                _ = visable_trees.insert((row, col));
            }

            println!();
        } // for
    }

    total += visable_trees.len();
    println!("TOTAL {}", total);
    dbg!(visable_trees);
}
fn main() {
    println!("Day 8 AoC22!");

    let data = fs::read_to_string("test_input.txt").expect("Failed to open file");

    testinput(&data);
}
