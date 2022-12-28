#![feature(iter_advance_by)]
use std::{collections::HashSet, fs};

const MAXLEN: usize = 99;

fn part1(data: &str) {
    let mut trees: [[u8; MAXLEN]; MAXLEN] = [[0; MAXLEN]; MAXLEN];
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
    let mut total = (MAXLEN * 2) + ((MAXLEN - 2) * 2);
    dbg!(&total);

    // Start at [1][1] since all outside trees are
    // considered, visable
    for row in 1..(MAXLEN - 1) {
        for col in 1..(MAXLEN - 1) {
            let mut vis = false;
            let tmp_tree_val = trees[row][col];
            let mut left_right_iter = 0..MAXLEN;
            let mut top_bottom_iter = left_right_iter.clone();

            println!("Evaluating TREE: [{}][{}] = {}", row, col, tmp_tree_val);

            while let Some(tmpcol) = left_right_iter.next() {
                // Left to right
                // println!(
                //     "[{}][{}] > [{}][{}] ---- {} > {}",
                //     row, col, row, tmpcol, tmp_tree_val, trees[row][tmpcol]
                // );
                if tmpcol == col {
                    // Same index
                    match vis {
                        true => {
                            // Getting to a true means all to the left
                            // are smaller and the tree is visable

                            // Nothing to to, fall through to insert
                            //println!("TREE AT [{}][{}] VISABLE!", row, col);
                            break;
                        }
                        false => {
                            //println!("not visable from left");
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
                        // We are on the "left" of [r][c]
                        // println!(
                        //     "skipping from [{}][{}] to [{}][{}]",
                        //     row,
                        //     tmpcol,
                        //     row,
                        //     col + 1
                        // );
                        _ = left_right_iter.advance_by(col - tmpcol);
                        //dbg!(&left_right_iter);
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
                println!();
                continue;
            }

            println!("--- TOP TO BOTTOM ---");

            while let Some(tmprow) = top_bottom_iter.next() {
                // top to bottom
                println!(
                    "[{}][{}] > [{}][{}] ---- {} > {}",
                    row, col, tmprow, col, tmp_tree_val, trees[tmprow][col]
                );

                if tmprow == row {
                    // Same index
                    match vis {
                        true => {
                            // Getting to a true means all above
                            // are smaller and the tree is visable

                            // Nothing to to, fall through to insert
                            println!("TREE AT [{}][{}] VISABLE!", row, col);
                            break;
                        }
                        false => {
                            println!("not visable from top");
                            continue;
                        }
                    }
                } else if tmp_tree_val > trees[tmprow][col] {
                    // Tree visable, continue on
                    vis = true;
                } else {
                    // Tree at [r][c] not visable
                    // set false and skip to c+1 to check on the right side
                    vis = false;
                    if tmprow < row {
                        // We are "above" [r][c]
                        println!(
                            "skipping from [{}][{}] to [{}][{}]",
                            tmprow,
                            col,
                            tmprow + 1,
                            col
                        );
                        _ = top_bottom_iter.advance_by(row - tmprow);
                        dbg!(&top_bottom_iter);
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
                println!();
                continue;
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

    let data = fs::read_to_string("day8_input.txt").expect("Failed to open file");

    part1(&data);
}
