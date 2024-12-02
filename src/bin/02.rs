advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|l| {
                let parsed = l
                    .split(" ")
                    .map(|c| c.parse::<i32>().unwrap())
                    .enumerate()
                    .collect::<Vec<_>>();

                let mut increasing = None;
                let mut safe = true;

                for (idx, p) in parsed.clone() {
                    if idx == 0 {
                        if p > parsed[idx + 1].1 {
                            increasing = Some(false)
                        } else {
                            increasing = Some(true)
                        }
                    }

                    if idx + 1 != parsed.len() {
                        let diff = (p - parsed[idx + 1].1).abs();

                        match (increasing, p, parsed[idx + 1].1) {
                            (Some(true), c, n) => {
                                if c > n {
                                    safe = false;
                                    break;
                                }
                            }
                            (Some(false), c, n) => {
                                if c < n {
                                    safe = false;
                                    break;
                                }
                            }
                            _ => panic!("aaaa"),
                        };

                        if diff == 0 || diff > 3 {
                            safe = false;
                            break;
                        }
                    }
                }

                match safe {
                    true => 1,
                    false => 0,
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|l| {
                let parsed = l
                    .split(" ")
                    .map(|c| c.parse::<i32>().unwrap())
                    .enumerate()
                    .collect::<Vec<_>>();

                let bads = solve(parsed.clone());

                if bads.iter().filter(|b| **b).collect::<Vec<_>>().len() == parsed.len() {
                    return 1;
                }

                for (idx, _) in bads.iter().enumerate() {
                    let mut parsed = parsed.clone().iter().map(|(_, v)| *v).collect::<Vec<_>>();

                    parsed.remove(idx);

                    let bads = solve(parsed.clone().into_iter().enumerate().collect::<Vec<_>>());

                    if bads.iter().filter(|b| **b).collect::<Vec<_>>().len() == parsed.clone().len()
                    {
                        return 1;
                    }
                }

                0
            })
            .sum(),
    )
}

fn solve(parsed: Vec<(usize, i32)>) -> Vec<bool> {
    let mut increasing = None;
    let mut ret = vec![];

    for (idx, p) in parsed.clone() {
        let mut safe = true;
        let mut idx_2 = 0;
        while increasing.is_none() {
            if p > parsed[idx_2 + 1].1 {
                increasing = Some(false)
            } else if p < parsed[idx_2 + 1].1 {
                increasing = Some(true)
            } else {
                increasing = None
            }
            idx_2 += 1;
        }

        if idx >= 1 {
            let diff = (p - parsed[idx - 1].1).abs();

            if diff == 0 || diff > 3 {
                safe = false;
            }
        }

        if idx + 1 < parsed.len() {
            let diff = (p - parsed[idx + 1].1).abs();

            match (increasing, p, parsed[idx + 1].1) {
                (Some(true), c, n) => {
                    if c > n {
                        safe = false;
                    }
                }
                (Some(false), c, n) => {
                    if c < n {
                        safe = false;
                    }
                }
                _ => {
                    println!("{:?}, {}, {}", increasing, p, parsed[idx + 1].1);
                    panic!("shid");
                }
            };

            if diff == 0 || diff > 3 {
                safe = false;
            }
        }
        ret.push(safe);
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two_bug() {
        let result = part_two("23 20 18 15 14 7");
        assert_eq!(result, Some(1));
    }
}
