use std::collections::HashMap;

fn main() {
    println!("Day1 AoC 2023!");

    let data1 = std::fs::read_to_string("data.txt").expect("Failed to open file");
    part_1(data1);

    let data2 = std::fs::read_to_string("data.txt").expect("Failed to open file");
    part_2(data2);
}

// lazy init
const INIT: usize = 999999;

fn find_next_number(line: &str, sidx: usize) -> Option<(usize, usize, usize)> {
    let mut num: Vec<char> = vec![];
    let mut start = INIT;
    let mut end = INIT;
    let diff = sidx;

    // Bounds check
    if sidx >= line.len() {
        return None;
    }

    for (i, c) in line[sidx..].char_indices() {
        if c.is_ascii_digit() {
            if start == INIT {
                start = diff + i;
            }
            num.push(c);
        } else if start != INIT {
            // Not a digit and we already parsed a number
            end = diff + i - 1;
            let num_conv: String = num.into_iter().collect();
            return Some((num_conv.parse::<usize>().unwrap(), start, end));
        }
    }
    // Can occur if the number is the last part of the line
    if start != INIT && end == INIT {
        end = start;
        let num_conv: String = num.iter().collect();
        return Some((num_conv.parse::<usize>().unwrap(), start, end));
    }
    None
}

fn search_sym(sstart: usize, send: usize, dv_idx: usize, data_vec: &Vec<&str>) -> bool {
    let mut si = 0;
    let mut ei = 0;

    if dv_idx != 0 {
        si = dv_idx - 1;
    }
    if dv_idx + 1 < data_vec.len() {
        ei = dv_idx + 1;
    } else {
        ei = dv_idx;
    }

    for line in data_vec[si..=ei].iter() {
        for c in line[sstart..=send].chars() {
            if !c.is_ascii_digit() && c != '.' {
                return true;
            }
        }
    }

    false
}

fn part_1(data: String) {
    let data_vec: Vec<&str> = data.lines().collect();
    let mut sum = 0;
    for (idx, line) in data_vec.iter().enumerate() {
        let mut curr = 0;

        while curr <= line.len() - 1 {
            let mut search_start = 0_usize;
            let mut search_end = 0_usize;

            match find_next_number(line, curr) {
                Some((num, start, end)) => {
                    curr = end + 1;
                    if start == 0 {
                        search_start = start;
                    } else {
                        search_start = start - 1;
                    }
                    if end == line.len() - 1 {
                        search_end = end;
                    } else {
                        search_end = end + 1;
                    }
                    let found = search_sym(search_start, search_end, idx, &data_vec);
                    if found {
                        sum += num;
                    }
                }
                None => break,
            }
        }
    }
    println!("Sum: {}", sum);
}

// (row, column_num_start, column_num_end)
const P2INIT: (usize, usize, usize) = (0, 0, 0);

// This has so much code reuse and code that should be condensed
// it actually hurts me a little. But this problem sucks and I just
// want it to be over....

