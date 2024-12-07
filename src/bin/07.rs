use std::time::SystemTime;

use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<isize> {
    Some(
        input
            .lines()
            .map(|l| {
                let mut parse = l.split(": ");
                let sum = parse.next().unwrap().parse::<isize>().unwrap();
                let digits = parse
                    .next()
                    .unwrap()
                    .split(" ")
                    .flat_map(str::parse)
                    .collect::<Vec<isize>>();

                let mut b = String::new();
                let num_ops = digits.len();
                for _ in 0..num_ops {
                    b += "1";
                }

                let parsed = isize::from_str_radix(&b, 2).unwrap();

                for i in 0..parsed {
                    let mut total = 0;
                    for (idx, z) in digits.iter().enumerate() {
                        match get_b_digit(i, idx.try_into().unwrap()) {
                            true => {
                                total *= z;
                            }
                            false => {
                                total += z;
                            }
                        }
                    }

                    if total == sum {
                        return sum;
                    }
                }

                0
            })
            .sum(),
    )
}

fn get_b_digit(n: isize, d: isize) -> bool {
    (n >> d) & 1 != 0
}

#[derive(Debug, Hash, PartialEq, PartialOrd, Ord, Eq, Clone, Copy)]
enum Operation {
    Add,
    Mul,
    Or,
}

pub fn part_two(input: &str) -> Option<isize> {
    let t = SystemTime::now();
    let line_count = input.lines().count();
    Some(
        input
            .lines()
            .enumerate()
            .par_bridge()
            .map(|(idx, l)| {
                println!("[{idx}/{line_count}] line: {l} {:?}", t.elapsed());
                let mut parse = l.split(": ");
                let sum = parse.next().unwrap().parse::<isize>().unwrap();
                let digits = parse
                    .next()
                    .unwrap()
                    .split(" ")
                    .flat_map(str::parse)
                    .collect::<Vec<isize>>();

                let num_ops = digits.len() - 1;

                let perms = [Operation::Mul, Operation::Add, Operation::Or];
                let perm_list = perms
                    .iter()
                    .cartesian_product(0..num_ops)
                    .combinations(num_ops);

                // println!("perm_list: {perm_list:?}");
                'outer: for perm in perm_list {
                    let mut idx = 0;
                    let mut total = 0;
                    let mut current_op = Operation::Add;

                    while idx < digits.len() {
                        match current_op {
                            Operation::Add => total += digits[idx],
                            Operation::Mul => total *= digits[idx],
                            Operation::Or => {
                                total = format!("{total}{}", digits[idx]).parse().unwrap()
                            }
                        }
                        if idx < num_ops {
                            let x = perm.iter().find(|(_, id)| *id == idx);
                            if x.is_none() {
                                continue 'outer;
                            }

                            current_op = *x.unwrap().0;
                        }
                        idx += 1;
                    }

                    if sum == total {
                        return total;
                    }
                }

                0
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
