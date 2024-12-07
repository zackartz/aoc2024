use std::{collections::HashSet, hash::Hash};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

advent_of_code::solution!(6);

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

impl From<(usize, usize)> for Point {
    fn from(value: (usize, usize)) -> Self {
        Self {
            x: value.0 as isize,
            y: value.1 as isize,
        }
    }
}

impl From<(isize, isize)> for Point {
    fn from(value: (isize, isize)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl From<(i32, i32)> for Point {
    fn from(value: (i32, i32)) -> Self {
        Self {
            x: value.0 as isize,
            y: value.1 as isize,
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    const VALUES_ASCII: [char; 4] = ['v', '^', '<', '>'];

    fn to_ascii(self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn offset(&self, point: Point) -> Point {
        let Point { x, y } = point;
        match self {
            Direction::Up => (x, y - 1).into(),
            Direction::Down => (x, y + 1).into(),
            Direction::Left => (x - 1, y).into(),
            Direction::Right => (x + 1, y).into(),
        }
    }
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '>' => Direction::Right,
            '<' => Direction::Left,
            _ => panic!("aaa"),
        }
    }
}

impl From<Direction> for char {
    fn from(value: Direction) -> Self {
        value.to_ascii()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut unique = HashSet::new();
    let map = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (guard, point) = map
        .iter()
        .enumerate()
        .find_map(|(y, x)| {
            let v = x
                .iter()
                .enumerate()
                .find(|x| Direction::VALUES_ASCII.contains(x.1));

            if let Some(v) = v {
                return Some((v.1, (v.0, y)));
            }

            None
        })
        .unwrap();

    unique.insert(point.into());

    let mut guard = std::convert::Into::<Direction>::into(*guard);

    let mut current_point: Point = point.into();

    assert!(!map.is_empty());

    loop {
        let new = guard.offset(current_point);
        match check_point(map[0].len(), map.len(), new) {
            true if map[new.y as usize][new.x as usize] == '#' => guard = guard.turn_right(),
            false => break,
            _ => {
                // println!("traversed: {:?}", new);
                unique.insert(new);
                current_point = new;
            }
        }
    }

    Some(unique.len())
}

fn check_point(width: usize, height: usize, point: Point) -> bool {
    if check_offset(width, point.x) && check_offset(height, point.y) {
        return true;
    }

    false
}

fn check_offset(max: usize, attempt: isize) -> bool {
    if attempt >= 0 && (attempt as usize) < max {
        return true;
    }

    false
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    assert!(!map.is_empty());

    let mut valid: HashSet<Point> = HashSet::new();

    let (guard, point) = map
        .iter()
        .enumerate()
        .find_map(|(y, x)| {
            let v = x
                .iter()
                .enumerate()
                .find(|x| Direction::VALUES_ASCII.contains(x.1));

            if let Some(v) = v {
                return Some((v.1, (v.0, y)));
            }

            None
        })
        .unwrap();

    let mut guard = std::convert::Into::<Direction>::into(*guard);

    let mut current_point: Point = point.into();

    loop {
        let new = guard.offset(current_point);
        match check_point(map[0].len(), map.len(), new) {
            true if map[new.y as usize][new.x as usize] == '#' => guard = guard.turn_right(),
            false => break,
            _ => {
                // println!("traversed: {:?}", new);
                valid.insert(new);
                current_point = new;
            }
        }
    }

    let unique = valid
        .par_iter()
        .map(|p| {
            let mut unique: HashSet<Point> = HashSet::new();
            let c = map[p.y as usize][p.x as usize];
            let mut traversed = HashSet::new();
            let (guard, point) = map
                .iter()
                .enumerate()
                .find_map(|(y, x)| {
                    let v = x
                        .iter()
                        .enumerate()
                        .find(|x| Direction::VALUES_ASCII.contains(x.1));

                    if let Some(v) = v {
                        return Some((v.1, (v.0, y)));
                    }

                    None
                })
                .unwrap();

            let mut guard = std::convert::Into::<Direction>::into(*guard);

            let mut point: Point = point.into();
            if c == '.' {
                let new_obstacle: Point = (p.x, p.y).into();

                loop {
                    let new = guard.offset(point);
                    match check_point(map[0].len(), map.len(), new) {
                        true if map[new.y as usize][new.x as usize] == '#'
                            || new_obstacle == (new.x, new.y).into() =>
                        {
                            guard = guard.turn_right();
                        }
                        false => {
                            break;
                        }
                        _ => {
                            if traversed.contains(&(guard, new)) {
                                unique.insert(new_obstacle);
                                break;
                            }
                            traversed.insert((guard, new));
                            point = new;
                        }
                    }
                }
            }

            unique
        })
        .flatten()
        .collect::<HashSet<Point>>();

    Some(unique.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
