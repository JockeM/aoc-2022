use std::collections::{HashMap, HashSet, VecDeque};

fn get_distance_walked(from: &Pos, to: &Pos, graph: &HashMap<Pos, Vec<Pos>>) -> Option<u32> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((*from, 0));

    while let Some((pos, distance)) = queue.pop_front() {
        if pos == *to {
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
type Pos = (usize, usize);

pub fn part_one(input: &str) -> Option<u32> {
    let data: HashMap<Pos, u8> = input
        .lines()
        .enumerate()
        .flat_map(|(y, row)| row.bytes().enumerate().map(move |(x, c)| ((x, y), c)))
        .collect();

    let (start, _) = data.iter().find(|(_, c)| **c == b'S').unwrap();
    let (end, _) = data.iter().find(|(_, c)| **c == b'E').unwrap();

    let graph = build_graph(&data);
    get_distance_walked(start, end, &graph)
}

fn build_graph(data: &HashMap<Pos, u8>) -> HashMap<Pos, Vec<Pos>> {
    let mut graph = HashMap::new();
    for (pos, from) in data {
        let mut neighbors = vec![];
        const SIDES: [(i8, i8); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];
        for (dx, dy) in SIDES {
            let pos = (pos.0 + dx as usize, pos.1 + dy as usize);
            if data
                .get(&pos)
                .map(|to| can_walk(*from, *to))
                .unwrap_or(false)
            {
                neighbors.push(pos);
            }
        }

        graph.insert(*pos, neighbors);
    }
    graph
}

pub fn part_two(input: &str) -> Option<u32> {
    let data: HashMap<Pos, u8> = input
        .lines()
        .enumerate()
        .flat_map(|(y, row)| row.bytes().enumerate().map(move |(x, c)| ((x, y), c)))
        .collect();

    let graph = build_graph(&data);
    let (end, _) = data.iter().find(|(_, c)| **c == b'E').unwrap();
    data.iter()
        .filter(|(_, c)| **c == b'a')
        .filter_map(|(from, _)| get_distance_walked(from, end, &graph))
        .min()
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
        assert_eq!(part_two(&input), Some(27));
    }
}
