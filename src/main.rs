use std::error::Error;
use std::fs;

mod day_1_calorie_counting;
mod day_2_rock_paper_scissors;

use day_1_calorie_counting::{get_highest_calories, get_n_highest_calories};
use day_2_rock_paper_scissors::{calculate_score, calculate_score_by_guide};

fn main() -> Result<(), Box<dyn Error>> {
    let day_1_input = &fs::read_to_string("input/day_1.txt")?;
    println!("Day 1 part 1: {:?}", get_highest_calories(day_1_input));
    println!("Day 1 part 2: {:?}", get_n_highest_calories(day_1_input, 3).iter().sum::<u32>());

    let day_2_input = &fs::read_to_string("input/day_2.txt")?;
    println!("Day 2 part 1: {:?}", calculate_score(day_2_input));
    println!("Day 2 part 2: {:?}", calculate_score_by_guide(day_2_input));

    Ok(())
}
