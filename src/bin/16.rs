use std::{
    collections::{HashMap, HashSet},
    vec,
};

use itertools::Itertools;

struct Valve {
    pub rate: u32,
    pub connected: Vec<(char, char)>,
}

pub fn part_one(input: &str) -> Option<u32> {
    let valves: HashMap<(char, char), Valve> = input
        .lines()
        .map(|line| {
            let split = line.split_ascii_whitespace().collect_vec();

            let num = split[4].split_once('=').unwrap().1.trim_end_matches(';');
            (
                split[1].chars().collect_tuple().unwrap(),
                Valve {
                    rate: num.parse().unwrap(),
                    connected: split[9..]
                        .iter()
                        .map(|x| {
                            let v = x.chars().collect_vec();
                            (v[0], v[1])
                        })
                        .collect(),
                },
            )
        })
        .collect();

    let mut open: HashSet<(char, char)> = HashSet::new();
    open.insert(('A', 'A'));
    let mut best = 0;
    go_to_valve(30, ('A', 'A'), &valves, open, 0, &mut best);

    Some(best)
}

fn go_to_valve(
    time: u32,
    current: (char, char),
    valves: &HashMap<(char, char), Valve>,
    open: HashSet<(char, char)>,
    pressure_released: u32,
    best: &mut u32,
) {
    let valve = valves.get(&current).unwrap();
    if time == 0 {
        if pressure_released > *best {
            *best = pressure_released;
        }
        return;
    } else if time == 1 {
        let new_pressure = pressure_released + valve.rate * time;
        if new_pressure > *best {
            *best = pressure_released;
        }
        return;
    }

    for v in &valve.connected {
        go_to_valve(time - 1, *v, valves, open.clone(), pressure_released, best);
    }

    if valve.rate != 0 && !open.contains(&current) {
        let new_pressure = pressure_released + valve.rate * time;
        let mut open_clone = open.clone();
        if time <= 1 {
            return;
        }
        open_clone.insert(current);
        for v in &valve.connected {
            go_to_valve(time - 2, *v, valves, open_clone.clone(), new_pressure, best);
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_two(&input), None);
    }
}
