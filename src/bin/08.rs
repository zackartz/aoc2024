use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<usize> {
    let mut frequency_map = HashMap::new();
    let mut ret_map = HashSet::new();

    let mut map = input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c.is_alphanumeric() {
                        frequency_map.entry(c).or_insert(Vec::new()).push((x, y));
                    }
                    c
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    frequency_map.iter().for_each(|(_frequency, coords)| {
        coords
            .iter()
            .map(|(x, y)| (*x as isize, *y as isize))
            .cartesian_product(coords.iter().map(|(x, y)| (*x as isize, *y as isize)))
            .for_each(|x| {
                if x.0 == x.1 {
                    return;
                }
                let (a_x, a_y) = x.0;
                let (b_x, b_y) = x.1;

                let diff_x = a_x - b_x;
                let diff_y = a_y - b_y;

                let a1 = (b_x - diff_x, b_y - diff_y);
                let a2 = (a_x + diff_x, a_y + diff_y);

                if check_bounds(map[0].len(), map.len(), a1.0, a1.1) {
                    ret_map.insert(a1);
                    if map[a1.1 as usize][a1.0 as usize] == '.' {
                        map[a1.1 as usize][a1.0 as usize] = '#'
                    }
                }

                if check_bounds(map[0].len(), map.len(), a2.0, a2.1) {
                    ret_map.insert(a2);
                    if map[a2.1 as usize][a2.0 as usize] == '.' {
                        map[a2.1 as usize][a2.0 as usize] = '#'
                    }
                }
            });
    });

    Some(ret_map.len())
}

fn check_bounds(width: usize, height: usize, x: isize, y: isize) -> bool {
    if check_bound(width, x) && check_bound(height, y) {
        return true;
    }
    false
}

fn check_bound(max: usize, value: isize) -> bool {
    if value >= 0 && (value as usize) < max {
        return true;
    }
    false
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut frequency_map = HashMap::new();
    let mut ret_map = HashSet::new();

    let mut map = input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c.is_alphanumeric() {
                        frequency_map.entry(c).or_insert(Vec::new()).push((x, y));
                    }
                    c
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    frequency_map.iter().for_each(|(_frequency, coords)| {
        coords
            .iter()
            .map(|(x, y)| (*x as isize, *y as isize))
            .cartesian_product(coords.iter().map(|(x, y)| (*x as isize, *y as isize)))
            .for_each(|x| {
                if x.0 == x.1 {
                    return;
                }

                if coords.len() > 1 {
                    ret_map.insert(x.0);
                    ret_map.insert(x.1);
                }

                let (a_x, a_y) = x.0;
                let (b_x, b_y) = x.1;

                let diff_x = a_x - b_x;
                let diff_y = a_y - b_y;

                let mut a1 = (b_x, b_y);
                let mut a2 = (a_x, a_y);

                let width = map[0].len();
                let height = map[1].len();

                while check_bounds(width, height, a1.0, a1.1)
                    || check_bounds(width, height, a2.0, a2.1)
                {
                    a1 = (a1.0 - diff_x, a1.1 - diff_y);
                    a2 = (a2.0 + diff_x, a2.1 + diff_y);
                    if check_bounds(map[0].len(), map.len(), a1.0, a1.1) {
                        ret_map.insert(a1);
                        if map[a1.1 as usize][a1.0 as usize] == '.' {
                            map[a1.1 as usize][a1.0 as usize] = '#'
                        }
                    }

                    if check_bounds(map[0].len(), map.len(), a2.0, a2.1) {
                        ret_map.insert(a2);
                        if map[a2.1 as usize][a2.0 as usize] == '.' {
                            map[a2.1 as usize][a2.0 as usize] = '#'
                        }
                    }
                }
            });
    });

    Some(ret_map.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
