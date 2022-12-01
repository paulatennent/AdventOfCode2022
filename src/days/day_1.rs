use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn get_calories(s: String) -> Vec<i32> {
    s.split("\n\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| {
            s.split("\n")
                .collect::<Vec<&str>>()
                .iter()
                .filter_map(|amount| amount.parse::<i32>().ok())
                .sum()
        })
        .collect::<Vec<i32>>()
}

pub fn day_1() {
    let path = Path::new("input/1.in");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => {}
    }

    let mut calories = get_calories(s);

    println!("Part 1: {}", calories.iter().max().unwrap());

    calories.sort_by(|a, b| b.cmp(a));
    let sum_top_3: i32 = calories.iter().take(3).sum();

    println!("Part 2: {}", sum_top_3);
}
