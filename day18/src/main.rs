use core::num;
use std::fs;
use itertools::Itertools;

#[derive(Debug, Clone)]
enum Node {
    Number {value: i32},
    Pair {left: Box<Node>, right: Box<Node>}
}

impl Node {
    fn format(&self) -> String {
        match self {
            Node::Number {value} => String::from(value.to_string()),
            Node::Pair {left, right} => format!("[{},{}]", left.format(), right.format())
        }
    }

    fn magnitude(&self) -> usize {
        match self {
            Node::Number {value} => *value as usize,
            Node::Pair {left, right} => (left.magnitude() * 3) + (right.magnitude() * 2),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Token {
    BracketLeft,
    BracketRight,
    Comma,
    Number {value: i32},
}

impl Token {
    fn format(&self) -> String {
        match self {
            Token::BracketLeft => String::from("["),
            Token::BracketRight => String::from("]"),
            Token::Comma => String::from(","),
            Token::Number {value} => value.to_string(),
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let values = parse_input(&input);

    let part1 = part1(&values);
    let part2 = part2(&values);

    println!("Part 1: {} Part 2: {}", part1, part2);
}

fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(String::from).collect()
}

fn part1(lines: &[String]) -> usize {
    let mut numbers = lines.iter().map(|line| parse_tokens(line)).collect_vec();
    for n in numbers.iter() {
        eprintln!("{:?}", format_tokens(n));
    }
    println!();


    numbers.reverse();
    let mut added = numbers.pop().unwrap();

    while !numbers.is_empty() {
        let to_add = numbers.pop().unwrap();

        let l = added.iter().copied().collect_vec();
        let r = to_add.iter().copied().collect_vec();

        added = add_tokens(&added, &to_add);
        added = reduce(added);

        println!("  {}", format_tokens(&l));
        println!("+ {}", format_tokens(&r));
        println!("= {}", format_tokens(&added));
        println!();
    }
    eprintln!(" = {:?}", format_tokens(&added));
    parse(&format_tokens(&added)).magnitude()
}

fn part2(lines: &[String]) -> usize {
    let mut numbers = lines.iter().map(|line| parse_tokens(line)).collect_vec();

    let mut largest_magnitude = 0;

    for left_number in numbers.iter().enumerate() {
        for right_number in numbers.iter().enumerate() {
            if left_number.0 == right_number.0 {
                continue;
            }

            let result = reduce(add_tokens(&left_number.1, &right_number.1));
            let magnitude = parse(&format_tokens(&result)).magnitude();

            largest_magnitude = largest_magnitude.max(magnitude);
        }    
    }

    largest_magnitude
}

fn add(left: Node, right: Node) -> Node {
    Node::Pair {left: Box::new(left), right: Box::new(right)}
}

fn reduce(token: Vec<Token>) -> Vec<Token> {
    let mut tokens = token;
    loop {
        let (new_tokens, exploded) = explode(tokens);
        tokens = new_tokens;

        if exploded {
            continue;
        }

        let (new_tokens, split) = split(tokens);
        tokens = new_tokens;

        if !split {
            break;
        }
    }
    
    tokens
}

fn get_value(token: &Token) -> i32{
    match token {
        Token::Number {value} => *value,
        _ => panic!()
    }
}

fn explode(tokens: Vec<Token>) -> (Vec<Token>, bool) {
    //eprintln!("{}", format_tokens(&tokens));
    let mut result = tokens.clone();

    let mut depth = 0;

    for token in tokens.iter().enumerate() {
        //eprintln!("token = {:?} (depth: {})", token, depth);
        match token.1 {
            Token::BracketLeft =>  {
                if depth == 4 {
                    let left = get_value(&tokens[token.0 + 1]);
                    let right = get_value(&tokens[token.0 + 3]);

                    //eprintln!("left = {:?}", left);
                    //eprintln!("right = {:?}", right);

                    for i in (0..token.0).rev() {
                        if let Token::Number {value} = tokens[i] {
                            result[i] = Token::Number {value: value + left};
                            break;
                        }
                    }

                    for i in token.0+4..tokens.len() {
                        if let Token::Number {value} = tokens[i] {
                            result[i] = Token::Number {value: value + right};
                            break;
                        }
                    }

                    result.remove(token.0); // [
                    result.remove(token.0); // left
                    result.remove(token.0); // ,
                    result.remove(token.0); // right
                    result.remove(token.0); // ]

                    result.insert(token.0, Token::Number {value: 0});

                    return (result, true);
                }

                depth += 1;
            },
            Token::BracketRight => depth -= 1,
            _ => {}
        }
    }

    return (tokens, false);
}

fn split(tokens: Vec<Token>) -> (Vec<Token>, bool) {
    //eprintln!("{}", format_tokens(&tokens));
    let mut result = tokens.clone();

    for token in tokens.iter().enumerate() {
        if let Token::Number {value} = token.1 {
            if *value >= 10 {
                result.remove(token.0);
                result.insert(token.0, Token::BracketLeft);
                result.insert(token.0 + 1, Token::Number {value: (*value as f32 / 2.0).floor() as i32});
                result.insert(token.0 + 2, Token::Comma);
                result.insert(token.0 + 3, Token::Number {value: (*value as f32 / 2.0).ceil() as i32});
                result.insert(token.0 + 4, Token::BracketRight);

                return (result, true);
            }
        }
    }

    (result, false)
}

fn add_tokens(left: &[Token], right: &[Token]) -> Vec<Token> {
    let mut result = Vec::new();

    result.push(Token::BracketLeft);
    for token in left.iter() {
        result.push(*token);
    }
    result.push(Token::Comma);
    for token in right.iter() {
        result.push(*token);
    }
    result.push(Token::BracketRight);

    result
    //left.iter().copied().chain(right.iter().copied()).collect()
}

fn parse_tokens(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut number = String::new();

    for chr in input.chars() {
        if chr.is_digit(10) {
            number.push_str(&chr.to_string());
        } else if !number.is_empty() {
            tokens.push(Token::Number { value: number.parse().unwrap()});
            number = String::new();
        }

        if chr == '[' {
            tokens.push(Token::BracketLeft);
        } else if chr == ']' {
            tokens.push(Token::BracketRight);
        } else if chr == ',' {
            tokens.push(Token::Comma);
        }
    }

    tokens
}

fn format_tokens(tokens: &[Token]) -> String {
    tokens.iter().map(|token| token.format()).collect()
}

fn parse(input: &str) -> Node {
    if input.starts_with("[") {
        let input = &input[1..input.len()-1];
        let mut depth = 0;

        for chr in input.chars().enumerate() {
            if chr.1 == '[' {
                depth += 1;
            } else if chr.1 == ']' {
                depth -= 1;
            } else if chr.1 == ',' && depth == 0 {
                let left = &input[..chr.0];
                let right = &input[chr.0+1..];

                return Node::Pair {
                    left: Box::new(parse(left)),
                    right: Box::new(parse(right)),
                };
            }
        }

        panic!();
    } else {
        Node::Number {value: input.parse().unwrap()}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_tokens_should_work() {
        let tokens = parse_tokens("[1,20]");
        let result = format_tokens(&tokens);

        assert_eq!("[1,20]", result);
    }

    #[test]
    fn parse_should_parse_single_pair() {
        let result = parse("[1,2]");

        assert_eq!("[1,2]", result.format());
    }

    #[test]
    fn parse_should_parse_nested_pair() {
        let result = parse("[[1,2],3]");

        assert_eq!("[[1,2],3]", result.format());
    }

    #[test]
    fn add_should_add_numbers() {
        let n1 = Node::Number{value: 1};
        let n2 = Node::Number{value: 2};
        let result = add(n1, n2);

        assert_eq!("[1,2]", result.format());
    }

    #[test]
    fn add_should_add_pairs() {
        let n1 = Node::Pair{left: Box::new(Node::Number{value: 1}), right: Box::new(Node::Number{value: 2})};
        let n2 = Node::Number{value: 3};
        let result = add(n1, n2);

        assert_eq!("[[1,2],3]", result.format());
    }

    #[test]
    fn explode_should_version1() {
        let tokens = parse_tokens("[[6,[5,[4,[3,2]]]],1]");
        let result = explode(tokens).0;

        assert_eq!("[[6,[5,[7,0]]],3]", format_tokens(&result));
    }

    #[test]
    fn explode_should_version2() {
        let tokens = parse_tokens("[[[[[9,8],1],2],3],4]");
        let result = explode(tokens).0;

        assert_eq!("[[[[0,9],2],3],4]", format_tokens(&result));
    }

    #[test]
    fn explode_should_version3() {
        let tokens = parse_tokens("[7,[6,[5,[4,[3,2]]]]]");
        let result = explode(tokens).0;

        assert_eq!("[7,[6,[5,[7,0]]]]", format_tokens(&result));
    }

    #[test]
    fn split_should_work() {
        let tokens = parse_tokens("[11,1]");
        let result = split(tokens).0;

        assert_eq!("[[5,6],1]", format_tokens(&result));
    }

    #[test]
    fn reduce_should_work() {
        let tokens = parse_tokens("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
        let result = reduce(tokens);

        assert_eq!("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", format_tokens(&result));
    }

    #[test]
    fn test() {
        let left = parse_tokens("[[[[4,3],4],4],[7,[[8,4],9]]]");
        let right = parse_tokens("[1,1]");
        let result = add_tokens(&left, &right);
        eprintln!("result = {:?}", format_tokens(&result));
        let result = reduce(result);
        eprintln!("result = {:?}", result);

        assert_eq!("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", format_tokens(&result));
    }

    #[test]
    fn magnitude() {
        let token = parse("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");
        assert_eq!(3488, token.magnitude());
    }
    // #[test]
    // fn explode_should_work_with_single_pair() {
    //     let n1 = parse("[[6,[5,[4,[3,2]]]],1]");
    //     let result = explode(n1);

    //     assert_eq!("[[6,[5,[7,0]]],3]", result.format());
    // }

    #[test]
    fn part1_should_work() {
        let input = fs::read_to_string("input.txt").unwrap();
        let input = parse_input(&input);
        let result = part1(&input);

        assert_eq!(3981, result);
    }

    #[test]
    fn part2_should_work() {
        let input = fs::read_to_string("input2.txt").unwrap();
        let input = parse_input(&input);
        let result = part2(&input);

        assert_eq!(3993, result);
    }

    // #[test]
    // fn part2_should_work() {
    //     let input = vec![String::from("123")];
    //     let result = part2(&input);

    //     assert_eq!(2, result);
    // }
}