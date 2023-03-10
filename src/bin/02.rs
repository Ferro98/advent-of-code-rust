pub fn get_score(input: (&str, &str)) -> u32 { 
    let mut score = 0; 
    match input.1 { 
        "X" => score += 1, 
        "Y" => score += 2, 
        "Z" => score += 3, 
        _ => score += 0 
    } 
    match input { 
        // winning cases 
        ("A", "Y") | ("B", "Z") | ("C", "X") => score += 6, 
        // draw cases 
        ("A", "X") | ("B", "Y") | ("C", "Z") => score += 3, 
        _ => score += 0 
    } 
    return score 
} 

pub fn get_move_to_end_round(input: (&str, &str)) -> String {
    match input {
        ("A", "X") | ("B", "Z") | ("C", "Y") => "Z".to_string(),
        ("A", "Y") | ("B", "X") | ("C", "Z") => "X".to_string(),
        ("A", "Z") | ("B", "Y") | ("C", "X") => "Y".to_string(),
        _ => "".to_string()
    }
}
 
pub fn part_one(input: &str) -> Option<u32> { 
    let mut total_score: u32 = 0; 
    for line in input.split("\n") { 
        let split: Vec<_> = line.split(" ").collect(); 
        if split.len() > 1 { 
            let score = get_score((split[0], split[1])); 
            total_score += score; 
        } 
    } 
    Some(total_score) 
} 
 
pub fn part_two(input: &str) -> Option<u32> { 
    let mut total_score: u32 = 0;
    for line in input.split("\n") { 
        let split: Vec<_> = line.split(" ").collect(); 
        if split.len() > 1 { 
            let m: &str = &get_move_to_end_round((split[0], split[1]));
            let score = get_score((split[0], m)); 
            total_score += score; 
        } 
    }
    Some(total_score)
} 
 
fn main() { 
    let input = &advent_of_code::read_file("inputs", 2); 
    advent_of_code::solve!(1, part_one, input); 
    advent_of_code::solve!(2, part_two, input); 
} 
 
#[cfg(test)] 
mod tests { 
    use super::*; 
 
    #[test] 
    fn test_part_one() { 
        let input = advent_of_code::read_file("examples", 2); 
        assert_eq!(part_one(&input), None); 
    } 
 
    #[test] 
    fn test_part_two() { 
        let input = advent_of_code::read_file("examples", 2); 
        assert_eq!(part_two(&input), None); 
    } 
}