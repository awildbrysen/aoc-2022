fn find_duplicate_char(parts: (&str, &str)) -> Option<char> {
    parts.0.chars().find(|&c| parts.1.find(c).is_some())
}

fn get_priority(c: char) -> u32 {
    if c.is_lowercase() {
        c as u32 - b'a' as u32 + 1
    } else {
        c as u32 - b'A' as u32 + 27
    }
}

fn find_group_badge(chunks: &[&str]) -> Option<char> {
    chunks[0].chars().find(|&c| chunks[1..].iter().all(|s| s.find(c).is_some()))
}

pub fn priorities(input: &str) -> u32 {
    input.lines()
        .map(|s| s.split_at(s.len() / 2))
        .flat_map(find_duplicate_char)
        .map(get_priority)
        .sum()
}

pub fn group_priorities(input: &str) -> u32 {
    input.lines().collect::<Vec<&str>>()[..]
        .chunks(3)
        .flat_map(find_group_badge)
        .map(get_priority)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day_3_rucksack_reorganization::{priorities, group_priorities};

    #[test]
    fn test_priorities() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(157, priorities(input));
    }

    #[test]
    fn test_group_priorities() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(70, group_priorities(input));
    }
}
