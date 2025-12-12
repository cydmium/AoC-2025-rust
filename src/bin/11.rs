use memoize::memoize;
use std::collections::HashMap;

advent_of_code::solution!(11);

fn parse(input: &str) -> HashMap<String, Vec<String>> {
    let mut connections: HashMap<String, Vec<String>> = HashMap::new();
    for line in input.lines() {
        let mut parts = line.split(":");
        let key: String = parts.next().unwrap().to_string();
        let value: Vec<String> = parts
            .next()
            .unwrap()
            .split_whitespace()
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        connections.insert(key, value);
    }
    connections
}

#[memoize(Ignore: connections)]
fn count_connections_pt1(key: String, connections: &HashMap<String, Vec<String>>) -> u64 {
    let outputs: &Vec<String> = connections.get(&key).unwrap();
    if *outputs.iter().next().unwrap() == String::from("out") {
        return 1;
    }
    outputs
        .iter()
        .map(|output| count_connections_pt1(output.to_string(), connections))
        .sum()
}

#[memoize(Ignore: connections)]
fn count_connections_pt2(
    key: String,
    connections: &HashMap<String, Vec<String>>,
    mut found_dac: bool,
    mut found_fft: bool,
) -> u64 {
    let outputs: &Vec<String> = connections.get(&key).unwrap();
    if *outputs.iter().next().unwrap() == String::from("out") {
        if found_dac && found_fft {
            return 1;
        } else {
            return 0;
        }
    }
    if key == "fft" {
        found_fft = true;
    } else if key == "dac" {
        found_dac = true;
    }
    outputs
        .iter()
        .map(|output| count_connections_pt2(output.to_string(), connections, found_dac, found_fft))
        .sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    let connections = parse(input);
    Some(count_connections_pt1(String::from("you"), &connections))
}

pub fn part_two(input: &str) -> Option<u64> {
    let connections = parse(input);
    Some(count_connections_pt2(
        String::from("svr"),
        &connections,
        false,
        false,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2));
    }
}
