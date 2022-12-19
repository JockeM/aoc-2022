use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

type Cord = [i8; 3];

const SIDES: [Cord; 6] = [
    [-1, 0, 0],
    [1, 0, 0],
    [0, -1, 0],
    [0, 1, 0],
    [0, 0, -1],
    [0, 0, 1],
];

fn add(a: &Cord, b: &Cord) -> Cord {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

pub fn part_one(input: &str) -> Option<u32> {
    let data: HashSet<Cord> = input
        .lines()
        .map(|line| {
            let mut it = line.split(',');
            [
                it.next().unwrap().parse().unwrap(),
                it.next().unwrap().parse().unwrap(),
                it.next().unwrap().parse().unwrap(),
            ]
        })
        .collect();

    let mut count = 0;
    for pos in &data {
        for dir in &SIDES {
            if !data.contains(&add(pos, dir)) {
                count += 1;
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let data: HashSet<Cord> = input
        .lines()
        .map(|line| {
            let mut it = line.split(',');
            [
                it.next().unwrap().parse().unwrap(),
                it.next().unwrap().parse().unwrap(),
                it.next().unwrap().parse().unwrap(),
            ]
        })
        .collect();

    let mut count = 0;
    let mut cache = HashSet::new();
    for pos in &data {
        for dir in &SIDES {
            let pos = add(pos, dir);
            if !data.contains(&pos) && is_outside(pos, &data, &mut cache) {
                count += 1;
            }
        }
    }

    Some(count)
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    position: Cord,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn is_outside(cord: Cord, blocked: &HashSet<Cord>, cache: &mut HashSet<Cord>) -> bool {
    let mut heap = BinaryHeap::new();
    let mut distances = HashMap::new();
    let mut visited = HashSet::new();

    heap.push(State {
        cost: 0,
        position: cord,
    });
    distances.insert(cord, 0);

    while let Some(State { cost: _, position }) = heap.pop() {
        if position == [0, 0, 0] || cache.contains(&position) {
            cache.extend(visited);
            return true;
        }
        if visited.contains(&position) {
            continue;
        }
        visited.insert(position);

        for dir in &SIDES {
            let next = add(&position, dir);
            if blocked.contains(&next) || visited.contains(&next) {
                continue;
            }
            let new_cost = position[0] as u32 ^ 2 + position[1] as u32 ^ 2 + position[2] as u32 ^ 2;
            let distance = distances.entry(next).or_insert(new_cost);
            if new_cost <= *distance {
                *distance = new_cost;
                heap.push(State {
                    cost: new_cost,
                    position: next,
                });
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
