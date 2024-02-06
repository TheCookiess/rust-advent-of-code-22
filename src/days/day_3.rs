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
    

    return 5;
}