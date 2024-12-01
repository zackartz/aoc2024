use std::error::Error;
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
    let out = input.lines().map(|l| {
        l.split("   ")
            .map(|c| c.to_string().parse::<i32>().unwrap())
            .collect::<Vec<_>>()
    });

    let left_vec = out.clone().map(|v| v[0]);
    let right_vec = out.map(|v| v[1]);

    Some(
        left_vec
            .map(|left| {
                let count = parse_i32(right_vec.clone().filter(|x| *x == left).count()).unwrap();

                left * count
            })
            .sum::<i32>()
            .try_into()
            .unwrap(),
    )
}

// usize is a u16 or u32, which always fits in a u32
#[cfg(any(target_pointer_width = "16", target_pointer_width = "32"))]
fn parse_i32(a: usize) -> Result<i32, Box<dyn Error>> {
    Ok(a as i32)
}

// usize is a u64, which might be too big
#[cfg(target_pointer_width = "64")]
fn parse_i32(a: usize) -> Result<i32, Box<dyn Error>> {
    if a > i32::MAX as usize {
        panic!("size bad")
    } else {
        Ok(a as i32)
    }
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
