use std::fmt;
advent_of_code::solution!(6);

struct Worksheet {
    operations: Vec<String>,
    numbers: Vec<Vec<u64>>,
}

impl fmt::Display for Worksheet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.operations.len() {
            for j in 0..self.numbers[i].len() {
                if j != self.numbers[i].len() - 1 {
                    write!(f, "{} {} ", self.numbers[i][j], self.operations[i])?;
                } else {
                    write!(f, "{}", self.numbers[i][j])?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn parse_pt1(input: &str) -> Worksheet {
    let mut lines = input.lines().rev();
    let operations: Vec<String> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    let numbers: Vec<Vec<u64>> = lines
        .map(|s| {
            s.split_whitespace()
                .map(|number| number.parse::<u64>().unwrap())
                .collect()
        })
        .collect();

    Worksheet {
        operations,
        numbers,
    }
}

fn solve_worksheet_pt1(worksheet: Worksheet) -> u64 {
    let mut result: Vec<u64> = worksheet.numbers[0].clone();
    worksheet.numbers[1..].iter().for_each(|numbers| {
        numbers
            .iter()
            .enumerate()
            .for_each(|(i, number)| match worksheet.operations[i].as_str() {
                "+" => result[i] += number,
                "*" => result[i] *= number,
                _ => panic!("Unexpected operation"),
            })
    });
    result.iter().sum()
}

fn parse_pt2(input: &str) -> Worksheet {
    let mut lines = input.lines().rev();
    let mut all_numbers: Vec<Vec<u64>> = Vec::new();
    let operations = lines.next().unwrap();
    let operations: Vec<String> = operations
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    let max_length = lines.clone().map(|line| line.len()).max().unwrap();

    let char_2d_array: Vec<Vec<char>> = lines
        .rev()
        .map(|line| {
            format!("{:width$}", line, width = max_length)
                .chars()
                .collect()
        })
        .collect();

    let mut numbers: Vec<u64> = Vec::new();
    for j in 0..max_length {
        let mut number: u64 = 0;
        for i in 0..char_2d_array.len() {
            let digit = char_2d_array[i][j].to_digit(10);
            match digit {
                Some(x) => number = number * 10 + x as u64,
                None => continue,
            }
        }
        match number {
            0 => {
                all_numbers.push(numbers);
                numbers = Vec::new();
            }
            x => numbers.push(x),
        }
    }
    all_numbers.push(numbers);

    Worksheet {
        operations,
        numbers: all_numbers,
    }
}

fn solve_worksheet_pt2(worksheet: Worksheet) -> u64 {
    worksheet
        .operations
        .iter()
        .enumerate()
        .map(|(i, operator)| match operator.as_str() {
            "+" => worksheet.numbers[i].iter().sum::<u64>(),
            "*" => worksheet.numbers[i].iter().product(),
            _ => panic!("Invalid operation"),
        })
        .sum::<u64>()
}

pub fn part_one(input: &str) -> Option<u64> {
    let worksheet = parse_pt1(input);
    Some(solve_worksheet_pt1(worksheet))
}

pub fn part_two(input: &str) -> Option<u64> {
    let worksheet = parse_pt2(input);
    Some(solve_worksheet_pt2(worksheet))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
