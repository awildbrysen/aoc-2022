use std::str;

pub fn calculate_score(input: &str) -> u32 {
    calc_indexes(input)
        .iter()
        .map(|x| [4, 8, 3, 1, 5, 9, 7, 2, 6][*x])
        .sum()
}

pub fn calculate_score_by_guide(input: &str) -> u32 {
    calc_indexes(input)
        .iter()
        .map(|x| [3, 4, 8, 1, 5, 9, 2, 6, 7][*x])
        .sum()
}

fn calc_indexes(play: &str) -> Vec<usize> {
     play.split('\n')
        .into_iter()
        .map(|x| x.split(' ').flat_map(|y| y.chars().nth(0)).collect::<Vec<char>>())
        .filter(|v| v.len() == 2)
        .map(|v| (*v.get(0).unwrap() as u32, *v.get(1).unwrap() as u32))
        .map(|c| ((c.0 - (b'A' as u32)) * 3 + (c.1 - (b'X' as u32))) as usize)
        .collect()
}


#[cfg(test)]
mod tests {
    use crate::day_2_rock_paper_scissors::{calculate_score, calculate_score_by_guide};

    #[test]
    fn test_score() {
        let input = "A Y\nB X\nC Z";
        assert_eq!(15, calculate_score(input));
    }

    #[test]
    fn test_score_by_guide() {
        let input = "A Y\nB X\nC Z";
        assert_eq!(12, calculate_score_by_guide(input));
    }
}
