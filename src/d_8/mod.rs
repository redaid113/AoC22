use std::collections::HashSet;

fn parse_line(line: &str) -> Vec<i32> {
    return line
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();
}

fn parse_trees(contents: &str) -> Vec<Vec<i32>> {
    return contents.split("\n").map(parse_line).collect();
}

fn part1(trees: Vec<Vec<i32>>) -> u32 {
    let mut found: HashSet<(usize, usize)> = HashSet::new();

    for i in 0..trees.len() {
        let mut biggest: i32 = -1;
        for j in 0..trees.len() {
            if trees[i][j] > biggest {
                biggest = trees[i][j];
                found.insert((i, j));
            }
        }
        biggest = -1;
        for j in (0..trees.len()).rev() {
            if trees[i][j] > biggest {
                biggest = trees[i][j];
                found.insert((i, j));
            }
        }
    }

    for j in 0..trees.len() {
        let mut biggest: i32 = -1;
        for i in 0..trees.len() {
            if trees[i][j] > biggest {
                biggest = trees[i][j];
                found.insert((i, j));
            }
        }
        biggest = -1;
        for i in (0..trees.len()).rev() {
            if trees[i][j] > biggest {
                biggest = trees[i][j];
                found.insert((i, j));
            }
        }
    }

    return found.len().try_into().unwrap();
}

fn calculate_score(trees: Vec<Vec<i32>>, i: usize, j: usize) -> u32 {
    let mut top_count = 0;
    for x in (0..i).rev() {
        top_count += 1;
        if trees[x][j] >= trees[i][j] {
            break;
        }
    }

    let mut bottom_count = 0;
    for x in i + 1..trees.len() {
        bottom_count += 1;
        if trees[x][j] >= trees[i][j] {
            break;
        }
    }

    let mut left_count = 0;
    for x in (0..j).rev() {
        left_count += 1;
        if trees[i][x] >= trees[i][j] {
            break;
        }
    }

    let mut right_count = 0;
    for x in j + 1..trees.len() {
        right_count += 1;
        if trees[i][x] >= trees[i][j] {
            break;
        }
    }

    return top_count * bottom_count * left_count * right_count;
}

fn part2(trees: Vec<Vec<i32>>) -> u32 {
    let mut max = 0;
    for i in 1..trees.len() - 1 {
        for j in 1..trees.len() - 1 {
            let score = calculate_score(trees.clone(), i, j);
            if score > max {
                max = score;
            }
        }
    }
    return max;
}

pub fn run(contents: &str) {
    let trees = parse_trees(contents);
    println!("Part 1: {}", part1(trees.clone()));
    println!("Part 2: {}", part2(trees.clone()));
}
