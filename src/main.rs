mod days {
    pub mod day_1;
}

use std::io::{self, BufRead};
use days::*;




fn main() {
    // if let Ok(lines) = read_lines("./data/day_1.txt") {
    //     for line in lines.flatten() {
    //         println!("{}", line);
    //     }
    // }

    // let file = std::fs::File::open(path);
    // if file.is_err() {
    //     println!("Error opening file: {:?}", file);
    //     return;
    // }

    // if let Err(e) = std::fs::File::open(path) {
    //     println!("Error opening file: {:?}", e);
    //     return;
    // }

    print!("\n\n");
    for _ in 0..34 { print!("-"); }
    println!("\n\n >> Rust: Advent of Code 2022 << \n");
    for _ in 0..34 { print!("-"); }
    print!("\n\n");

    
    let day = 1;
    let path = format!("./src/data/day_{}.txt", day);
    let file = std::fs::File::open(&path).unwrap_or_else(|_| panic!("Error opening file"));
    let reader = io::BufReader::new(file);
    
    let mut lines = reader.lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();  
  
    println!("Day 1, Part 1: {}", day_1::part1(&mut lines));
    println!("Day 1, Part 2: {}", day_1::part2(&mut lines));
}