fn part_2(data: String) {
    let mut data_vec: Vec<Vec<char>> = vec![];
    let mut len = 0;

    let mut results: HashMap<(usize, usize), Vec<usize>> = HashMap::new();

    for line in data.lines() {
        if len == 0 {
            len = line.len();
        }
        let mut tmp = vec![];
        for c in line.chars() {
            tmp.push(c);
        }
        data_vec.push(tmp);
    }

    let mut nums_found: Vec<(usize, usize, usize)> = vec![];
    let mut num = P2INIT;
    let mut found = false;

    // Find numbers
    for r in 0..140 {
        // println!("Searching row {} = {:?}", r, &data_vec[r]);
        for c in 0..140 {
            if data_vec[r][c].is_ascii_digit() {
                // Found number
                // println!("{} is a number", data_vec[r][c]);
                if num == P2INIT && !found {
                    // This will account for if the number is the very end
                    // of the vector without the need for another conditional
                    found = true;
                    // println!("num before={:?} Found start at [{},{}]", &num, r, c);
                    num = (r, c, c);
                    // println!(" num after={:?}", num);
                }
            } else if found {
                // println!(" {} is NOT a number", data_vec[r][c]);
                // Not a digit && a num was previously found, reached end of number
                // only updated the column_num_end member of the tuple
                // println!("  num={:?} Found end at [{},{}]", &num, r, c);
                num.2 = c - 1;
                nums_found.push(num);
                num = P2INIT;
                found = false;
            }
        }
    }
    // dbg!(&nums_found);

    for val in nums_found {
        let mut sstart = val.1;
        let mut send = val.2;
        let mut found = false;

        if sstart != 0 {
            sstart -= 1;
        }
        if send != (len - 1) {
            send += 1
        }

        println!("Range is {}..{}", sstart, send);
        // search above
        if val.0 != 0 {
            // println!("Above");
            let mut tmp = sstart;
            while tmp <= send {
                //println!("  Row-{} checking {}", val.0 - 1, &data_vec[val.0 - 1][tmp]);
                if data_vec[val.0 - 1][tmp] == '*' {
                    println!("  Found * at [{},{}]", val.0 - 1, tmp);

                    let tmp_num: String = data_vec[val.0][val.1..=val.2]
                        .to_vec()
                        .into_iter()
                        .collect();
                    let tmp_res = tmp_num.parse::<usize>().unwrap();

                    results
                        .entry((val.0 - 1, tmp))
                        .and_modify(|r| r.push(tmp_res))
                        .or_insert_with(|| vec![tmp_res]);

                    found = true;
                    break;
                }
                tmp += 1;
            }
            if found {
                continue;
            }
        }
        // search below
        if val.0 != (len - 1) {
            // println!("Below");
            let mut tmp = sstart;
            while tmp <= send {
                if data_vec[val.0 + 1][tmp] == '*' {
                    println!("  Found * at [{},{}]", val.0 + 1, tmp);

                    let tmp_num: String = data_vec[val.0][val.1..=val.2]
                        .to_vec()
                        .into_iter()
                        .collect();
                    let tmp_res = tmp_num.parse::<usize>().unwrap();

                    results
                        .entry((val.0 + 1, tmp))
                        .and_modify(|r| r.push(tmp_res))
                        .or_insert_with(|| vec![tmp_res]);

                    found = true;
                    break;
                }
                tmp += 1;
            }
            if found {
                continue;
            }
        }
        // search left and right
        if val.1 != 0 {
            // println!("Left");
            if data_vec[val.0][val.1 - 1] == '*' {
                println!("  Found * at [{},{}]", val.0, val.1 - 1);

                let tmp_num: String = data_vec[val.0][val.1..=val.2]
                    .to_vec()
                    .into_iter()
                    .collect();
                let tmp_res = tmp_num.parse::<usize>().unwrap();

                results
                    .entry((val.0, val.1 - 1))
                    .and_modify(|r| r.push(tmp_res))
                    .or_insert_with(|| vec![tmp_res]);

                found = true;
                continue;
            }
        }
        if val.2 != (len - 9) {
            // println!("Right");
            if data_vec[val.0][val.2 + 1] == '*' {
                println!("  Found * at [{},{}]", val.0, val.2 + 1);

                let tmp_num: String = data_vec[val.0][val.1..=val.2]
                    .to_vec()
                    .into_iter()
                    .collect();
                let tmp_res = tmp_num.parse::<usize>().unwrap();

                results
                    .entry((val.0, val.2 + 1))
                    .and_modify(|r| r.push(tmp_res))
                    .or_insert_with(|| vec![tmp_res]);

                found = true;
                continue;
            }
        }
    }
    // dbg!(&results);
    let mut total = 0;
    for (k, v) in results {
        if v.len() == 2 {
            total += v[0] * v[1];
        }
    }

    println!("Day 2 total {}", total);
}
