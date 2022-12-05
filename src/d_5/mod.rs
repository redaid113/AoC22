fn crane_count(line: &str) -> i32 {
    return line.split(' ').last().unwrap().parse().expect("number");
}

fn parse_crates(input: &str) -> Vec<Vec<char>> {
    let mut list: Vec<&str> = input.split('\n').collect();

    let total_cranes = crane_count(list.pop().unwrap());
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _ in 0..total_cranes {
        stacks.push(Vec::new())
    }

    while list.len() > 0 {
        let line = list.pop().unwrap();
        let cur: Vec<char> = line.chars().collect();
        for i in 0..total_cranes {
            let index: usize = ((i * 4) + 1) as usize;
            if index >= (cur.len()) {
                break;
            }

            let c = cur[index];
            if c != ' ' {
                stacks[i as usize].push(c);
            }
        }
    }
    return stacks;
}

fn parse_instruction_line(line: &str) -> (i32, i32, i32) {
    let nums = line.split(' ').collect::<Vec<&str>>();

    let out = (
        nums[1].parse().expect("Not a number"),
        nums[3].parse().expect("Not a number"),
        nums[5].parse().expect("Not a number"),
    );

    return out;
}
fn parse_instructions(input: &str) -> Vec<(i32, i32, i32)> {
    return input
        .trim()
        .split("\n")
        .into_iter()
        .map(|line| parse_instruction_line(line))
        .collect();
}
fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<(i32, i32, i32)>) {
    let parts: Vec<&str> = input.trim_end().split("\n\n").collect();

    let crates: Vec<Vec<char>> = parse_crates(parts[0]);
    let instructions: Vec<(i32, i32, i32)> = parse_instructions(parts[1]);

    return (crates, instructions);
}

fn part1(crates: &mut Vec<Vec<char>>, instructions: Vec<(i32, i32, i32)>) -> String {
    instructions.into_iter().for_each(|(count, from, to)| {
        for _ in 0..count {
            let c: char = crates[from as usize - 1].pop().unwrap();
            crates[to as usize - 1].push(c);
        }
    });
    let firsts: Vec<char> = crates.into_iter().map(|vec| *vec.last().unwrap()).collect();
    return firsts.iter().collect::<String>();
}

fn part2(crates: &mut Vec<Vec<char>>, instructions: Vec<(i32, i32, i32)>) -> String {
    instructions.into_iter().for_each(|(count, from, to)| {
        let mut temp: Vec<char> = Vec::new();
        for _ in 0..count {
            let c: char = crates[from as usize - 1].pop().unwrap();
            temp.push(c);
        }
        for _ in 0..count {
            let c: char = temp.pop().unwrap();
            crates[to as usize - 1].push(c);
        }
    });
    let firsts: Vec<char> = crates.into_iter().map(|vec| *vec.last().unwrap()).collect();
    return firsts.iter().collect::<String>();
}

pub fn run(contents: &str) {
    let (crates, instructions) = parse_input(contents);

    println!(
        "Part 1: {}",
        part1(&mut crates.clone(), instructions.clone())
    );
    println!(
        "Part 2: {}",
        part2(&mut crates.clone(), instructions.clone())
    );
}
