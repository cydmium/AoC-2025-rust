advent_of_code::solution!(2);

fn p1_invalid(x: u64) -> bool {
    let s = x.to_string();
    let mid_point = s.len() / 2;
    let (first_half, second_half) = s.split_at(mid_point);
    s.len() % 2 == 0 && first_half == second_half
}

fn p2_invalid(x: u64) -> bool {
    let s = x.to_string();

    for i in 1..=(s.len() / 2) {
        if s.len() % i != 0 {
            continue;
        }
        let num_duplicates: usize = s.len() / i;
        if s[..i].repeat(num_duplicates) == s {
            return true;
        }
    }
    false
}

fn get_invalid_sum(input: &str, checker: fn(u64) -> bool) -> u64 {
    let ids: u64 = input
        .split(",")
        .map(|x| {
            let mut range = x.split("-");
            let min: u64 = range.next().unwrap().parse().unwrap();
            let max: u64 = range.next().unwrap().parse().unwrap();

            (min..=max)
                .filter(|x| checker(*x))
                .fold(0, |acc, x| acc + x)
        })
        .sum();
    ids
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(get_invalid_sum(input, p1_invalid))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(get_invalid_sum(input, p2_invalid))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
