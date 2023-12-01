fn main() {
    println!("Day1 AoC 2023!");

    let data = std::fs::read_to_string("data.txt").expect("Failed to open file");
    part_1(data);
}

fn part_1(data: String) {
    let mut res: Vec<u32> = vec![];
    for line in data.lines() {
        let num: Vec<_> = line.chars().filter(|c| c.is_digit(10)).collect();
        let mut combo = String::from(num[0]);
        combo.push(*num.last().unwrap());
        println!("{:?}", combo.parse::<u32>());
        res.push(combo.parse::<u32>().unwrap())
    }
    println!("{}", res.iter().sum::<u32>())
}
