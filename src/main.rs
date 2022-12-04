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

fn main() {
    day_one()
}
