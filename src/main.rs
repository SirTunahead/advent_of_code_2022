fn day_one(){

const INPUT: &str = include_str!("day_1_input.txt");

println!("---------Day One-----------");

let mut input = vec![];
let mut result: Vec<i32> = vec![];
let mut sum = 0;

for line in INPUT.lines(){
    if line.is_empty(){
        input.push(0);
    }
    else{
        input.push(line.parse::<i32>().unwrap());
    }   
}
input.push(0);
for x in &input{
    if x > &0 {
        sum += x;
    }if x == &0 {
        result.push(sum);
        sum = 0;
    }
}

let mut most_calories = 0;

for x in result{
    if x > most_calories{
        most_calories = x;
    }
}

println!("The Elf with the most calories: {most_calories}");

}

fn day_two(){
    println!("---------Day Two-----------");
    const INPUT: &str = include_str!("day_2_input.txt");

    let mut guide_entry: Vec<String> = Vec::new();
    let mut score = 0;

    for line in INPUT.lines() {
        guide_entry.push(line.parse::<String>().unwrap());
    }

    //Rock (1) = A, X ; Paper(2) = B, Y ; Scissors (3)  = C, Z
    // Lost = 0, Draw = 3, Win = 6 

    for x in guide_entry{
        let enemy = x.chars().next().unwrap();
        let me = x.chars().nth(2).unwrap();

        //Draw 
        if enemy == 'A' && me == 'X' || enemy == 'B' && me == 'Y' || enemy == 'C' && me == 'Z'{
            score += 3;
        }
        //Lose 
        if enemy == 'A' && me == 'Z' || enemy == 'B' && me == 'X' || enemy == 'C' && me == 'Y'{
            score += 0;
        }
        //Win
        if enemy == 'A' && me == 'Y' || enemy == 'B' && me == 'Z' || enemy == 'C' && me == 'X'{
            score += 6;
        }

        if me == 'X'  {
            score += 1;
        } else if me == 'Y'  {
            score += 2;
        } else {
            score += 3;
        }
    }
    println!("SCORE: {score}");
}

fn main() {
    day_one();
    day_two();
}
