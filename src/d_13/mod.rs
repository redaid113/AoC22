use serde::Deserialize;
use std::cmp::Ordering;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
enum Value {
    Number(u32),
    Array(Vec<Value>),
}

impl PartialOrd for Value {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl Ord for Value {
    fn cmp(&self, rhs: &Self) -> Ordering {
        match (self, rhs) {
            (Value::Number(l), Value::Number(r)) => l.cmp(r),
            (Value::Array(l), Value::Array(r)) => l.cmp(r),
            (Value::Number(l), Value::Array(r)) => vec![Value::Number(*l)].cmp(r),
            (Value::Array(l), Value::Number(r)) => l.cmp(&vec![Value::Number(*r)]),
        }
    }
}

fn parse_value(line: &str) -> Value {
    let temp: Value = serde_json::from_str(line).unwrap();
    return temp;
}

fn parse_values(line: &str) -> (Value, Value) {
    let bits: Vec<&str> = line.split("\n").collect();
    return (parse_value(bits[0]), parse_value(bits[1]));
}

fn parse_contents(contents: &str) -> Vec<(Value, Value)> {
    return contents.split("\n\n").map(parse_values).collect();
}

fn part1(contents: &str) -> i32 {
    let lists: Vec<(Value, Value)> = parse_contents(contents);
    return lists
        .iter()
        .enumerate()
        .map(|(i, values)| {
            if values.0.cmp(&values.1) == Ordering::Less {
                return i as i32 + 1;
            }
            return 0;
        })
        .sum();
}

fn parse_list_of_values(contents: &str) -> Vec<Value> {
    return contents
        .replace("\n\n", "\n")
        .split("\n")
        .map(parse_value)
        .collect();
}

fn part2(contents: &str) -> usize {
    let mut lists: Vec<Value> = parse_list_of_values(contents);

    let two: Value = serde_json::from_str("[[2]]").unwrap();
    let six: Value = serde_json::from_str("[[6]]").unwrap();

    lists.push(two.clone());
    lists.push(six.clone());

    lists.sort();

    let two_index = lists.iter().position(|value| value.eq(&two)).unwrap() + 1;
    let six_index = lists.iter().position(|value| value.eq(&six)).unwrap() + 1;

    return two_index * six_index;
}

pub fn run(contents: &str) {
    println!("Part 1: {}", part1(contents));
    println!("Part 2: {}", part2(contents));
}
