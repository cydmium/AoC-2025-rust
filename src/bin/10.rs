use good_lp::{Expression, Solution, SolverModel, constraint, default_solver, variables};
use std::{cmp, collections::HashSet, usize};

advent_of_code::solution!(10);

struct Machine {
    lights: Vec<usize>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<usize>,
}

impl Machine {
    fn generate_lights_matrix(&self) -> Vec<Vec<usize>> {
        let mut matrix: Vec<Vec<usize>> = Vec::new();
        for _ in 0..self.lights.len() {
            matrix.push(vec![0; self.buttons.len() + 1]);
        }
        for (col, button) in self.buttons.iter().enumerate() {
            for row in button.iter() {
                matrix[*row][col] = 1;
            }
        }

        let last_col = self.buttons.len();
        for (row, on) in self.lights.iter().enumerate() {
            if *on == 1 {
                matrix[row][last_col] = 1;
            }
        }

        matrix
    }

    fn generate_joltage_matrix(&self) -> Vec<Vec<usize>> {
        let mut matrix: Vec<Vec<usize>> = Vec::new();
        for _ in 0..self.lights.len() {
            matrix.push(vec![0; self.buttons.len() + 1]);
        }
        for (col, button) in self.buttons.iter().enumerate() {
            for row in button.iter() {
                matrix[*row][col] = 1;
            }
        }

        let last_col = self.buttons.len();
        for (row, joltage) in self.joltages.iter().enumerate() {
            matrix[row][last_col] = *joltage;
        }

        matrix
    }
}

fn gaussian_elim_mod2(matrix: &mut Vec<Vec<usize>>) -> HashSet<usize> {
    'outer: for i in 0..cmp::min(matrix.len(), matrix[0].len()) {
        let mut pivot_row = i;
        while matrix[pivot_row][i] == 0 {
            pivot_row += 1;
            if pivot_row == matrix.len() {
                continue 'outer;
            }
        }
        if pivot_row != i {
            let temp: Vec<usize> = matrix[i].clone();
            matrix[i] = matrix[pivot_row].clone();
            matrix[pivot_row] = temp;
        }

        for j in i + 1..matrix.len() {
            if matrix[j][i] == 1 {
                matrix[j] = add_mod2(&matrix[i], &matrix[j]);
            }
        }
    }

    // Backwards substitution
    let mut free_variables: HashSet<usize> = HashSet::new();
    for i in (0..cmp::min(matrix.len(), matrix[0].len())).rev() {
        let pivot: Option<usize> = matrix[i].iter().position(|&d| d == 1);
        let pivot: usize = match pivot {
            Some(x) => x,
            None => continue,
        };
        for j in 0..i {
            if matrix[j][pivot] == 1 {
                matrix[j] = add_mod2(&matrix[i], &matrix[j]);
            }
        }
        for j in pivot + 1..matrix[0].len() - 1 {
            if matrix[i][j] == 1 {
                free_variables.insert(j);
            }
        }
    }
    free_variables
}

fn get_combinations(free_variables: &Vec<usize>) -> Vec<Vec<usize>> {
    let num_free_variables: usize = free_variables.len();
    let num_combinations = 2usize.pow(num_free_variables as u32);
    let mut combinations: Vec<Vec<usize>> = Vec::with_capacity(num_combinations);

    for i in 0..num_combinations {
        let mut current_vec: Vec<usize> = Vec::with_capacity(num_free_variables);
        for j in 0..num_free_variables {
            // Check the jth bit of integer i
            let is_pushed: usize = (i >> j) & 1;
            current_vec.push(is_pushed);
        }
        combinations.push(current_vec);
    }

    combinations
}

