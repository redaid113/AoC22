use std::collections::HashMap;

static HUMAN: &str = "humn";
static ROOT: &str = "root";

#[derive(Debug, Clone)]

enum Operation {
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Debug, Clone)]
struct Monkey {
    id: String,
    value: Option<i64>,
    operation: Option<Operation>,
    monkey1: Option<String>,
    monkey2: Option<String>,
}

fn parse_operation(o: &str) -> Option<Operation> {
    match o {
        "+" => Some(Operation::Plus),
        "-" => Some(Operation::Minus),
        "*" => Some(Operation::Multiply),
        "/" => Some(Operation::Divide),
        _ => None,
    }
}

fn parse_monkey(line: &str) -> Monkey {
    let bits: Vec<&str> = line.split(' ').collect();

    let id = bits[0].replace(":", "").to_string();
    if bits.len() == 2 {
        return Monkey {
            id,
            value: Some(bits[1].parse().unwrap()),
            operation: None,
            monkey1: None,
            monkey2: None,
        };
    }

    return Monkey {
        id,
        value: None,
        operation: parse_operation(bits[2]),
        monkey1: Some(bits[1].to_string()),
        monkey2: Some(bits[3].to_string()),
    };
}

fn parse_input(contents: &str) -> HashMap<String, Monkey> {
    let list: Vec<Monkey> = contents.split('\n').map(parse_monkey).collect();
    let mut monkeys: HashMap<String, Monkey> = HashMap::new();
    for monkey in list.iter() {
        monkeys.insert(monkey.id.clone(), monkey.clone());
    }
    monkeys
}

fn math(operation: Operation, a: i64, b: i64) -> i64 {
    match operation {
        Operation::Plus => a + b,
        Operation::Minus => a - b,
        Operation::Multiply => a * b,
        Operation::Divide => a / b,
    }
}

fn silly_monkey(monkeys: &mut HashMap<String, Monkey>, id: String) -> i64 {
    let monkey = monkeys.get(&id).unwrap().clone();
    if monkey.value.is_some() {
        return monkey.value.unwrap();
    }

    let a = silly_monkey(monkeys, monkey.monkey1.clone().unwrap());
    let b = silly_monkey(monkeys, monkey.monkey2.clone().unwrap());
    let value = math(monkey.operation.clone().unwrap(), a, b);

    monkeys.insert(
        id.clone(),
        Monkey {
            id,
            value: Some(value),
            operation: None,
            monkey1: None,
            monkey2: None,
        },
    );
    return value;
}

fn part1(contents: &str) -> i64 {
    let mut monkeys = parse_input(contents);

    silly_monkey(&mut monkeys, ROOT.to_string())
}

fn no_more_monkeys_jumping_on_the_bed(
    monkeys: &mut HashMap<String, Monkey>,
    id: String,
) -> (i64, bool) {
    let monkey = monkeys.get(&id).unwrap().clone();
    if monkey.value.is_some() {
        return (monkey.value.unwrap(), id == HUMAN.to_string());
    }

    let (a, ah) = no_more_monkeys_jumping_on_the_bed(monkeys, monkey.monkey1.clone().unwrap());
    let (b, bh) = no_more_monkeys_jumping_on_the_bed(monkeys, monkey.monkey2.clone().unwrap());
    let value = math(monkey.operation.clone().unwrap(), a, b);

    let seen_human = ah || bh;
    if !seen_human {
        monkeys.insert(
            id.clone(),
            Monkey {
                id,
                value: Some(value),
                operation: None,
                monkey1: None,
                monkey2: None,
            },
        );
    }

    return (value, seen_human);
}

fn part2(contents: &str) -> i64 {
    let mut monkeys = parse_input(contents);
    let root = monkeys.get(&ROOT.to_string()).clone().unwrap();

    let m1 = root.monkey1.clone().unwrap();
    let m2 = root.monkey2.clone().unwrap();

    let mut i = 1;
    let mut step = 10000000000;

    let mut ct = 0;
    while ct < 1000000 {
        ct += 1;
        monkeys.insert(
            HUMAN.to_string(),
            Monkey {
                id: HUMAN.to_string(),
                value: Some(i),
                operation: None,
                monkey1: None,
                monkey2: None,
            },
        );
        let a = no_more_monkeys_jumping_on_the_bed(&mut monkeys, m1.clone()).0;
        let b = no_more_monkeys_jumping_on_the_bed(&mut monkeys, m2.clone()).0;
        if a == b && step == 1 {
            return i;
        }

        if a > b {
            i += step
        } else {
            step /= 2;
            i -= step;
        }
    }

    0
}

pub fn run(contents: &str) {
    println!("Part 1: {}", part1(contents));
    println!("Part 2: {}", part2(contents));
}
