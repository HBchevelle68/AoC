use std::collections::HashMap;

fn main() {
    println!("Day1 AoC 2023!");

    let data1 = std::fs::read_to_string("data.txt").expect("Failed to open file");
    part_1(data1);

    let data2 = std::fs::read_to_string("data.txt").expect("Failed to open file");
    part_2(data2);
}

fn part_1(data: String) {
    let mut total = 0;
    let max: HashMap<&str, usize> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    for line in data.lines() {
        let mut valid = true;
        let split_line: Vec<String> = split_game_data(line);
        let game_num = split_line[0].parse::<usize>().unwrap();
        let game = split_cube_data(split_line[1].to_string());

        for cubes in game {
            let vals: Vec<_> = cubes.split_ascii_whitespace().collect();
            if vals[0].parse::<usize>().unwrap() > *max.get(vals[1]).unwrap() {
                valid = false;
                break;
            }
        }
        if valid {
            total += game_num;
        }
    }
    println!("Total: {}", total)
}

// I know there is too much code re-use but I'm being lazy and just want
// to complete this one
fn part_2(data: String) {
    let mut total = 0;

    for line in data.lines() {
        let mut least_needed: HashMap<&str, usize> =
            HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

        let split_line: Vec<String> = split_game_data(line);
        let game = split_cube_data(split_line[1].to_string());

        for cubes in game {
            let vals: Vec<_> = cubes.split_ascii_whitespace().collect();
            let converted = vals[0].parse::<usize>().unwrap();
            if converted > *least_needed.get(vals[1]).unwrap() {
                *least_needed.get_mut(vals[1]).unwrap() = converted;
            }
        }
        let mut power = 1;
        for v in least_needed.values() {
            power *= v;
        }
        total += power;
    }
    println!("Total: {}", total)
}

fn split_game_data(gd: &str) -> Vec<String> {
    gd.split(':')
        .map(|s| s.replace("Game", ""))
        .map(|s| s.trim().to_owned())
        .collect()
}

fn split_cube_data(cd: String) -> Vec<String> {
    let cubes = cd.replace(';', ",");
    cubes.split(',').map(|s| s.trim().to_owned()).collect()
}
