use std::fs;

fn main() {
    let lines = parse_input("input.txt");
    let part1 = part1(&lines);
    let part2 = part2(&lines);

    println!("Part 1: {} Part 2: {}", part1, part2);
}

fn part1(lines: &[String]) -> usize {
    let mut gamma = String::new();
    let mut epsilon = String::new();

    for i in 0..lines[0].len() {
        let common_bit = find_most_common_bit(lines, i).unwrap();
        let least_common_bit = !common_bit;

        gamma.push(format_bit(common_bit));
        epsilon.push(format_bit(least_common_bit));
    }

    let gamma = parse_binary(gamma.as_str());
    let epsilon = parse_binary(epsilon.as_str());

    gamma * epsilon
}

fn part2(lines: &[String]) -> usize {
    let o2generator = find_o2_generator(lines);
    let co2scrubber = find_co2_scrubber(lines);

    o2generator * co2scrubber
}

fn find_o2_generator(lines: &[String]) -> usize {
    let line = reduce_lines(lines, |lines, index| 
        find_most_common_bit(lines, index).unwrap_or(true)
    );

    parse_binary(line.as_str())
}

fn find_co2_scrubber(lines: &[String]) -> usize {
    let line = reduce_lines(lines, |lines, index| 
        match find_most_common_bit(lines, index) {
            Some(value) =>!value,
            None => false
        });

    parse_binary(line.as_str())
}

fn reduce_lines(lines: &[String], column_filter: fn(&[String], usize) -> bool) -> String {
    let mut remaining_lines: Vec<String> = lines.to_vec();

    for i in 0..lines[0].len() {
        let bit_filter = column_filter(&remaining_lines, i);
        remaining_lines = filter_lines(&remaining_lines, i, bit_filter);

        if remaining_lines.len() == 1 {
            break;
        }
    }

    remaining_lines.pop().unwrap()
}

fn filter_lines(lines: &[String], index: usize, bit: bool) -> Vec<String> {
    lines.iter().cloned().filter(|line| parse_bit(line.chars().nth(index).unwrap()) == bit).collect()
}

fn find_most_common_bit(lines: &[String], index: usize) -> Option<bool> {
    let ones = lines.iter()
        .map(|l| l.chars().nth(index).unwrap())
        .filter(|c| *c == '1')
        .count();
    let zeros = lines.len() - ones;

    match ones.cmp(&zeros) {
        std::cmp::Ordering::Greater => Some(true),
        std::cmp::Ordering::Less => Some(false),
        std::cmp::Ordering::Equal => None
    }
}

fn parse_bit(bit: char) -> bool {
    bit == '1'
}

fn format_bit(bit: bool) -> char {
    match bit {
        true => '1',
        false => '0',
    }
}

fn parse_binary(input: &str) -> usize {
    usize::from_str_radix(input, 2).unwrap()
}

fn parse_input(file: &str) -> Vec<String> {
    let input = fs::read_to_string(file).unwrap();
    input.lines().map(String::from).collect()
}