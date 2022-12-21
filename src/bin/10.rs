pub fn part_one(input: &str) -> Option<u32> {
    let mut iter = input
        .lines()
        .map(|line| {
            line.split_once(' ')
                .map(|(_, num)| num.parse::<i32>().unwrap())
        })
        .cycle();

    let mut sum = 0;
    let mut x = 1;
    let mut cycle = 0;
    while cycle < 220 {
        if let Some(value) = iter.next().unwrap() {
            cycle += 1;
            check_sum(cycle, x, &mut sum);
            cycle += 1;
            check_sum(cycle, x, &mut sum);
            x += value;
        } else {
            cycle += 1;
            check_sum(cycle, x, &mut sum);
        }

        fn check_sum(cycle: i32, x: i32, sum: &mut i32) {
            if (cycle + 20) % 40 == 0 {
                *sum += x * cycle as i32;
            }
        }
    }

    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut iter = input
        .lines()
        .map(|line| {
            line.split_once(' ')
                .map(|(_, num)| num.parse::<i32>().unwrap())
        })
        .cycle();

    let mut data = vec![];
    let mut x = 1;
    let mut cycle = -1;
    while cycle < 240 {
        if let Some(value) = iter.next().unwrap() {
            cycle += 1;
            data.push(x >= cycle % 40 - 1 && x <= cycle % 40 + 1);
            cycle += 1;
            data.push(x >= cycle % 40 - 1 && x <= cycle % 40 + 1);
            x += value;
        } else {
            cycle += 1;
            data.push(x >= cycle % 40 - 1 && x <= cycle % 40 + 1);
        }
    }

    for y in 0..40 * 6 {
        if y % 40 == 0 {
            println!();
        }
        if data[y] {
            print!("#");
        } else {
            print!(" ");
        }
    }
    println!();

    Some(1)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), Some(1));
    }
}
