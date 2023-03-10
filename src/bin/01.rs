use std::collections::HashMap; 
 
pub fn get_elves(input: &str) -> HashMap<u32, u32> { 
    let mut elves: HashMap<u32, u32> = HashMap::new(); 
    let mut i: u32 = 0; 
    for line in input.split("\n") { 
        if line.trim().is_empty() { 
            i += 1; 
            continue; 
        } 
        let calorie: u32 = line.parse().unwrap(); 
        elves.entry(i).and_modify(|counter| *counter += calorie).or_insert(calorie); 
    } 
    elves 
} 
 
// Volevo fare con un vettore, avendo ogni indice come elfo. Si può fare? 
// Forse è meglio iterare e salvarsi solo la somma e l'indice di ogni elfo, senza salvare tutto in memoria. 
pub fn part_one(input: &str) -> Option<u32> { 
    let elves = get_elves(input); 
    // Ci deve essere un modo migliore per restituire il valore... 
    Some(*elves.iter().max_by_key(|x| x.1).unwrap().1) 
} 
 
pub fn part_two(input: &str) -> Option<u32> { 
    let elves = get_elves(input); 
    let mut sorted: Vec<_> = elves.into_iter().collect(); 
    sorted.sort_by(|&a, &b| b.1.cmp(&a.1)); 
    let mut total: u32 = 0; 
    for (_i, x) in &sorted[..3] { 
        total += x; 
    } 
    Some(total) 
} 
 
fn main() { 
    let input = &advent_of_code::read_file("inputs", 1); 
    advent_of_code::solve!(1, part_one, input); 
    advent_of_code::solve!(2, part_two, input); 
} 
 
#[cfg(test)] 
mod tests { 
    use super::*; 
 
    #[test] 
    fn test_part_one() { 
        let input = advent_of_code::read_file("examples", 1); 
        assert_eq!(part_one(&input), None); 
    } 
 
    #[test] 
    fn test_part_two() { 
        let input = advent_of_code::read_file("examples", 1); 
        assert_eq!(part_two(&input), None); 
    } 
}