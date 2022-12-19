use std::collections::HashSet;

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let data: HashSet<(i8,i8,i8)> = 
        input
        .lines()
        .map(|line| line.split(',').map(|n| n.parse().unwrap()).collect_tuple().unwrap())
        .collect();

    const SIDES: [[i8; 3]; 6] = [[-1,0,0], [1,0,0], [0,-1,0], [0,1,0], [0,0,-1], [0,0,1]];
    let mut count = 0;
    for (x,y,z) in &data {
        for side in &SIDES {
            if !data.contains(&(x + side[0], y + side[1], z + side[2])){
                count += 1;
            }
        }
    }
    
    Some(count)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 18);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_one(&input), Some(64));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_two(&input), None);
    }
}
