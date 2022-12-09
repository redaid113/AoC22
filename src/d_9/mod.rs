use std::collections::HashSet;

fn parse_instruction(line: &str) -> (&str, i32) {
    let bits: Vec<&str> = line.split(" ").collect();
    return (bits[0], bits[1].parse().unwrap());
}

fn parse_instructions(contents: &str) -> Vec<(&str, i32)> {
    return contents.split("\n").map(parse_instruction).collect();
}

fn move_head(dir: &str, head: (i32, i32)) -> (i32, i32) {
    match dir {
        "U" => {
            return (head.0, head.1 + 1);
        }
        "D" => {
            return (head.0, head.1 - 1);
        }
        "L" => {
            return (head.0 - 1, head.1);
        }
        "R" => {
            return (head.0 + 1, head.1);
        }
        _ => panic!("NOOO"),
    }
}
fn move_piece(piece: (i32, i32), move_x: i32, y_diff: i32) -> (i32, i32) {
    return (piece.0 + move_x, piece.1 + y_diff);
}

fn move_rope(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    let x_diff = a.0 - b.0;
    let y_diff = a.1 - b.1;
    if x_diff.abs() == 2 || y_diff.abs() == 2 {
        let move_x = if x_diff == 0 {
            0
        } else {
            (x_diff + x_diff / x_diff.abs()) / 2
        };
        let move_y = if y_diff == 0 {
            0
        } else {
            (y_diff + y_diff / y_diff.abs()) / 2
        };

        return move_piece(b, move_x, move_y);
    }
    return b;
}

fn calculated_visted<const ROPE_LENGTH: usize>(instructions: Vec<(&str, i32)>) -> usize {
    let mut rope: [(i32, i32); ROPE_LENGTH] = [(0, 0); ROPE_LENGTH];
    let head = 0;
    let tail = ROPE_LENGTH - 1;

    let mut visited = HashSet::<(i32, i32)>::new();
    visited.insert(rope[tail]);
    instructions.into_iter().for_each(|(dir, steps)| {
        for _ in 0..steps {
            rope[head] = move_head(dir, rope[head]);
            for i in 0..ROPE_LENGTH - 1 {
                rope[i + 1] = move_rope(rope[i], rope[i + 1]);
            }
            visited.insert(rope[tail]);
        }
    });

    return visited.len();
}

fn part1(instructions: Vec<(&str, i32)>) -> usize {
    return calculated_visted::<2>(instructions);
}

fn part2(instructions: Vec<(&str, i32)>) -> usize {
    return calculated_visted::<10>(instructions);
}

pub fn run(contents: &str) {
    let instructions = parse_instructions(contents);
    println!("Part 1: {}", part1(instructions.clone()));
    println!("Part 2: {}", part2(instructions.clone()));
}
