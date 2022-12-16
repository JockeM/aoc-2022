use itertools::Itertools;

fn parse(input: &str) -> (Vec<Vec<bool>>, usize) {
    let lines: Vec<Vec<(usize, usize)>> = input
        .lines()
        .map(|row| {
            row.split(" -> ")
                .map(|x| {
                    let (a, b) = x.split_once(',').unwrap();
                    (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap())
                })
                .collect()
        })
        .collect();
    let mut rocks: Vec<Vec<bool>> = vec![vec![false; 1000]; 1000];
    for row in &lines {
        for (a, b) in row.iter().tuple_windows() {
            let (x1, y1) = *a;
            let (x2, y2) = *b;

            let (x1, x2) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
            let (y1, y2) = if y1 < y2 { (y1, y2) } else { (y2, y1) };

            for x in x1..=x2 {
                for y in y1..=y2 {
                    rocks[y][x] = true;
                }
            }
        }
    }
    let max = lines
        .iter()
        .flat_map(|x| x.iter().map(|a| a.1))
        .max()
        .unwrap();

    (rocks, max)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut rocks, max) = parse(input);

    let (mut x, mut y) = (500, 0);
    let mut i = 0;
    loop {
        if !rocks[y + 1][x] {
            y += 1;
            if y > max {
                return Some(i);
            }
        } else if !rocks[y + 1][x - 1] {
            y += 1;
            x -= 1;
        } else if !rocks[y + 1][x + 1] {
            y += 1;
            x += 1;
        } else {
            rocks[y][x] = true;
            x = 500;
            y = 0;
            i += 1;
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut rocks, max) = parse(input);

    for x in 0..1000 {
        rocks[max + 2][x] = true;
    }

    let (mut x, mut y) = (500, 0);
    let mut i = 0;
    loop {
        if !rocks[y + 1][x] {
            y += 1;
        } else if !rocks[y + 1][x - 1] {
            y += 1;
            x -= 1;
        } else if !rocks[y + 1][x + 1] {
            y += 1;
            x += 1;
        } else {
            rocks[y][x] = true;
            x = 500;
            y = 0;
            i += 1;
            if rocks[0][500] {
                return Some(i);
            }
        }
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
