#[derive(Debug)]
enum CommandType {
    FORWARD,
    UP,
    DOWN
}

#[derive(Debug)]
struct Command {
    pub command: CommandType,
    pub unit: i32
}

impl Command {
    pub fn from_str(str: &str) -> Command {
        let command_type = match str.split_whitespace().nth(0).unwrap() {
            "forward" => CommandType::FORWARD,
            "down" => CommandType::DOWN,
            "up" => CommandType::UP,
            _ => panic!("Input error!")
        };

        Command {
            command: command_type,
            unit: str.split_whitespace()
                        .nth(1)
                        .unwrap()
                        .parse::<i32>()
                        .unwrap()
        }
    }
}

fn solve_part_one(input_list: &Vec<Command>) -> i32 {
    let (mut x, mut d) = (0, 0);
    for input in input_list.iter() {
        match input.command {
            CommandType::FORWARD => x += input.unit,
            CommandType::DOWN => d += input.unit,
            CommandType::UP => d -= input.unit
        }
    }
    x * d
}

fn solve_part_two(input_list: &Vec<Command>) -> i32 {           
    let (mut x, mut d, mut aim) = (0, 0, 0);
    for input in input_list.iter() {
        match input.command {
            CommandType::FORWARD => { 
                x += input.unit;
                d += aim * input.unit
            },
            CommandType::DOWN => aim += input.unit,
            CommandType::UP => aim -= input.unit
        }
    }
    x * d
}

fn main() {
    let input = include_str!("../input.txt");
    let input_list: Vec<Command> = input
        .lines()
        .map(|f| Command::from_str(f))
        .collect();
    println!("Day2 Part1: {}", solve_part_one(input_list.as_ref()));
    println!("Day2 Part2: {}", solve_part_two(input_list.as_ref()));
}
