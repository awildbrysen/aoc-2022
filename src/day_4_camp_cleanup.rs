use std::collections::HashSet;

fn get_sets(line: &str) -> Vec<HashSet<i32>> {
    line.split(',')
        .flat_map(|s| s.split('-'))
        .flat_map(|s| s.parse::<i32>().ok())
        .collect::<Vec<i32>>()[..]
        .chunks(2)
        .map(|c: &[i32]| -> HashSet<i32> {
            HashSet::from_iter(c[0]..(c[1]+1))
        })
        .collect()
}

fn contains_a_subset(sets: &Vec<HashSet<i32>>) -> bool {
    let s1 = sets.get(0).unwrap();
    let s2 = sets.get(1).unwrap();
    s1.is_subset(s2) || s2.is_subset(s1)
}

fn overlap(sets: &Vec<HashSet<i32>>) -> bool {
    let s1 = sets.get(0).unwrap();
    let s2 = sets.get(1).unwrap();
    !s1.is_disjoint(s2)
}

pub fn subset_assignments(input: &str) -> u32 {
    input.lines()
        .map(get_sets)
        .filter(contains_a_subset)
        .count() as u32
}

pub fn overlapping_assignments(input: &str) -> u32 {
    input.lines()
        .map(get_sets)
        .filter(overlap)
        .count() as u32
}

#[cfg(test)]
mod tests {
    use crate::day_4_camp_cleanup::{subset_assignments, overlapping_assignments};

    #[test]
    fn test_subset() {
        let input = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";
        assert_eq!(2, subset_assignments(input));
    }

    #[test]
    fn test_overlap() {
        let input = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";
        assert_eq!(4, overlapping_assignments(input));
    }
}