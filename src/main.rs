mod days {
    pub mod day_1;
    pub mod day_2;
    pub mod day_3;
}

use std::io::{self, BufRead};

fn main() {
    print!("\n\n");
    for _ in 0..34 { print!("-"); }
    println!("\n\n >> Rust: Advent of Code 2022 << \n");
    for _ in 0..34 { print!("-"); }
    print!("\n\n");

    // ------------------------------
    let day = 1;
    use days::day_1 as puzzle;
    // ------------------------------

    let path = format!("./data/day_{}.txt", day);
    let file = std::fs::File::open(&path).unwrap_or_else(|_| panic!("\n[AOC] Error opening file\n"));
    let reader = io::BufReader::new(file);
    
    let data = reader
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();
    let test_cases: Vec<Vec<String>> = data
        .split(|str| *str == ">>TEST_END<<".to_string())
        .map(|str| str.to_vec())
        .collect::<Vec<Vec<String>>>();

    let mut console_input = String::new();
    std::io::stdin().read_line(&mut console_input).unwrap();
    let test_case = console_input.to_lowercase(); 

    let practice_case: &str = "t";
    let mut puzzle_input: Vec<String>;
    if test_case.contains(practice_case) { 
        puzzle_input = test_cases[0].clone();
    } else { 
        puzzle_input = test_cases[1].clone(); 
    }
    
    println!("Day {}, Part 1: {}", day, puzzle::part1(&mut puzzle_input));
    println!("Day {}, Part 2: {}", day, puzzle::part2(&mut puzzle_input));
}
