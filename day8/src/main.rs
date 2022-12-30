#![feature(iter_advance_by)]
use std::{collections::HashSet, fs};

const MAXLEN: usize = 99;

macro_rules! if_vis_break_or_continue {
    ($vis : expr) => {
        match $vis {
            true => {
                break;
            }
            false => {
                continue;
            }
        }
    };
}

fn part1(data: &str) {
    let mut trees: [[u8; MAXLEN]; MAXLEN] = [[0; MAXLEN]; MAXLEN];
    let mut visable_trees: HashSet<(usize, usize)> = HashSet::new();

    for (r, line) in data.lines().map(|l| l.trim()).enumerate() {
        for (c, num) in line
            .chars()
            .map(|cell| cell.to_digit(10).unwrap() as u8)
            .enumerate()
        {
            trees[r][c] = num;
        }
    }
    let mut total = (MAXLEN * 2) + ((MAXLEN - 2) * 2);

    // Start at [1][1] since all outside trees are
    // considered, visable
    for row in 1..(MAXLEN - 1) {
        for col in 1..(MAXLEN - 1) {
            let mut vis = false;
            let tmp_tree_val = trees[row][col];
            let mut left_right_iter = 0..MAXLEN;
            let mut top_bottom_iter = left_right_iter.clone();

            while let Some(tmpcol) = left_right_iter.next() {
                // Left to right
                if tmpcol == col {
                    // Same index
                    if_vis_break_or_continue!(vis);
                } else if tmp_tree_val > trees[row][tmpcol] {
                    // Tree visable, continue on
                    vis = true;
                } else {
                    // Tree at [r][c] not visable
                    // set false and skip to c+1 to check on the right side
                    vis = false;
                    if tmpcol < col {
                        // We are on the "left" of [r][c]
                        _ = left_right_iter.advance_by(col - tmpcol);
                        continue;
                    } else {
                        // We are on the right, which can only occur
                        // after we have already checked the left and
                        // did not have a true evaluation. time to move on
                        break;
                    }
                }
            }

            if vis == true {
                _ = visable_trees.insert((row, col));
                continue;
            }

            while let Some(tmprow) = top_bottom_iter.next() {
                // top to bottom
                if tmprow == row {
                    // Same index
                    if_vis_break_or_continue!(vis);
                } else if tmp_tree_val > trees[tmprow][col] {
                    // Tree visable, continue on
                    vis = true;
                } else {
                    // Tree at [r][c] not visable
                    // set false and skip to r+1 to check on the right side
                    vis = false;
                    if tmprow < row {
                        // We are "above" [r][c]
                        _ = top_bottom_iter.advance_by(row - tmprow);
                        continue;
                    } else {
                        // We are on the right, which can only occur
                        // after we have already checked the left and
                        // did not have a true evaluation. time to move on
                        break;
                    }
                }
            }
            if vis == true {
                _ = visable_trees.insert((row, col));
                continue;
            }
        }
    }

    total += visable_trees.len();
    println!("Part 1 TOTAL {}", total);
}

fn part2(data: &str) {
    let mut trees: [[u8; MAXLEN]; MAXLEN] = [[0; MAXLEN]; MAXLEN];
    //let mut visable_trees: HashSet<(usize, usize)> = HashSet::new();

    let mut best_tree: ((usize, usize), usize) = ((0, 0), 0);

    for (r, line) in data.lines().map(|l| l.trim()).enumerate() {
        for (c, num) in line
            .chars()
            .map(|cell| cell.to_digit(10).unwrap() as u8)
            .enumerate()
        {
            trees[r][c] = num;
        }
    }

    for row in 1..(MAXLEN - 1) {
        for col in 1..(MAXLEN - 1) {
            //let tmp_tree_val = trees[row][col];
            let up_iter = (0..row).rev();
            let down_iter = (row + 1)..MAXLEN;
            let left_iter = (0..col).rev();
            let right_iter = (col + 1)..MAXLEN;

            let mut up = 0;
            let mut down = 0;
            let mut left = 0;
            let mut right = 0;

            // up
            for up_row in up_iter {
                up += 1;
                if trees[row][col] <= trees[up_row][col] {
                    break;
                }
            }
            // down
            for down_row in down_iter {
                down += 1;
                if trees[row][col] <= trees[down_row][col] {
                    break;
                }
            }
            // left
            for left_col in left_iter {
                left += 1;
                if trees[row][col] <= trees[row][left_col] {
                    break;
                }
            }
            // right
            for right_col in right_iter {
                right += 1;
                if trees[row][col] <= trees[row][right_col] {
                    break;
                }
            }

            if (up * down * left * right) > best_tree.1 {
                best_tree = ((row, col), (up * down * left * right));
            }
        }
    }

    println!(
        "Part 2 Best Tree: [{}] [{}] with value {}",
        best_tree.0 .0, best_tree.0 .0, best_tree.1
    );
}
fn main() {
    println!("Day 8 AoC22!");

    let data = fs::read_to_string("day8_input.txt").expect("Failed to open file");

    part1(&data);
    part2(&data);
}
