use std::collections::VecDeque;

enum Operator {
    TIMES,
    ADD,
    POWER,
}

struct Monkey {
    pub count: u64,
    pub worries: VecDeque<u64>,
    operator: Operator,
    operator_value: u64,
    test: u64,
    truthy: usize,
    falsey: usize,
}

fn parse_worries(line: &str) -> VecDeque<u64> {
    line.trim()
        .replace("Starting items: ", "")
        .split(", ")
        .map(|num| num.parse().unwrap())
        .collect()
}

fn get_operator(bit: &str) -> Operator {
    match bit {
        "*" => return Operator::TIMES,
        "+" => return Operator::ADD,
        "^" => return Operator::POWER,

        _ => panic!("NOoo"),
    }
}

fn parse_operation(line: &str) -> (Operator, u64) {
    let bits: Vec<&str> = line.split(" ").collect();
    let mut operator_str = bits[bits.len() - 2];
    let mut value = bits[bits.len() - 1];
    if value == "old" {
        operator_str = "^";
        value = "2";
    }

    let operator = get_operator(operator_str);
    (operator, value.parse().unwrap())
}

fn parse_last_number(line: &str) -> u64 {
    return line.trim().split(" ").last().unwrap().parse().unwrap();
}

impl Monkey {
    pub fn new(monkey_string: &str) -> Self {
        let lines: Vec<&str> = monkey_string.split("\n").collect();

        let worries: VecDeque<u64> = parse_worries(lines[1]);
        let operation = parse_operation(lines[2]);
        let test = parse_last_number(lines[3]);
        let truthy = parse_last_number(lines[4]) as usize;
        let falsey = parse_last_number(lines[5]) as usize;

        Monkey {
            count: 0,
            worries,
            operator: operation.0,
            operator_value: operation.1,
            test,
            truthy,
            falsey,
        }
    }

    pub fn inspect_worry(&mut self) -> u64 {
        self.count += 1;
        let mut worry = self.worries.pop_front().unwrap();
        match self.operator {
            Operator::ADD => worry += self.operator_value,
            Operator::TIMES => worry *= self.operator_value,
            Operator::POWER => worry = worry * worry,
        }

        return worry;
    }

    pub fn throw_to(&mut self, worry: u64) -> usize {
        if worry % self.test == 0 {
            return self.truthy;
        }
        return self.falsey;
    }

    pub fn catch(&mut self, worry: u64) {
        self.worries.push_back(worry);
    }
}

fn parse_monkey(monkey_string: &str) -> Box<Monkey> {
    let monkey: Monkey = Monkey::new(monkey_string);
    return Box::new(monkey);
}

fn parse_monkeys(contents: &str) -> Vec<Box<Monkey>> {
    return contents
        .split("\n\n")
        .map(|line| parse_monkey(line))
        .collect();
}

fn part1(contents: &str) -> u64 {
    let mut monkeys: Vec<Box<Monkey>> = parse_monkeys(contents);

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while monkeys[i].worries.len() > 0 {
                let monkey = &mut monkeys[i];
                let worry = monkey.inspect_worry() / 3;
                let throw_to = monkey.throw_to(worry);
                monkeys[throw_to].catch(worry);
            }
        }
    }
    let mut counts: Vec<u64> = monkeys.iter().map(|monkey| monkey.count).collect();
    counts.sort();
    counts.reverse();

    return counts[0] * counts[1];
}

fn part2(contents: &str) -> u64 {
    let mut monkeys: Vec<Box<Monkey>> = parse_monkeys(contents);
    let reduce_level: u64 = monkeys.iter().map(|monkey| monkey.test).product();

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            while monkeys[i].worries.len() > 0 {
                let monkey = &mut monkeys[i];
                let worry = monkey.inspect_worry() % reduce_level;
                let throw_to = monkey.throw_to(worry);
                monkeys[throw_to].catch(worry);
            }
        }
    }
    let mut counts: Vec<u64> = monkeys.iter().map(|monkey| monkey.count).collect();
    counts.sort();
    counts.reverse();

    return counts[0] * counts[1];
}

pub fn run(contents: &str) {
    println!("Part 1: {}", part1(contents.clone()));
    println!("Part 2: {}", part2(contents.clone()));
}
