use std::{cmp, collections::HashMap};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<usize> {
    let mut parsed = input.split("\n\n");
    let mut map = HashMap::new();

    if let Some(l) = parsed.next() {
        l.lines().for_each(|l| {
            let ret = l
                .split("|")
                .map(str::parse)
                .filter_map(Result::ok)
                .collect::<Vec<isize>>();

            map.entry(ret[0]).or_insert_with(Vec::new).push(ret[1]);
        });
    }

    Some(
        parsed
            .next()
            .unwrap()
            .lines()
            .map(|l| l.split(",").map(str::parse).filter_map(Result::ok))
            .filter(|nums| {
                nums.clone()
                    .is_sorted_by(|a, b| matches!(map.get(a), Some(rules_a) if rules_a.contains(b)))
            })
            .map(|nums| {
                let v = nums.collect::<Vec<_>>();
                v[v.len() / 2] as usize
            })
            .sum::<usize>(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut parsed = input.split("\n\n");
    let mut map = HashMap::new();

    if let Some(l) = parsed.next() {
        l.lines().for_each(|l| {
            let ret = l
                .split("|")
                .map(str::parse)
                .filter_map(Result::ok)
                .collect::<Vec<isize>>();

            map.entry(ret[0]).or_insert_with(Vec::new).push(ret[1]);
        });
    }

    Some(
        parsed
            .next()
            .unwrap()
            .lines()
            .map(|l| l.split(",").map(str::parse).filter_map(Result::ok))
            .filter(|nums| {
                !nums
                    .clone()
                    .is_sorted_by(|a, b| matches!(map.get(a), Some(rules_a) if rules_a.contains(b)))
            })
            .map(|nums| {
                let mut v = nums.collect::<Vec<_>>();
                v.sort_by(|a, b| match map.get(a) {
                    Some(rules_a) if rules_a.contains(b) => cmp::Ordering::Less,
                    _ => cmp::Ordering::Equal,
                });
                v[v.len() / 2] as usize
            })
            .sum::<usize>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
