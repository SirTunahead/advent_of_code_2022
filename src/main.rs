fn day_one() {
    const INPUT: &str = include_str!("day_1_input.txt");

    println!("---------Day One-----------");

    let mut input = vec![];
    let mut result: Vec<i32> = vec![];
    let mut sum = 0;

    for line in INPUT.lines() {
        if line.is_empty() {
            input.push(0);
        } else {
            input.push(line.parse::<i32>().unwrap());
        }
    }
    input.push(0);
    for x in &input {
        if x > &0 {
            sum += x;
        }
        if x == &0 {
            result.push(sum);
            sum = 0;
        }
    }

    let mut most_calories = 0;

    for x in result {
        if x > most_calories {
            most_calories = x;
        }
    }

    println!("The Elf with the most calories: {most_calories}");
}

fn day_two() {
    println!("---------Day Two-----------");
    const INPUT: &str = include_str!("day_2_input.txt");

    let mut guide_entry: Vec<String> = Vec::new();
    let mut score = 0;

    for line in INPUT.lines() {
        guide_entry.push(line.parse::<String>().unwrap());
    }

    //Rock (1) = A, X ; Paper(2) = B, Y ; Scissors (3)  = C, Z
    // Lost = 0, Draw = 3, Win = 6

    for x in guide_entry {
        let enemy = x.chars().next().unwrap();
        let me = x.chars().nth(2).unwrap();

        //Draw
        if enemy == 'A' && me == 'X' || enemy == 'B' && me == 'Y' || enemy == 'C' && me == 'Z' {
            score += 3;
        }
        //Lose
        if enemy == 'A' && me == 'Z' || enemy == 'B' && me == 'X' || enemy == 'C' && me == 'Y' {
            score += 0;
        }
        //Win
        if enemy == 'A' && me == 'Y' || enemy == 'B' && me == 'Z' || enemy == 'C' && me == 'X' {
            score += 6;
        }

        if me == 'X' {
            score += 1;
        } else if me == 'Y' {
            score += 2;
        } else {
            score += 3;
        }
    }
    println!("SCORE: {score}");
}

#[derive(Debug)]
struct Rucksack {
    number: u16,
    full_rucksack: String,
    first_compartment: String,
    second_compartment: String,
}

fn day_three() {
    println!("---------Day Three-----------");
    const INPUT: &str = include_str!("day_3_input.txt");
    let mut counter = 0;

    let mut rucksacks: Vec<Rucksack> = Vec::new();

    for line in INPUT.lines() {
        counter += 1;
        let (split1, split2) = line.split_at(line.len() / 2);

        let rucksack = Rucksack {
            number: counter,
            full_rucksack: line.parse::<String>().unwrap().to_string(),
            first_compartment: split1.to_string(),
            second_compartment: split2.to_string(),
        };
        rucksacks.push(rucksack);
    }

    let mut priority = 0;
    let mut match_found = false;

    for rucksack in rucksacks {
        for item in rucksack.first_compartment.chars() {
            if match_found == false {
                for item2 in rucksack.second_compartment.chars() {
                    if item == item2 {
                        match_found = true;
                        let item_ascii = item as u32;
                        if item_ascii >= 65 && item_ascii <= 90 {
                            //calc for priority for big letters
                            priority += item_ascii - 38;
                            break;
                        } else if item_ascii >= 97 {
                            //calc for priority for small letters
                            priority += item_ascii - 96;
                            break;
                        }
                    }
                }
            }
        }
        match_found = false;
    }
    println!("sum of the priorities: {priority}");
}

fn main() {
    day_one();
    day_two();
    day_three();
}
