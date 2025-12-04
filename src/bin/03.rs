advent_of_code::solution!(3);

fn bucket_brigade(bank: &mut Vec<u32>, new_digit: u32) {
    let mut old = new_digit;
    for i in 0..bank.len() {
        if bank[i] <= old {
            let temp = old;
            old = bank[i];
            bank[i] = temp;
        } else {
            break;
        }
    }
}

fn get_bank(input: &str, num_batteries: usize) -> u64 {
    input
        .lines()
        .map(|s| {
            let digits: Vec<u32> = s.chars().map(|c| c.to_digit(10).unwrap()).collect();

            // Create battery bank
            let mut bank: Vec<u32> = Vec::new();
            for i in s.len() - num_batteries..s.len() {
                bank.push(digits[i]);
            }

            for digit in digits[0..s.len() - num_batteries].iter().rev() {
                if *digit >= bank[0] {
                    bucket_brigade(&mut bank, *digit);
                }
            }

            // Sum total bank output
            bank.iter()
                .fold(0, |total, digit| total * 10 + *digit as u64)
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(get_bank(input, 2))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(get_bank(input, 12))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
