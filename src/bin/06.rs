use std::{
    char,
    collections::{HashMap, HashSet},
    ops::Add,
};

advent_of_code::solution!(6);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Move,
    Turn,
    Finish,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Coordinate {
    x: isize,
    y: isize,
}

impl Add for Coordinate {
    type Output = Coordinate;

    fn add(self: Coordinate, other: Coordinate) -> Coordinate {
        Coordinate {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

pub fn get_next_direction(cur: Direction) -> Direction {
    match cur {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
    }
}

pub fn get_next_coordinate(coordinate: Coordinate, direction: Direction) -> Coordinate {
    coordinate
        + match direction {
            Direction::North => Coordinate { x: 0, y: -1 },
            Direction::East => Coordinate { x: 1, y: 0 },
            Direction::South => Coordinate { x: 0, y: 1 },
            Direction::West => Coordinate { x: -1, y: 0 },
        }
}

pub fn get_char_at_coordinate(map: &str, coordinate: Coordinate) -> Option<char> {
    let lines = map.lines().collect::<Vec<_>>();
    let column_len = lines.len() as isize;
    let row_len = lines[0].len() as isize;

    if !((0..row_len).contains(&coordinate.x) && (0..column_len).contains(&coordinate.y)) {
        return None;
    }

    Some(lines[coordinate.y as usize].as_bytes()[coordinate.x as usize] as char)
}

pub fn get_next_command_from_char(next_char: Option<char>) -> Command {
    match next_char {
        Some('.') => Command::Move,
        Some('^') => Command::Move,
        Some('#') => Command::Turn,
        None => Command::Finish,
        _ => Command::Finish,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut cur_coord = None;
    for (y, line) in input.lines().enumerate() {
        let chars = line.chars().collect::<Vec<_>>();
        if chars.contains(&'^') {
            cur_coord = Some(Coordinate {
                x: chars.iter().position(|&c| c == '^').unwrap() as isize,
                y: y as isize,
            });
            break;
        }
    }

    let next_command = Command::Move;
    let cur_direction = Direction::North;

    Some(solve(
        input,
        cur_coord.unwrap(),
        cur_direction,
        next_command,
        HashSet::new(),
    ))
}

pub fn solve(
    map: &str,
    mut cur_coord: Coordinate,
    mut cur_direction: Direction,
    mut next_command: Command,
    mut visited: HashSet<Coordinate>,
) -> u32 {
    match next_command {
        Command::Move => {
            visited.insert(cur_coord);
            cur_coord = get_next_coordinate(cur_coord, cur_direction);
            let next_coord = get_next_coordinate(cur_coord, cur_direction);
            next_command = get_next_command_from_char(get_char_at_coordinate(map, next_coord));
            solve(map, cur_coord, cur_direction, next_command, visited)
        }
        Command::Turn => {
            cur_direction = get_next_direction(cur_direction);
            let next_coord_candidate = get_next_coordinate(cur_coord, cur_direction);
            next_command =
                get_next_command_from_char(get_char_at_coordinate(map, next_coord_candidate));
            solve(map, cur_coord, cur_direction, next_command, visited)
        }
        Command::Finish => {
            visited.insert(cur_coord);
            visited.len() as u32
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cur_coord = None;
    for (y, line) in input.lines().enumerate() {
        let chars = line.chars().collect::<Vec<_>>();
        if chars.contains(&'^') {
            cur_coord = Some(Coordinate {
                x: chars.iter().position(|&c| c == '^').unwrap() as isize,
                y: y as isize,
            });
            break;
        }
    }

    let next_command = Command::Move;
    let cur_direction = Direction::North;

    Some(solve_pt2(
        input,
        cur_coord.unwrap(),
        cur_direction,
        next_command,
        HashMap::new(),
        false,
    ))
}

pub fn solve_pt2(
    map: &str,
    mut cur_coord: Coordinate,
    mut cur_direction: Direction,
    mut next_command: Command,
    mut visited: HashMap<Coordinate, Vec<Direction>>,
    has_blocked: bool,
) -> u32 {
    match next_command {
        Command::Move => {
            let prev_visit_direction = visited.entry(cur_coord).or_insert_with(Vec::new);

            if prev_visit_direction.contains(&cur_direction) {
                return 1;
            }

            prev_visit_direction.push(cur_direction);

            cur_coord = get_next_coordinate(cur_coord, cur_direction);
            let next_coord = get_next_coordinate(cur_coord, cur_direction);
            next_command = get_next_command_from_char(get_char_at_coordinate(map, next_coord));

            let next_visit_direction = visited.entry(next_coord).or_insert_with(Vec::new);

            let mut to_add = 0;

            let next_coord_can_be_blocked = next_visit_direction.is_empty();

            if !has_blocked && next_command == Command::Move && next_coord_can_be_blocked {
                to_add = solve_pt2(
                    map,
                    cur_coord,
                    get_next_direction(cur_direction),
                    Command::Move,
                    visited.clone(),
                    true,
                );
            }
            solve_pt2(
                map,
                cur_coord,
                cur_direction,
                next_command,
                visited,
                has_blocked,
            ) + to_add
        }
        Command::Turn => {
            cur_direction = get_next_direction(cur_direction);
            let next_coord_candidate = get_next_coordinate(cur_coord, cur_direction);
            next_command =
                get_next_command_from_char(get_char_at_coordinate(map, next_coord_candidate));
            solve_pt2(
                map,
                cur_coord,
                cur_direction,
                next_command,
                visited,
                has_blocked,
            )
        }
        Command::Finish => 0,
    }
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
