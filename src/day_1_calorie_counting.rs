pub fn get_highest_calories(input: &str) -> Option<u32> {
    collect_calories(input)
        .into_iter()
        .max()
}

pub fn get_n_highest_calories(input: &str, n: usize) -> Vec<u32> {
    collect_calories_sorted(input)[0..n].to_owned()
}

pub fn collect_calories_sorted(input: &str) -> Vec<u32> {
    let mut c = collect_calories(input);
    c.sort();
    c.reverse();
    c
}

fn collect_calories(input: &str) -> Vec<u32> {
    input
        .split("\n\n")
        .into_iter()
        .map(|x| x.split('\n'))
        .map(|x| x.into_iter().map(|i| i.parse::<u32>()).filter_map(|i| i.ok()).sum::<u32>())
        .collect::<Vec<u32>>()
}

#[cfg(test)]
mod tests {
    use crate::day_1_calorie_counting::{collect_calories, collect_calories_sorted, get_highest_calories, get_n_highest_calories};

    static INPUT: &str = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";

    #[test]
    fn test_get_highest_calories() {
        let highest_calories = get_highest_calories(INPUT);
        assert_eq!(highest_calories, Some(24000));
    }

    #[test]
    fn test_collect_calories() {
        let calories = collect_calories(INPUT);
        assert_eq!(calories, vec!(6000, 4000, 11000, 24000, 10000));
    }

    #[test]
    fn test_get_n_highest_calories() {
        let calories = get_n_highest_calories(INPUT, 3);
        assert_eq!(calories, vec!(24000, 11000, 10000));
    }

    #[test]
    fn test_collect_calories_sorted() {
        let calories = collect_calories_sorted(INPUT);
        assert_eq!(calories, vec!(24000, 11000, 10000, 6000, 4000));
    }
}
