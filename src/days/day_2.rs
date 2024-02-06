#![allow(dead_code, unused_variables)]
use std::collections::HashMap;

pub fn part1(lines: &mut Vec<String>) -> i32 {
    // [A, X]: Rock         1 pt
    // [B, Y]: Paper        2 pt
    // [C, Z]: Scissors     3 pt
    let win_map: HashMap<&char, char> = HashMap::from([
        (&'A', 'B'), // if chosen Rock     >> Paper    == win
        (&'B', 'C'), // if chosen Paper    >> Scissors == win
        (&'C', 'A'), // if chosen Scissors >> Rock     == win
    ]);
    
    let bonus_map: HashMap<&char, i32> = HashMap::from([
        (&'A', 1),
        (&'B', 2),
        (&'C', 3),
    ]);

    let mut games = lines
    .iter()
    .map(|str| str.chars().filter(|c| c != &' ').collect::<Vec<char>>())
    .collect::<Vec<Vec<char>>>();

    // Functional Nested Stuff!
    // games
    //     .into_iter()
    //     .flat_map(|game| {
    //         game.into_iter()
    //             .map(|c| char_map[&c])
    //     })
    //     .collect();
    
    let mut points = 0;
    for game in &mut games {

        game[1] = match game[1] {
            'X' => 'A',
            'Y' => 'B',
            'Z' => 'C',
            _ => ' ',
        };

        let mut outcome = 0;
        if game[0] == game[1] { outcome = 3; } 
        else if win_map[&game[1]] == game[0] { outcome = 6; }
        // else if win_map.get(&game[1]) == Some(&game[0]) { outcome = 6; }

        points += outcome;
        points += bonus_map[&game[1]];
    }

    return points;
}

pub fn part2(lines: &mut Vec<String>) -> i32 {
    let lose_map: HashMap<&char, char> = HashMap::from([
        (&'A', 'C'), // if chosen Rock     >> Scissors == lose 
        (&'B', 'A'), // if chosen Paper    >> Rock     == lose
        (&'C', 'B'), // if chosen Scissors >> Paper    == lose
    ]);
    
    let win_map: HashMap<&char, char> = HashMap::from([
        (&'A', 'B'), // if chosen Rock     >> Paper    == win
        (&'B', 'C'), // if chosen Paper    >> Scissors == win
        (&'C', 'A'), // if chosen Scissors >> Rock     == win
    ]);
    
    let bonus_map: HashMap<&char, i32> = HashMap::from([
        (&'A', 1),
        (&'B', 2),
        (&'C', 3),
    ]);

    let mut games = lines
        .iter()
        .map(|str| str.chars().filter(|c| c != &' ').collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut points = 0;
    for game in &mut games {
        let outcome: i32 = match game[1] {
            'X' => 0, // need to lose
            'Y' => 1, // need to draw 
            'Z' => 2, // need to win
            _ => 0, // default
        };

        // determine move to play
        points += match outcome {
            0 => bonus_map[&lose_map[&game[0]]],
            1 => bonus_map[&game[0]],
            2 => bonus_map[&win_map[&game[0]]],
            _ => 0,
        };

        points += outcome * 3;
    }

    return points;
}