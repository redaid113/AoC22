use std::collections::HashSet;

fn value(c: &char) -> i32 {
    let mut val = *c as i32 - 'a' as i32 + 1;
    if val <= 0 {
        val = *c as i32 - 'A' as i32 + 27;
    }

    return val;
}

fn part1(line: &str) -> i32 {
    let mut found: HashSet<char> = HashSet::new();

    let first_half = &line[0..line.len() / 2];
    let second_half = &line[line.len() / 2..];

    first_half.chars().for_each(|c| {
        found.insert(c);
    });

    for c in second_half.chars() {
        if found.contains(&c) {
            return value(&c);
        }
    }
    return 0;
}

fn part2(line1: &str, line2: &str, line3: &str) -> i32 {
    let mut found1: HashSet<char> = HashSet::new();
    let mut found2: HashSet<char> = HashSet::new();

    line1.chars().for_each(|c| {
        found1.insert(c);
    });

    line2.chars().for_each(|c| {
        if found1.contains(&c) {
            found2.insert(c);
        }
    });

    for c in line3.chars() {
        if found2.contains(&c) {
            return value(&c);
        }
    }
    return 0;
}

pub fn run(contents: &str) {
    let arr: Vec<&str> = contents.trim().split("\n").collect();
    let arr2 = arr.clone();

    let sum1: i32 = arr.into_iter().map(|line| part1(line)).sum();

    let mut sum2 = 0;
    for i in 0..arr2.len() {
        if i % 3 != 0 {
            continue;
        }

        sum2 += part2(arr2[i], arr2[i + 1], arr2[i + 2])
    }

    println!("Part 1: {}", sum1);
    println!("Part 2: {}", sum2);
}
