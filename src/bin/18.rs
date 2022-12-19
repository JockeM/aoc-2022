use std::collections::{BinaryHeap, HashMap, HashSet};

use itertools::Itertools;

const SIDES: [(i8, i8, i8); 6] = [
    (-1, 0, 0),
    (1, 0, 0),
    (0, -1, 0),
    (0, 1, 0),
    (0, 0, -1),
    (0, 0, 1),
];

pub fn part_one(input: &str) -> Option<u32> {
    let data: HashSet<(i8, i8, i8)> = input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();

    let mut count = 0;
    for (x, y, z) in &data {
        for (sx, sy, sz) in &SIDES {
            if !data.contains(&(x + sx, y + sy, z + sz)) {
                count += 1;
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let data: HashSet<(i8, i8, i8)> = input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();

    let mut count = 0;
    let mut cache = HashSet::new();
    for (x, y, z) in &data {
        for (sx, sy, sz) in &SIDES {
            let pos = (x + sx, y + sy, z + sz);
            if !data.contains(&pos) && dijkstra(pos, (0, 0, 0), &data, &mut cache) {
                count += 1;
            }
        }
    }

    Some(count)
}

fn dijkstra(
    start: (i8, i8, i8),
    end: (i8, i8, i8),
    blocked: &HashSet<(i8, i8, i8)>,
    cache: &mut HashSet<(i8, i8, i8)>,
) -> bool {
    let mut heap = BinaryHeap::new();
    let mut distances = HashMap::new();
    let mut visited = HashSet::new();

    heap.push((0, start));
    distances.insert(start, 0);

    while let Some((cost, position)) = heap.pop() {
        if position == end || cache.contains(&position) {
            cache.extend(visited);
            return true;
        }
        if visited.contains(&position) {
            continue;
        }
        visited.insert(position);

        for (sx, sy, sz) in SIDES {
            let next = (position.0 + sx, position.1 + sy, position.2 + sz);
            if blocked.contains(&next) || visited.contains(&next) {
                continue;
            }
            let new_cost = cost + 1;
            let distance = distances.entry(next).or_insert(new_cost);
            if new_cost <= *distance {
                *distance = new_cost;
                heap.push((new_cost, next));
            }
        }
    }

    false
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
        assert_eq!(part_two(&input), Some(58));
    }
}
