use std::{cmp, fmt};

advent_of_code::solution!(5);

struct Range {
    min: u64,
    max: u64,
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use write! macro to write formatted output to the formatter
        write!(f, "{} - {}", self.min, self.max)
    }
}

struct Ingredients {
    ranges: Vec<Range>,
    ids: Vec<u64>,
}

fn merge_ranges(ranges: Vec<Range>) -> Vec<Range> {
    let mut merged_ranges: Vec<Range> = Vec::new();
    let mut current_range = Range {
        min: ranges[0].min,
        max: ranges[0].max,
    };
    for i in 1..ranges.len() {
        let new_range = Range {
            min: ranges[i].min,
            max: ranges[i].max,
        };

        if new_range.min <= current_range.max {
            current_range.max = cmp::max(current_range.max, new_range.max);
        } else {
            merged_ranges.push(current_range);
            current_range = new_range;
        }
    }
    merged_ranges.push(current_range);
    merged_ranges
}

fn parse(input: &str) -> Ingredients {
    let mut ranges: Vec<Range> = Vec::new();
    let mut ids: Vec<u64> = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        if line.contains("-") {
            let mut parts = line.split("-");
            ranges.push(Range {
                min: parts.next().unwrap().parse().unwrap(),
                max: parts.next().unwrap().parse().unwrap(),
            });
        } else {
            ids.push(line.parse().unwrap());
        }
    }

    // Sort and simplify ranges
    ranges.sort_by(|a, b| a.min.cmp(&b.min));
    ids.sort();
    let ranges = merge_ranges(ranges);
    Ingredients { ranges, ids }
}

pub fn part_one(input: &str) -> Option<u64> {
    let ingredients = parse(input);

    let mut total: u64 = 0;
    let mut range_index: usize = 0;
    let mut id_index: usize = 0;
    while range_index < ingredients.ranges.len() && id_index < ingredients.ids.len() {
        let id = ingredients.ids[id_index];
        let range = &ingredients.ranges[range_index];
        if id > range.max {
            range_index += 1;
            continue;
        }
        if id >= range.min {
            total += 1;
        }
        id_index += 1;
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let ingredients = parse(input);

    Some(
        ingredients
            .ranges
            .iter()
            .fold(0, |acc, range| acc + range.max - range.min + 1),
    )
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
        assert_eq!(result, Some(14));
    }
}
