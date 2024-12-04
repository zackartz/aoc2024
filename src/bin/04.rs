use std::rc::Rc;

advent_of_code::solution!(4);

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
    NorthEast,
    SouthEast,
    SouthWest,
    NorthWest,
}

#[derive(Debug, Clone, Copy)]
struct Coords {
    x: isize,
    y: isize,
}

impl From<(usize, usize)> for Coords {
    fn from(value: (usize, usize)) -> Self {
        Self {
            x: value.0.try_into().unwrap(),
            y: value.1.try_into().unwrap(),
        }
    }
}

impl From<(isize, isize)> for Coords {
    fn from(value: (isize, isize)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl Direction {
    const VALUES: [Self; 8] = [
        Self::North,
        Self::South,
        Self::East,
        Self::West,
        Self::NorthEast,
        Self::SouthEast,
        Self::SouthWest,
        Self::NorthWest,
    ];

    fn offset(&self, current: Coords) -> Coords {
        let Coords { x, y } = current;
        let ret = match self {
            Direction::North => (x, y - 1),
            Direction::South => (x, y + 1),
            Direction::East => (x + 1, y),
            Direction::West => (x - 1, y),
            Direction::NorthEast => (x + 1, y - 1),
            Direction::SouthEast => (x + 1, y + 1),
            Direction::SouthWest => (x - 1, y + 1),
            Direction::NorthWest => (x - 1, y - 1),
        };

        ret.into()
    }

    fn inverse(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
            Direction::NorthEast => Direction::SouthWest,
            Direction::SouthEast => Direction::NorthWest,
            Direction::SouthWest => Direction::NorthEast,
            Direction::NorthWest => Direction::SouthEast,
        }
    }
}

const SEARCH_STR: &str = "XMAS";

pub fn part_one(input: &str) -> Option<u32> {
    let vec = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let vec = Rc::new(vec);

    let start_char = SEARCH_STR.chars().next().unwrap();
    let mut total = 0;

    for (y, line) in vec.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == start_char {
                for dir in Direction::VALUES {
                    let res = find(vec.clone(), dir, (x, y).into());
                    if res.len() == 3 {
                        total += 1;
                    }
                }
            }
        }
    }

    Some(total)
}

fn find(vec: Rc<Vec<Vec<char>>>, dir: Direction, start: Coords) -> Vec<Coords> {
    assert!(!vec.is_empty());
    let mut traversed = vec![];
    let mut current = start;
    let mut pos = 1;

    let width = vec[0].len();
    let height = vec.len();

    while pos < 4 {
        current = dir.offset(current);

        if !check_offset(width, current.x) || !check_offset(height, current.y) {
            return traversed;
        }

        if vec[current.y as usize][current.x as usize] == SEARCH_STR.chars().nth(pos).unwrap() {
            pos += 1;
            traversed.push(current);
        } else {
            return traversed;
        }
    }

    traversed
}

fn check_offset(size: usize, new: isize) -> bool {
    if new >= 0 && new < size as isize {
        return true;
    }

    false
}

fn find_mas(vec: Rc<Vec<Vec<char>>>, start: Coords) -> bool {
    let dirs_check = [Direction::NorthEast, Direction::SouthEast];

    let width = vec[0].len();
    let height = vec.len();

    dirs_check
        .iter()
        .map(|d| {
            let chars = ['S', 'M'];

            let first_check = d.offset(start);

            if !check_offset(width, first_check.x) || !check_offset(height, first_check.y) {
                return false;
            }

            let first = vec.clone()[first_check.y as usize][first_check.x as usize];
            if !chars.contains(&first) {
                return false;
            }

            let second_check = d.inverse().offset(start);

            if !check_offset(width, second_check.x) || !check_offset(height, second_check.y) {
                return false;
            }

            let second = vec.clone()[second_check.y as usize][second_check.x as usize];

            matches!((first, second), ('S', 'M') | ('M', 'S'))
        })
        .filter(|d| !(*d))
        .count()
        == 0
}

pub fn part_two(input: &str) -> Option<u32> {
    let vec = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let vec = Rc::new(vec);

    let start_char = 'A';
    let mut total = 0;

    for (y, line) in vec.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == start_char && find_mas(vec.clone(), (x, y).into()) {
                total += 1;
            }
        }
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
