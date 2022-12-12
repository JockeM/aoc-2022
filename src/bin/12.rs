use std::collections::HashMap;

type Pos = (usize, usize);

fn can_walk(from: u8, to: u8) -> bool {
    match (from, to) {
        (b'E', b'z') | (b'S', b'a') => true,
        (f, t) => f < t,
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
            if x + 1 < y_len && can_walk(c, data[y][x + 1]) {
                neighbors.push((x + 1, y));
            }
            graph.insert((x, y), neighbors);
        }
    }

    println!("start: {start:?} end:{end:?}");
    dbg!(graph);
    Some(1)
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
