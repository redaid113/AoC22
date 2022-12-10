fn parse_line(line: &str) -> (&str, i32) {
    let bits: Vec<&str> = line.split(" ").collect();
    if bits[0] == "noop" {
        return ("noop", 0);
    }
    return (bits[0], bits[1].parse().unwrap());
}

fn parse_instructions(contents: &str) -> Vec<(&str, i32)> {
    return contents.split('\n').map(parse_line).collect();
}

fn add_noops(instructions: Vec<(&str, i32)>) -> Vec<(&str, i32)> {
    return instructions
        .into_iter()
        .map(|instruction| {
            if instruction.0 == "addx" {
                return [("noop", 0), instruction].to_vec();
            }
            return [instruction].to_vec();
        })
        .flatten()
        .collect();
}

fn load_instructions(contents: &str) -> Vec<(&str, i32)> {
    let instructions = parse_instructions(contents);
    return add_noops(instructions);
}

fn part1(contents: &str) -> i32 {
    let instructions = load_instructions(contents);
    let mut x: i32 = 1;
    let mut sum = 0;
    for i in 0..instructions.len() {
        if (i + 1 + 20) % 40 == 0 {
            sum += x * (i + 1) as i32;
        }
        if instructions[i].0 == "addx" {
            x += instructions[i].1
        }
    }
    return sum;
}

fn print(line: Vec<&str>) {
    for i in 0..line.len() {
        if i % 40 == 0 {
            println!();
        }
        print!("{}", line[i]);
    }
    println!();
}

fn part2(contents: &str) {
    let instructions = load_instructions(contents);
    let mut x: i32 = 1;
    let mut line: Vec<&str> = [].to_vec();
    for i in 0..instructions.len() {
        if [x - 1, x, x + 1].contains(&((i % 40) as i32)) {
            line.push("#");
        } else {
            line.push(".");
        }
        if instructions[i].0 == "addx" {
            x += instructions[i].1
        }
    }
    print(line);
}

pub fn run(contents: &str) {
    println!("Part 1: {}", part1(contents.clone()));
    println!("Part 2: ");
    part2(contents.clone());
}
