use std::fs;

fn part1(data: &String) {
    let mut total = 0;
    for line in data.lines() {
        let (left, right) = line.split_once(',').unwrap();

        let (e1_small, e1_big) = left.split_once('-').unwrap();
        let (e2_small, e2_big) = right.split_once('-').unwrap();

        let e1_small = e1_small.parse::<u32>().unwrap();
        let e1_big = e1_big.parse::<u32>().unwrap();
        let e2_small = e2_small.parse::<u32>().unwrap();
        let e2_big = e2_big.parse::<u32>().unwrap();

        if e1_small <= e2_small && e1_big >= e2_big {
            total += 1;
            continue;
        }
        if e2_small <= e1_small && e2_big >= e1_big {
            total += 1;
            continue;
        }
    }
    println!("Part 1 total: {}", total);
}

fn part2(data: &String) {
    let mut total = 0;
    for line in data.lines() {
        let (left, right) = line.split_once(',').unwrap();

        let (e1_small, e1_big) = left.split_once('-').unwrap();
        let (e2_small, e2_big) = right.split_once('-').unwrap();

        let e1_small = e1_small.parse::<u32>().unwrap();
        let e1_big = e1_big.parse::<u32>().unwrap();
        let e2_small = e2_small.parse::<u32>().unwrap();
        let e2_big = e2_big.parse::<u32>().unwrap();

        // Not elegant but fast
        if e1_small <= e2_small && e2_small <= e1_big
            || e1_small <= e2_big && e2_big <= e1_big
            || e2_small <= e1_small && e1_small <= e2_big
            || e2_small <= e1_big && e1_big <= e2_big
        {
            total += 1;
            continue;
        }
    }
    println!("Part 2 total: {}", total);
}
fn main() {
    println!("Day 4 AoC22!");

    let data = fs::read_to_string("day4_input.txt").expect("Failed to open file");
    part1(&data);
    part2(&data);
}
