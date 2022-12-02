use std::fs;

fn main() {
    let data =
        fs::read_to_string("/home/ap/MyRepos/AoC22/day1/test.txt").expect("Unable to read file");

    let split = data.split("\n");

    let mut total: u32 = 0;
    let mut top_total: u32 = 0;
    //let mut vec_totals: Vec<u32> = vec![];
    for num in split {
        if num.is_empty() {
            if top_total < total {
                top_total = total;
            }
            total = 0;
            continue;
        }

        println!("Adding {} to {}", num, total);
        total += num.parse::<u32>().unwrap();
    }

    println!("Top total {}", top_total);
}
