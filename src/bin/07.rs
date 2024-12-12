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


pub fn part_two(input: &str) -> Option<isize> {
    Some(
        input
            .lines()
            .par_bridge()
            .map(|l| {
                let mut parse = l.split(": ");
                let sum = parse.next().unwrap().parse::<isize>().unwrap();
                let digits = parse
                    .next()
                    .unwrap()
                    .split(" ")
                    .flat_map(str::parse)
                    .collect::<Vec<isize>>();

                let num_ops = digits.len();
                let parsed = 3_isize.pow(num_ops as u32);

                'outer: for i in 0..parsed {
                    let mut total = digits[0];
                    let opstr = format_radix(i as u64, 3);
                    for (idx, z) in digits.iter().skip(1).enumerate() {
                        let mut c = b'0';
                        let strlen = opstr.len() as isize;
                        if ((strlen) - 1) - (idx as isize) > 0 {
                            c = *opstr
                                .as_bytes()
                                .get((strlen as usize - 1) - idx)
                                .unwrap_or(&b'0');
                        }
                        match c as char {
                            '0' => total *= z,
                            '1' => total += z,
                            '2' => total = total * 10_isize.pow(z.ilog10() + 1) + z,
                            _ => panic!("aaaaah!"),
                        }
                    }

                    if total > sum {
                        continue 'outer;
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

fn format_radix(mut x: u64, radix: u32) -> String {
    let mut result = Vec::with_capacity(15);

    loop {
        let m = x % radix as u64;
        x /= radix as u64;

        // will panic if you use a bad radix (< 2 or > 36).
        result.push(std::char::from_digit(m as u32, radix).unwrap());
        if x == 0 {
            break;
        }
    }
    result.into_iter().rev().collect()
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
