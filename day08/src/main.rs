use std::{fs, collections::HashSet};
use itertools::Itertools;

struct Entry {
    signals: Vec<String>,
    output: Vec<String>
}

fn main() {
    let input = fs::read_to_string("input2.txt").unwrap();
    let values = parse_input(&input);

    let part1 = part1(&values);
    let part2 = part2(&values);

    println!("Part 1: {} Part 2: {}", part1, part2);
}

fn parse_input(input: &str) -> Vec<Entry> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Entry {
    let parts = line.split('|').collect_vec();

    Entry { 
        signals: parts[0].trim().split(' ').map(|v| String::from(v)).collect_vec(), 
        output: parts[1].trim().split(' ').map(|v| String::from(v)).collect_vec(), 
    }
}

fn part1(entries: &[Entry]) -> usize {
    let x= entries.iter().map(|entry| 
        let lengths: HashSet<usize> = entry.output.iter().map(|v| v.len()).collect();

        entry.output.iter().filter(|output| vec![2, 4, 3, 7].contains(&(output.len() as i32))).count()
    ); //.sum()
    x.sum()
    //1
}

fn part2(values: &[Entry]) -> i32 {
    2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_should_work() {
        let input = vec![String::from("123")];
        let result = part1(&input);

        assert_eq!(1, result);
    }

    #[test]
    fn part2_should_work() {
        let input = vec![String::from("123")];
        let result = part2(&input);

        assert_eq!(2, result);
    }
}