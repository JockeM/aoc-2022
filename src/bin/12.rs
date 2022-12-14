use std::{collections::{HashMap, HashSet, VecDeque}};
use itertools::Itertools;

type Pos = (usize, usize);

fn get_distance_walked(from: Pos, to: Pos, graph: HashMap<Pos, Vec<Pos>>) -> Option<u32> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((from, 0));

    while let Some((pos, distance)) = queue.pop_front() {
        if pos == to {
            return Some(distance);
        }
        if visited.contains(&pos) {
            continue;
        }
        visited.insert(pos);

        if let Some(neighbors) = graph.get(&pos) {
            for &neighbor in neighbors {
                queue.push_back((neighbor, distance + 1));
            }
        }
    }

    None
}

fn can_walk(from: u8, to: u8) -> bool {
    match (from, to) {
        (b'z', b'E') | (b'S', b'a') => true,
        (b'S', _) | (b'E', _) | (_, b'S') | (_, b'E') => false,
        (f, t) => f + 1 >= t,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let data: Vec<Vec<u8>> = input.lines().map(|x| x.bytes().collect()).collect();

    let y_len = data.len();
    let x_len = data[0].len();
    let mut graph: HashMap<Pos, Vec<Pos>> = HashMap::new();
    let mut start = (0, 0);
    let mut end = (0, 0);
    for y in 0..y_len {
        for x in 0..x_len {
            let c = data[y][x];
            match c {
                b'E' => end = (y, x),
                b'S' => start = (y, x),
                _ => (),
            };

            let mut neighbors = vec![];
            if y > 0 && can_walk(c, data[y - 1][x]) {
                neighbors.push((x, y - 1));
            }
            if y + 1 < y_len && can_walk(c, data[y + 1][x]) {
                neighbors.push((x, y + 1));
            }
            if x > 0 && can_walk(c, data[y][x - 1]) {
                neighbors.push((x - 1, y));
            }
            if x + 1 < x_len && can_walk(c, data[y][x + 1]) {
                neighbors.push((x + 1, y));
            }

            graph.insert((x, y), neighbors);
        }
    }

    for (p, n) in graph.clone().iter().sorted() {
        print!("{p:?} [");
        for s in n {
            print!("{s:?} ");
        } 
        println!("]");
    }

    get_distance_walked(start, end, graph)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_walk() {
        assert!(!can_walk(b'a', b'z'));
        assert!(!can_walk(b'a', b'S'));
        assert!(!can_walk(b'S', b'b'));
        assert!(!can_walk(b'E', b'z'));
        assert!(can_walk(b'S', b'a'));
        assert!(can_walk(b'b', b'a'));
        assert!(can_walk(b'z', b'E'));
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), None);
    }
}
