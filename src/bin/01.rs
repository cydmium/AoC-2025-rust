advent_of_code::solution!(1);

fn get_rotation(turn: &str) -> i32 {
    match turn.chars().next() {
        Some('R') => turn[1..].parse::<i32>().unwrap(),
        Some('L') => -1 * turn[1..].parse::<i32>().unwrap(),
        _ => panic!("Received invalid turn direction"),
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let mut count = 0;
    let mut lock_value = 50;

    for turn in input.lines() {
        let rotation = get_rotation(turn);
        lock_value = (lock_value + rotation).rem_euclid(100);
        if lock_value == 0 {
            count += 1;
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut count = 0;
    let mut lock_value = 50;

    for turn in input.lines() {
        let mut rotation = get_rotation(turn);

        count += rotation.abs().div_euclid(100); // count the full turns
        rotation = rotation % 100; // get net rotation

        // if we moved past 0 on the net rotation add 1 to counter
        if ((lock_value + rotation) <= 0 && lock_value != 0) || (lock_value + rotation) >= 100 {
            count += 1;
        }
        lock_value = (lock_value + rotation).rem_euclid(100); // update position
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
