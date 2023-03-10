use std::collections::HashSet;

pub fn get_priority(c: char) -> u32 {
    if c.is_uppercase() {
        c as u32 - 64 + 26
    } else {
        c as u32 - 96
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut priorities: u32 = 0;
    for line in input.split("\n") {
        let len = line.len();
            if len > 0 {
            let (half1, half2) = line.split_at(len / 2);
            let set: HashSet<char> = half1.chars().collect();
            let shared_char = half2.chars().find(|c| set.contains(&c)).unwrap();
            priorities += get_priority(shared_char);
        }
    }
    Some(priorities)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut priorities: u32 = 0;
    let r: Vec<_> = input.split("\n").collect();
    for chunk in r.chunks(3) {
        if chunk.len() > 2 {
            let set1: HashSet<_> = chunk[0].chars().collect();
            let set2: HashSet<_> = chunk[1].chars().collect();
            let shared_chars: HashSet<_> = set1.intersection(&set2).collect();
            let shared_char = chunk[2].chars().find(|c| shared_chars.contains(&c)).unwrap();
            priorities += get_priority(shared_char);
        }
    }
    Some(priorities)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), None);
    }
}
