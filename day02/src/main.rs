use std::fs;

struct Command {
    command: String,
    value: i32,
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let commands = parse_input(&input);
    let part1 = part1(&commands);
    let part2 = part2(&commands);

    println!("Part 1: {} Part 2: {}", part1, part2);
}

fn parse_input(input: &str) -> Vec<Command> {
    input.lines().map(parse_input_line).collect()
}

fn parse_input_line(line: &str) -> Command {
    let mut parts = line.split(' ');
    let command = String::from(parts.next().unwrap());
    let value = parts.next().unwrap().parse().unwrap();

    Command { command, value }
}

fn part1(commands: &[Command]) -> i32 {
    let mut x = 0;
    let mut depth = 0;

    for command in commands.iter() {
        match command.command.as_str() {
            "forward" => x += command.value,
            "down" => depth += command.value,
            "up" => depth -= command.value,
            _ => panic!()
        }
    }

    x * depth
}

fn part2(commands: &[Command]) -> i32 {
    let mut x = 0;
    let mut depth = 0;
    let mut aim = 0;

    for command in commands.iter() {
        match command.command.as_str() {
            "forward" =>  {
                x += command.value;
                depth += aim * command.value;
            },
            "down" => aim += command.value,
            "up" => aim -= command.value,
            _ => panic!()
        }
    }

    x * depth
}