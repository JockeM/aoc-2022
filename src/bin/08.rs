use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let data: Vec<Vec<i8>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| x.to_digit(10).unwrap() as i8)
                .collect()
        })
        .collect();

    let y_len = data.len();
    let x_len = data[0].len();
    let mut hs: HashSet<(usize, usize)> = HashSet::new();
    for y in 0..y_len {
        let mut left = -1i8;
        for x in 0..x_len {
            let l = data[y][x];
            if l > left {
                hs.insert((x, y));
                left = l;
            }
        }
        let mut right = -1i8;
        for x in (0..x_len).rev() {
            let l = data[y][x];
            if l > right {
                hs.insert((x, y));
                right = l;
            }
        }
    }

    for x in 0..x_len {
        let mut top = -1i8;
        for y in 0..y_len {
            let t = data[y][x];
            if t > top {
                hs.insert((x, y));
                top = t;
            }
        }
        let mut bot = -1i8;
        for y in (0..y_len).rev() {
            let b = data[y][x];
            if b > bot {
                hs.insert((x, y));
                bot = b;
            }
        }
    }

    Some(hs.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), None);
    }
}
