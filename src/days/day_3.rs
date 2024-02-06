#![allow(dead_code, unused_variables)]

pub fn part1(lines: &mut Vec<String>) -> i32 {
    use std::collections::HashSet;
    
    let mut sum: u32 = 0;        
    let lower_offset: u32 = 96;
    let upper_offset: u32 = 64 - 26;
    
    for (i,str) in lines.iter().enumerate() {
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

// pub fn part1_pure(lines: &mut Vec<String>) -> i32 {
//     use std::collections::HashSet;
//     return lines
//         .iter()
//         .map(|line| -> Result<_> {
//             let (first, second) = line.split_at(line.len() / 2);
//             let first_items = first
//                 .bytes()
//                 .map(|val| match val {
//                     b'a'..=b'z' | b'A'..=b'Z' => Ok(val),
//                     _ => Err(val),
//                 })
//                 .collect::<Result<HashSet<_>, _>>()?;
//             process_results(second.bytes().map(|val| match val {
//                 b'a'..=b'z' | b'A'..=b'Z' => Ok(val),
//                 _ => Err(val),
//             }), |mut it| {
//                 it.find(|&item| first_items.contains(&item))
//                     .map(|item| dbg!(item.score()))
//                     .ok_or_else(|| println!("compartments have no items in common"))
//             })
//         })
//         .sum();
// }

pub fn part2(lines: &mut Vec<String>) -> i32 {
    // split lines into groups of 3 
    let groups: Vec<Vec<String>> = lines
        .chunks(3)
        .map(|chunk| chunk.to_vec())
        .collect();

    // let default: Vec<String> = Vec::from(["default".to_string()]);
    // let common_chars = groups
    //     .iter()
    //     .skip(1)
    //     .find(|group| {
    //         let first = group[0].chars();
    //         let second = group[1].chars();
    //         let third = group[2].chars();
    //         first.zip(second).zip(third).all(|((a, b), c)| a == b && b == c)
    //     })
    //     .unwrap_or_else(|| &&default);


    let common_chars: Vec<Vec<u32>> = groups
        .iter()
        .skip(1)
        .map(|group| {
            let first:  Vec<u32> = group[0].chars().map(|c| c as u32).collect();
            let second: Vec<u32> = group[1].chars().map(|c| c as u32).collect();
            let third:  Vec<u32> = group[2].chars().map(|c| c as u32).collect();
            
            return first.iter()
                .zip(second.iter())
                .zip(third.iter())
                .filter(|((a,b),c)| a == b && b == c)
                


            // return first
            //     .iter()
            //     .enumerate()
            //     .filter(|(i,c)| {
            //         // *c == &second[*i] && second[*i] == third[*i]
            //         // they are of different lengths
            //         // fuck
            //     })
                .map(|(i, c)| *c)
                .collect::<Vec<u32>>()
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