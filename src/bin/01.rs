use std::collections::HashMap;
advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left, mut right) = input
        .lines()
        .map(|l| {
            let mut split = l.split("   ");
            let left = split.next();
            let right = split.next();

            (
                left.unwrap().parse::<i32>().unwrap(),
                right.unwrap().parse::<i32>().unwrap(),
            )
        })
        .collect::<(Vec<_>, Vec<_>)>();

    left.sort();
    right.sort();

    Some(
        left.iter()
            .zip(right)
            .map(|(left, right)| (*left - right).abs())
            .sum::<i32>()
            .try_into()
            .unwrap(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map = HashMap::new();
    let mut left_arr = Vec::with_capacity(input.lines().count());

    for line in input.lines() {
        let mut split = line.split("   ");
        let left = split.next().unwrap();
        let right = split.next().unwrap();

        let count = map.entry(right).or_insert(0);
        *count += 1;

        left_arr.push(left);
    }

    Some(
        left_arr
            .iter()
            .map(|l| l.parse::<i32>().unwrap() * (*map.get(l).unwrap_or(&0)))
            .sum::<i32>()
            .try_into()
            .unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
