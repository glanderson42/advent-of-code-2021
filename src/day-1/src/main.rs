fn solve_part_one(input_list: &Vec<i32>) -> usize {
    input_list
        .windows(2)
        .filter(|f| f[0] < f[1])
        .count()
}

fn solve_part_two(input: &Vec<i32>) -> usize {
    let groups: Vec<i32> = input
                            .windows(3)
                            .map(|w| w.iter().sum())
                            .collect();

    groups
        .windows(2)
        .filter(|f| f[0] < f[1])
        .count()
}

fn main() {
    let input = include_str!("../input.txt");
    let input_list: Vec<i32> = input
        .lines()
        .map(|f| f.parse::<i32>().unwrap())
        .collect();

    
    println!("Day1 Part1 Answer: {}", solve_part_one(input_list.as_ref()));
    println!("Day1 Part2 Answer: {}", solve_part_two(input_list.as_ref()));
}