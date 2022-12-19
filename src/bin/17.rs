#![feature(is_some_and)]
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let rocks: Vec<Vec<(i32, i32)>> = vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        vec![(0, 0), (1, 0), (2, 0), (1, 0), (2, 0)],
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (1, 0), (0, 1), (1, 1)],
    ];

    const X_SIZE: i32 = 7;
    const Y_SIZE: i32 = 10000;

    let stream = input.chars().collect_vec();
    let mut stream_index = 0;

    let mut data: Vec<Vec<bool>> = vec![vec![false; X_SIZE as usize]; Y_SIZE as usize];
    let mut top_rock = 0;

    for i in 0..3 {
        let rock = &rocks[i % rocks.len()];
        let mut x = 2;
        let mut y = top_rock + 3 + rock.iter().max_by(|a, b| Ord::cmp(&a.1, &b.1)).unwrap().1;
        loop {
            let steam_move = match stream[stream_index % stream.len()] == '>' {
                true => 1,
                false => -1,
            };
            let c = stream[stream_index % stream.len()];
            stream_index += 1;

            if can_move(&rock, &data, (x + steam_move, y)) {
                x += steam_move;
            }
            println!("after move {c}");
            let real_rock = rock.iter().map(|pos| (pos.0 + x, pos.1 + y)).collect_vec();
            for y in (0..10).rev() {
                for x in 0..7 {
                    if data[y][x] {
                        print!("#");
                    } else if real_rock.contains(&(x as i32, y as i32)) {
                        print!("O");
                    } else {
                        print!(",");
                    }
                }
                println!();
            }
            println!("after drop");
            if can_move(&rock, &data, (x, y - 1)) {
                y -= 1;
            }

            if !can_move(&rock, &data, (x, y - 1)) {
                let real_rock = rock.iter().map(|pos| (pos.0 + x, pos.1 + y)).collect_vec();
                for (x, y) in real_rock {
                    data[y as usize][x as usize] = true;
                    if y > top_rock {
                        top_rock = y;
                    }
                }

                dbg!(top_rock);
                println!("stuck at");
                let real_rock = rock.iter().map(|pos| (pos.0 + x, pos.1 + y)).collect_vec();
                for y in (0..10).rev() {
                    for x in 0..7 {
                        if data[y][x] {
                            print!("#");
                        } else if real_rock.contains(&(x as i32, y as i32)) {
                            print!("O");
                        } else {
                            print!(",");
                        }
                    }
                    println!();
                }
                println!();
                break;
            }

            println!("rock n {i}");
            let real_rock = rock.iter().map(|pos| (pos.0 + x, pos.1 + y)).collect_vec();
            for y in (0..10).rev() {
                for x in 0..7 {
                    if data[y][x] {
                        print!("#");
                    } else if real_rock.contains(&(x as i32, y as i32)) {
                        print!("O");
                    } else {
                        print!(",");
                    }
                }
                println!();
            }
            println!();
        }
    }

    Some((top_rock) as u32)
}

fn can_move(rock: &Vec<(i32, i32)>, data: &Vec<Vec<bool>>, offset: (i32, i32)) -> bool {
    let rock: Vec<(i32, i32)> = rock
        .iter()
        .map(|(x, y)| (x + offset.0, y + offset.1))
        .collect();

    if rock.iter().any(|&(x, y)| x < 0 || x >= 7 || y < 0) {
        return false;
    }

    for (x, y) in rock {
        if data[y as usize][x as usize] {
            return false;
        }
    }

    true
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 17);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_one(&input), Some(3068));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_two(&input), None);
    }
}
