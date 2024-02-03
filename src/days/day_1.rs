pub fn part1(lines: &mut Vec<String>) -> i32 {
    // split by space (empty line) >> new vectors
    // for each vector: sum the calories
    // if cur_vector's cur_calories > most_calories, set most_calories = cur_calories, largest_idx = cur_idx
    
    let calorie_sets: Vec<Vec<String>> = lines
        .split(|line| line == "")
        .map(|line| line.to_vec())
        .collect();
    
    let mut most_calories = 0;
    for set in &calorie_sets {
        let calories:i32 = set
            .iter()
            .map(|munch| munch.parse::<i32>().unwrap())
            .sum();
        if calories > most_calories {
            most_calories = calories;
        }
    }

    return most_calories;
}

pub fn part2(lines: &mut Vec<String>) -> i32 {

    let calorie_sets: Vec<Vec<String>> = lines
    .split(|line| line == "")
    .map(|line| line.to_vec())
    .collect();

    // let mut most_calories = 0;
    let mut top_3: Vec<i32> = Vec::new();
    top_3.resize(3,-1);
       
    for set in &calorie_sets {
        let calories: i32 = set
            .iter()
            .map(|item| item.parse::<i32>().unwrap())
            .sum();

        top_3.sort();
        for cur_calories in &mut top_3 {
            if calories > *cur_calories {
                *cur_calories = calories;
                break;
            }
        }   
    }


    return top_3.iter().sum();
}