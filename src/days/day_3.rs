#![allow(dead_code, unused_variables)]
use std::collections::HashSet;

pub fn part1(puzzle_input: &mut Vec<String>) -> i32 {
    
    let mut sum: u32 = 0;        
    let lower_offset: u32 = 96;
    let upper_offset: u32 = 64 - 26;
    
    for (i,str) in puzzle_input.iter().enumerate() {
        let mut used_characters: HashSet<u32> = HashSet::new();
        let backpack: Vec<u32> = str
            .chars()
            .map(|c| {
                if c.is_lowercase() { return c as u32 - lower_offset; }
                return c as u32 - upper_offset;
            })
            .collect();
        
        let (first, second) = backpack.split_at(backpack.len() / 2);

        for c1 in first.iter() {
            for c2 in second.iter() {
                if c1 == c2 && !used_characters.contains(c1) {
                    sum += c1;
                    used_characters.insert(*c1);
                }
            }
        }
    }    
    return sum as i32;
}


pub fn part2(puzzle_input: &mut Vec<String>) -> i32 {
    let mut set: HashSet<i32> = HashSet::new();
    let upper_offset: i32 = 64 - 26;
    let lower_offset: i32 = 96;
    let mut sum: i32 = 0;

    let groups: Vec<Vec<Vec<i32>>>  = puzzle_input
        .chunks(3)
        .map(|chunk| {
            chunk.to_vec()
                .iter()
                .map(|x| {
                    x.chars()
                        .map(|x| {
                            let temp = x as i32;
                            if temp > lower_offset { return temp - lower_offset } // lower_case
                            return temp - upper_offset // convert upper case
                        })
                        .collect::<Vec<i32>>()
                })
                .collect::<Vec<Vec<i32>>>()
        })
        .collect::<Vec<Vec<Vec<i32>>>>(); // it.. just works

    for group in groups.iter() {
        let first:  &Vec<i32> = &group[0];
        let second: &Vec<i32> = &group[1];
        let third:  &Vec<i32> = &group[2];
        
        for c1 in first.iter() {
            for c2 in second.iter() {
                for c3 in third.iter() {
                    if c1 == c2 && c2 == c3 && !set.contains(c1) { set.insert(*c1); }
                }
            }
        }
        sum += set.iter().sum::<i32>();
        set.clear();
    }
    return sum;
}

/* 
pub fn part1_pure(puzzle_input: &mut Vec<String>) -> i32 {
    use std::collections::HashSet;
    return puzzle_input
        .iter()
        .map(|line| -> Result<_> {
            let (first, second) = line.split_at(line.len() / 2);
            let first_items = first
                .bytes()
                .map(|val| match val {
                    b'a'..=b'z' | b'A'..=b'Z' => Ok(val),
                    _ => Err(val),
                })
                .collect::<Result<HashSet<_>, _>>()?;
            process_results(second.bytes().map(|val| match val {
                b'a'..=b'z' | b'A'..=b'Z' => Ok(val),
                _ => Err(val),
            }), |mut it| {
                it.find(|&item| first_items.contains(&item))
                    .map(|item| dbg!(item.score()))
                    .ok_or_else(|| println!("compartments have no items in common"))
            })
        })
        .sum();
}


pub fn part2_pure(puzzle_input: &mut Vec<String>) -> i32 {
    // split puzzle_input into groups of 3 
    let groups: Vec<Vec<String>> = puzzle_input
        .chunks(3)
        .map(|chunk| chunk.to_vec())
        .collect();

    let common_chars: Vec<Vec<u32>> = groups
        .iter()
        .skip(1)
        .map(|group| {
            let first:  Vec<u32> = group[0].chars().map(|c| c as u32).collect();
            let second: Vec<u32> = group[1].chars().map(|c| c as u32).collect();
            let third:  Vec<u32> = group[2].chars().map(|c| c as u32).collect();
            
            println!("{:?}\n{:?}\n{:?}", first, second, third);

            // let dwa: Vec<u32> = first.iter()
            //     .zip(second.iter())
            //     .zip(third.iter())
            //     .map(|((a,b),c)| {
            //         println!("{:?} | {:?} | {:?}", char::from_u32(*a),char::from_u32(*b),char::from_u32(*c));
            //         return *c
            //     })
            //     // .filter(|((a,b),c)| a == b && b == c)
            //     .collect::<Vec<u32>>();
            // println!("{:?}", dwa);

            
            return dwa
            // return first
            //     .iter()
            //     .enumerate()
            //     .filter(|(i,c)| {
            //         // *c == &second[*i] && second[*i] == third[*i]
            //         // they are of different lengths
            //         // fuck
            //     })

        })
        .collect();

    println!("{:?}", common_chars);
    
    let lower_offset: i32 = 96;
    let upper_offset: i32 = 64 - 26;
    return common_chars
        .iter()
        .map(|nums| {
            nums.iter()
                .map(|num| {
                    let temp: i32 = *num as i32;
                    if temp > lower_offset { return temp - lower_offset } // lower_case
                    return temp - upper_offset
                })
                .collect::<Vec<i32>>()
        })
        .flatten()
        .sum::<i32>()

        // return common_chars
    //     .iter()
    //     // .filter(|str| str != &&default[0])
    //     .map(|str| {
    //         str.chars()
    //             .map(|c|{
    //                 if c.is_lowercase() { return c as i32 - lower_offset; }
    //                 return c as i32 - upper_offset;
    //             })
    //             .sum::<i32>()
    //     })
    //     .sum();
}
*/  