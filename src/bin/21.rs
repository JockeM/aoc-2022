use std::collections::HashMap;
#[derive(Debug)]
enum Entry<'a> {
    Value(i128),
    Lazy(&'a str, &'a str, &'a str),
}

pub fn part_one(input: &str) -> Option<i128> {
    let data: HashMap<&str, Entry> = input
        .lines()
        .map(|line| {
            let (k, r) = line.split_once(": ").unwrap();
            if let Ok(v) = r.parse() {
                return (k, Entry::Value(v));
            }
            let mut s = r.split(' ');
            (
                k,
                Entry::Lazy(s.next().unwrap(), s.next().unwrap(), s.next().unwrap()),
            )
        })
        .collect();
    let root = &data.get("root").unwrap();

    Some(resolve(root, &data))
}

fn resolve(entry: &Entry, data: &HashMap<&str, Entry>) -> i128 {
    match entry {
        Entry::Value(v) => *v,
        Entry::Lazy(a, op, b) => {
            let a = resolve(data.get(a).unwrap(), data);
            let b = resolve(data.get(b).unwrap(), data);

            match *op {
                "+" => a + b,
                "-" => a - b,
                "*" => a * b,
                "/" => a / b,
                _ => unreachable!(),
            }
        }
    }
}

pub fn part_two(input: &str) -> Option<i128> {
    let mut data: HashMap<&str, Entry> = input
        .lines()
        .map(|line| {
            let (k, r) = line.split_once(": ").unwrap();
            if let Ok(v) = r.parse() {
                return (k, Entry::Value(v));
            }
            let mut s = r.split(' ');
            (
                k,
                Entry::Lazy(s.next().unwrap(), s.next().unwrap(), s.next().unwrap()),
            )
        })
        .collect();

    if let Entry::Lazy(a, _, b) = &data.get("root").unwrap() {
        let a = a.clone();
        let b = resolve(&data.get(b).unwrap(), &data);

        let mut y_values = vec![];
        let mut x_values = vec![];
        for i in 0..100 {
            let x = i * 1_000_000;
            let y = fun_name(&mut data, x, a);
            y_values.push(x);
            x_values.push(y);
        }

        let x = interpolate_x(b, &x_values, &y_values);
        println!("x {x}");
        //let mut low = 0;
        //let mut high = i64::MAX as i128;
        //
        //while low <= high {
        //    println!("low {low}, high {high}");
        //    let mid = low + (high - low) / 2;
        //    let a = fun_name(&mut data, mid, a);
        //    println!("a {a}, b {b}");
        //    if a == b {
        //        return Some(mid);
        //    } else if a > b {
        //        low = mid + 1;
        //    } else {
        //        high = mid - 1;
        //    }
        //}
    }

    None
}

fn fun_name(data: &mut HashMap<&str, Entry>, mid: i128, a: &str) -> i128 {
    data.insert("humn", Entry::Value(mid));
    resolve(data.get(a).unwrap(), data)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 21);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_one(&input), Some(152));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_two(&input), Some(301));
    }
}