fn solve_mod2(matrix: &mut Vec<Vec<usize>>) -> u64 {
    let free_variables = gaussian_elim_mod2(matrix);
    let rows = matrix.len();
    let cols = matrix[0].len();
    let free_variable_vec: Vec<usize> = free_variables.iter().map(|x| *x).collect();
    let combinations: Vec<Vec<usize>> = get_combinations(&free_variable_vec);
    let mut min_score: usize = usize::MAX;
    for combination in combinations.iter() {
        let mut temp_matrix = matrix.clone();
        let mut score: usize = 0;
        for (var_index, value) in free_variable_vec.iter().zip(combination) {
            score += value;
            for i in 0..rows {
                if temp_matrix[i][*var_index] == 1 {
                    temp_matrix[i][*var_index] = 0;
                    temp_matrix[i][cols - 1] = (temp_matrix[i][cols - 1] + value) % 2;
                }
            }
        }
        let soln: Option<usize> = solve_reduced_mod2(&temp_matrix);
        match soln {
            Some(x) => score += x,
            None => continue,
        }
        if score < min_score {
            min_score = score;
        }
    }

    min_score as u64
}

fn solve_reduced_mod2(matrix: &Vec<Vec<usize>>) -> Option<usize> {
    let mut sum = 0;
    for row in matrix.iter() {
        let row_sum: usize = row.iter().sum();
        if row_sum == 1 && *row.last().unwrap() == 1 {
            return None;
        }
        if row_sum == 2 {
            sum += 1;
        }
    }
    Some(sum)
}

fn add_mod2(v1: &Vec<usize>, v2: &Vec<usize>) -> Vec<usize> {
    assert!(v1.len() == v2.len());
    v1.iter()
        .enumerate()
        .map(|(i, x)| (x + v2[i]) % 2)
        .collect()
}

fn solve(matrix: &mut Vec<Vec<usize>>) -> u64 {
    let cols = matrix[0].len();
    let num_vars: usize = cols - 1;
    variables!(problem: 0<= x[num_vars] (integer));
    let objective: Expression = x.iter().sum();
    let mut solver = problem.minimise(objective).using(default_solver);
    for row in matrix.iter() {
        let expr: Expression = x
            .iter()
            .enumerate()
            .map(|(i, &val)| val * row[i] as f64)
            .sum();
        solver.add_constraint(constraint!(expr == row[cols - 1] as f64));
    }
    let solution = solver.solve().unwrap();
    let x_values: Vec<f64> = x.iter().map(|&var| solution.value(var)).collect();

    // Need to sum as floats then round to avoid losing .999999... values
    let float_val: f64 = x_values.iter().sum();
    float_val.round() as u64
}

fn parse(input: &str) -> Vec<Machine> {
    input
        .lines()
        .map(|s| {
            let mut parts = s.split_whitespace();
            let lights: Vec<usize> = parts
                .next()
                .unwrap()
                .trim_matches(|c| c == '[' || c == ']')
                .chars()
                .filter_map(|c| if c == '#' { Some(1) } else { Some(0) })
                .collect();
            let mut buttons: Vec<Vec<usize>> = parts
                .clone()
                .rev()
                .skip(1)
                .into_iter()
                .map(|button| {
                    button
                        .trim_matches(|c| c == '(' || c == ')')
                        .split(',')
                        .map(|s| s.parse().unwrap())
                        .collect()
                })
                .collect();
            buttons.reverse();
            let joltages: Vec<usize> = parts
                .last()
                .unwrap()
                .trim_matches(|c| c == '{' || c == '}')
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect();
            Machine {
                lights,
                buttons,
                joltages,
            }
        })
        .collect()
}

#[allow(dead_code)]
fn print_matrix(matrix: &Vec<Vec<usize>>) {
    for row in matrix.iter() {
        for (i, elem) in row.iter().enumerate() {
            if i == row.len() - 1 {
                print!("| {elem}");
            } else {
                print!("{elem} ");
            }
        }
        print!("\n");
    }
    print!("\n");
}

pub fn part_one(input: &str) -> Option<u64> {
    let machines: Vec<Machine> = parse(input);
    Some(
        machines
            .iter()
            .map(|machine| {
                let mut matrix: Vec<Vec<usize>> = machine.generate_lights_matrix();
                solve_mod2(&mut matrix)
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let machines: Vec<Machine> = parse(input);
    Some(
        machines
            .iter()
            .map(|machine| {
                let mut matrix: Vec<Vec<usize>> = machine.generate_joltage_matrix();
                solve(&mut matrix)
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}
