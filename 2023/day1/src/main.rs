fn main() {
    println!("Day1 AoC 2023!");

    let data1 = std::fs::read_to_string("data.txt").expect("Failed to open file");
    part_1(data1);

    let data2 = std::fs::read_to_string("data_2.txt").expect("Failed to open file");
    part_2(data2);
}

fn part_1(data: String) {
    let mut res: Vec<u32> = vec![];
    for line in data.lines() {
        let num: Vec<_> = line.chars().filter(|c| c.is_digit(10)).collect();
        let mut combo = String::from(num[0]);
        combo.push(*num.last().unwrap());
        res.push(combo.parse::<u32>().unwrap())
    }
    println!("{}", res.iter().sum::<u32>())
}

fn part_2(data: String) {
    // Iterate all lines from data
    let mut res = 0_u32;
    for line in data.lines() {
        let only_digits: Vec<u32> = iter_substrings(line)
            .filter_map(|sub| convert(sub))
            .collect();
        res += only_digits.first().unwrap() * 10 + only_digits.last().unwrap();
        println!("Orig: {} New: {:?}", line, only_digits);
    }
    println!("Part 2 Total: {}", res);
}

fn convert(s: &str) -> Option<u32> {
    let matched = match s {
        s if s.starts_with("one") => Some(1),
        s if s.starts_with("two") => Some(2),
        s if s.starts_with("three") => Some(3),
        s if s.starts_with("four") => Some(4),
        s if s.starts_with("five") => Some(5),
        s if s.starts_with("six") => Some(6),
        s if s.starts_with("seven") => Some(7),
        s if s.starts_with("eight") => Some(8),
        s if s.starts_with("nine") => Some(9),
        _ => None,
    };
    if matched.is_some() {
        return matched;
    }
    s.chars().next().and_then(|c| c.to_digit(10))
}

fn iter_substrings(s: &str) -> impl Iterator<Item = &str> {
    (0..s.len()).map(|offset| &s[offset..])
}
