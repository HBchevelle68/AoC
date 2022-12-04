use std::fs;

fn main() {
    let data = fs::read_to_string("day1_input.txt").expect("Failed to open file");

    let mut curr_total: u32 = 0;
    let mut top_total: u32 = 0;
    let mut elf_id: u32 = 1;

    // .0 is elf_id
    // .1 is total
    let mut vec_totals: Vec<(u32, u32)> = vec![];

    for num in data.lines() {
        // Found empty line
        if num.is_empty() {
            if top_total < curr_total {
                top_total = curr_total;
            }
            vec_totals.push((elf_id, curr_total));
            elf_id += 1;
            curr_total = 0;
            continue;
        }

        curr_total += num.parse::<u32>().unwrap();
    }

    println!("Part 1 total {}", top_total);
    vec_totals.sort_by_key(|&w| std::cmp::Reverse(w.1));

    println!(
        "Part 2 top 3 {}",
        vec_totals[0].1 + vec_totals[1].1 + vec_totals[2].1
    );
}
