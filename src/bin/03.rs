use std::rc::Rc;

use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let regex = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
    let regex2 = Regex::new(r"[0-9]{1,3},[0-9]{1,3}").unwrap();
    let captures = regex.find_iter(input);

    let regex2 = Rc::new(regex2);

    Some(
        captures
            .map(|c| {
                let c2 = regex2.clone().captures(c.into()).unwrap();
                let x = c2.iter().next().unwrap().unwrap();

                let values = x
                    .as_str()
                    .split(",")
                    .map(|v| v.parse::<u32>().unwrap())
                    .collect::<Vec<_>>();

                values[0] * values[1]
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let regex = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
    let regex2 = Regex::new(r"[0-9]{1,3},[0-9]{1,3}").unwrap();
    let do_dont = Regex::new(r"(do\(\)|don't\(\))").unwrap();
    let do_donts = do_dont.find_iter(input).collect::<Vec<_>>();
    let captures = regex.find_iter(input);

    let regex2 = Rc::new(regex2);

    Some(
        captures
            .map(|c| {
                let prev = do_donts
                    .clone()
                    .into_iter()
                    .filter(|d| d.start() < c.start())
                    .last();

                if prev.is_some() && prev.unwrap().as_str() == "don't()" {
                    return 0;
                }

                let c2 = regex2.clone().captures(c.into()).unwrap();
                let x = c2.iter().next().unwrap().unwrap();

                let values = x
                    .as_str()
                    .split(",")
                    .map(|v| v.parse::<u32>().unwrap())
                    .collect::<Vec<_>>();

                values[0] * values[1]
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
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
