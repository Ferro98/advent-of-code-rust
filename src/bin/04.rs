use std::{collections::HashSet};

pub fn get_section_start_and_end(line: &str) -> (u32, u32, u32, u32) {
    // Sicuramente c'è un modo migliore
    let s: Vec<_> = line.split(",").collect();
    if s.len() > 1 {
        let (one, two): (Vec<_>, Vec<_>) = (s[0].split("-").collect(), s[1].split("-").collect());
        (one[0].parse::<u32>().unwrap(), one[1].parse::<u32>().unwrap(),
        two[0].parse::<u32>().unwrap(), two[1].parse::<u32>().unwrap())
    } else {
        (0, 0, 0, 0)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut count: u32 = 0;
    for line in input.split("\n") {
        let (start1, end1, start2, end2): (u32, u32, u32, u32) = get_section_start_and_end(line);
        if ((start1 <= start2 && end1 >= end2) || (start2 <= start1 && end2 >= end1)) && (start1 != 0) {
            count += 1;
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut count: u32 = 0;
    for line in input.split("\n") {
        let (start1, end1, start2, end2): (u32, u32, u32, u32) = get_section_start_and_end(line);
        println!("{start1}, {end1}, {start2}, {end2}");
        let v: Vec<u32> = Vec::from_iter(start1..end1 + 1);
        let h: HashSet<u32> = HashSet::from_iter(start2..end2 + 1);
        if v.iter().any(|x| h.contains(x)) && start1 != 0 {
            count += 1;
        }
    }
    Some(count)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), None);
    }
}
