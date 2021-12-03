#[derive(Debug)]
enum CommandType {
    FORWARD,
    UP,
    DOWN
}

#[derive(Debug)]
struct Command {
    command: CommandType,
    unit: i32
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

struct Submarine {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

impl Submarine {
    pub fn new(h: i32, d: i32, a: i32) -> Submarine {
        Submarine {
            horizontal: h,
            depth: d,
            aim: a
        }
    }

    pub fn reset(&mut self) {
        self.horizontal = 0;
        self.depth = 0;
        self.aim = 0;
    }

    pub fn solve_part_one(&mut self, input_list: &Vec<Command>) -> i32 {
        for input in input_list.iter() {
            match input.command {
                CommandType::FORWARD => self.horizontal += input.unit,
                CommandType::DOWN => self.depth += input.unit,
                CommandType::UP => self.depth -= input.unit
            }
        }
        self.horizontal * self.depth
    }

    pub fn solve_part_two(&mut self, input_list: &Vec<Command>) -> i32 {           
        for input in input_list.iter() {
            match input.command {
                CommandType::FORWARD => { 
                    self.horizontal += input.unit;
                    self.depth += self.aim * input.unit
                },
                CommandType::DOWN => self.aim += input.unit,
                CommandType::UP => self.aim -= input.unit
            }
        }
        self.horizontal * self.depth
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let input_list: Vec<Command> = input
        .lines()
        .map(|f| Command::from_str(f))
        .collect();

    let mut submarine = Submarine::new(0, 0, 0);
    println!("Day2 Part1: {}", submarine.solve_part_one(input_list.as_ref()));
    submarine.reset();
    println!("Day2 Part2: {}", submarine.solve_part_two(input_list.as_ref()));
}
