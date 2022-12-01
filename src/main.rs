use std::error::Error;
use std::fs;

mod day_1_calorie_counting;

use day_1_calorie_counting::{get_highest_calories, get_n_highest_calories};

fn main() -> Result<(), Box<dyn Error>> {
    let day_1_input = &fs::read_to_string("input/day_1.txt")?;
    println!("Day 1 part 1: {:?}", get_highest_calories(day_1_input));
    println!("Day 1 part 2: {:?}", get_n_highest_calories(day_1_input, 3).iter().sum::<u32>());

    Ok(())
}
