use std::vec;

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let start_x = grid
        .first()
        .unwrap()
        .iter()
        .position(|c| *c == '.')
        .unwrap();
    let end_x = grid.last().unwrap().iter().position(|c| *c == '.').unwrap();
    let start = (start_x, 0);
    let end = (end_x, grid.len());

    let mut blizzards: Vec<(usize, usize, char)> = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|&(_, c)| matches!(c, '^' | 'v' | '<' | '>'))
                .map(move |(x, c)| (x, y, *c))
        })
        .collect();

    dbg!(&blizzards);

    let mut maps: Vec<Vec<bool>> = vec![];
    let (top, width, left, height) = (0, grid.len(), 0, grid[0].len());
    for _ in 0..30 {
        let mut buffer: Vec<bool> = vec![false; width * height];
        for (x, y, c) in blizzards.iter_mut() {
            let xi = *x as i32;
            let yi = *y as i32;
            let (mut new_x, mut new_y) = match *c {
                '^' => (xi, yi - 1),
                'v' => (xi, yi + 1),
                '<' => (xi - 1, yi),
                '>' => (xi + 1, yi),
                _ => unreachable!(),
            };
            if new_y >= height as i32 {
                new_y = 1;
            } else if new_y <= 0 {
                new_y = height as i32;
            }

            if new_x >= width as i32 {
                new_x = 1;
            } else if new_x <= 0 {
                new_x = width as i32;
            }

            *x = new_x as usize;
            *y = new_y as usize;
            println!("({new_x}, {new_y})");
            buffer[*x + *y * height] = true;
        }

        maps.push(buffer);
    }

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 24);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 24);
        assert_eq!(part_one(&input), Some(18));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 24);
        assert_eq!(part_two(&input), None);
    }
}
