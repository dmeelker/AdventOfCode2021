use std::fs;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let values = parse_input(&input);

    let part1 = part1(&values);
    let part2 = part2(&values);

    println!("Part 1: {} Part 2: {}", part1, part2);
}

fn parse_input(input: &str) -> Vec<i32> {
    input.split(',').map(|p| p.parse().unwrap()).collect()
}

fn part1(values: &[i32]) -> usize {
    simulate_fish(values, 80)
}

fn part2(values: &[i32]) -> usize {
    simulate_fish(values, 256)
}

fn simulate_fish(values: &[i32], days: usize) -> usize {
    let mut fish: [usize; 9] = [0; 9];

    for group in values.iter().counts_by(|v| v).iter() {
        fish[**group.0 as usize] = *group.1;
    }

    (0..days).fold(fish, |fish, _| simulate_day(&fish))
        .iter().sum()
}

fn simulate_day(input: &[usize; 9]) -> [usize; 9] {
    let mut new_fish: [usize; 9] = [0; 9];

    for i in 1..input.len() {
        new_fish[i-1] = input[i];
    }

    new_fish[8] = input[0];
    new_fish[6] += input[0];
    new_fish
}