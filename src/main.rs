mod days {
    pub mod day_1;
    pub mod day_2;
    pub mod day_3;
}

use std::io::{self, BufRead};
trait StringUtils {
    fn substring(&self, start: usize, len: usize) -> Self;
}

impl StringUtils for String {
    fn substring(&self, start: usize, len: usize) -> Self {
        self.chars().skip(start).take(len).collect()
    }
}

fn main() {
    print!("\n\n");
    for _ in 0..34 { print!("-"); }
    println!("\n\n >> Rust: Advent of Code 2022 << \n");
    for _ in 0..34 { print!("-"); }
    print!("\n\n");

    // ------------------------------
    let day = 3;
    use days::day_3 as puzzle;
    // ------------------------------

    let path = format!("./src/data/day_{}.txt", day);
    let file = std::fs::File::open(&path).unwrap_or_else(|_| panic!("\n[AOC] Error opening file\n"));
    let reader = io::BufReader::new(file);
    
    let mut lines = reader
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();  

    println!("Day {}, Part 1: {}", day, puzzle::part1(&mut lines));
    println!("Day {}, Part 2: {}", day, puzzle::part2(&mut lines));
}
