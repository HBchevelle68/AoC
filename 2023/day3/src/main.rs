fn main() {
    println!("Day1 AoC 2023!");

    let data1 = std::fs::read_to_string("data.txt").expect("Failed to open file");
    part_1(data1);

    // let data2 = std::fs::read_to_string("data.txt").expect("Failed to open file");
    // part_2(data2);
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

// fn part_2(data: String) {}
