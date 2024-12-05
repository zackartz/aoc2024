use std::{cmp, collections::HashMap, isize, rc::Rc};

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

            let e = map.entry(ret[0]).or_insert_with(Vec::new);
            e.push(ret[1]);
        });
    }

    let map = Rc::new(map);

    Some(
        parsed
            .next()
            .unwrap()
            .lines()
            .map(|l| {
                let nums = l
                    .split(",")
                    .map(str::parse)
                    .filter_map(Result::ok)
                    .collect::<Vec<isize>>();
                let mut total = 0;

                let mut ok = true;

                for (idx, n) in nums.iter().enumerate() {
                    if let Some(rules) = map.get(n) {
                        if rules
                            .iter()
                            .map(|r| nums[0..idx].contains(r))
                            .filter(|b| *b)
                            .count()
                            != 0
                        {
                            ok = false;
                        }
                    }
                }

                total += match ok {
                    true => nums[nums.len() / 2] as usize,
                    false => 0,
                };

                total
            })
            .sum(),
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

            let e = map.entry(ret[0]).or_insert_with(Vec::new);
            e.push(ret[1]);
        });
    }

    let map = Rc::new(map);

    Some(
        parsed
            .next()
            .unwrap()
            .lines()
            .map(|l| {
                let nums = l
                    .split(",")
                    .map(str::parse)
                    .filter_map(Result::ok)
                    .collect::<Vec<isize>>();

                let mut ok = true;

                let mut bad = vec![];

                for (idx, n) in nums.iter().enumerate() {
                    if let Some(rules) = map.get(n) {
                        let n_clone = nums.clone();
                        if rules
                            .iter()
                            .map(|r| n_clone[0..idx].contains(r))
                            .filter(|b| *b)
                            .count()
                            > 0
                        {
                            ok = false;
                        }
                    }
                }

                if !ok {
                    bad.push(nums);
                }

                bad.into_iter()
                    .map(|mut a| {
                        a.sort_by(|a, b| match map.get(a) {
                            Some(rules_a) if rules_a.contains(b) => cmp::Ordering::Less,

                            _ => cmp::Ordering::Equal,
                        });
                        a[a.len() / 2] as usize
                    })
                    .sum::<usize>()
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
