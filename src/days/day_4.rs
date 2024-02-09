#![allow(dead_code, unused_variables)]

pub fn part1(puzzle_input: &mut Vec<String>) -> i32 {
    let mut sum: i32 = 0;
    
    let assignment_pairs: Vec<i32> = puzzle_input
        .iter()
        .split(|x| x == &',') // each region
        .map(|x| {
            x.iter()
                .split(|y| y == &'-')
                .map(|y| y as u32)
                .collect(); // get start & end
        })
        .collect();

    let overlapping_pairs: i32 = assignment_pairs
        .iter();
        // .collect();

    return overlapping_pairs;
}

pub fn part2(puzzle_input: &mut Vec<String>) -> i32 {
    

    return 5;
}