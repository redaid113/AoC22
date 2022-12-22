type Point = (usize, usize, usize);
type Next = [Point; 4];

#[derive(Debug, Clone)]
struct Element {
    value: String,
    next: Next,
}

type Map = Vec<Vec<Element>>;

#[derive(Debug, Clone)]
enum Instruction {
    Move(i32),
    Turn(String),
}

type Instructions = Vec<Instruction>;

fn parse_element(c: &str) -> Element {
    return Element {
        value: c.to_string(),
        next: [(0, 0, 0), (0, 0, 0), (0, 0, 0), (0, 0, 0)],
    };
}

fn parse_row(line: &str) -> Vec<Element> {
    line.split("")
        .filter(|c| c != &"")
        .map(parse_element)
        .collect()
}

fn parse_map(contents: &str) -> Map {
    let mut map: Map = contents.split('\n').map(parse_row).collect();

    let mut square = map.len();
    for y in 0..map.len() {
        square = square.max(map[y].len());
    }

    for _ in map.len()..square {
        map.push(vec![]);
    }

    for y in 0..map.len() {
        for _ in map[y].len()..square {
            map[y].push(Element {
                value: " ".to_string(),
                next: [(0, 0, 0), (0, 0, 0), (0, 0, 0), (0, 0, 0)],
            })
        }
    }

    map
}

fn parse_instructions(line: &str) -> Instructions {
    let mut output: Instructions = vec![];

    let it = line.split("");
    let mut num: String = "".to_string();
    for next in it {
        if next == "R" || next == "L" {
            output.push(Instruction::Move(num.parse().unwrap()));
            output.push(Instruction::Turn(next.to_string()));
            num = "".to_string();
        } else {
            num += next;
        }
    }
    if num.len() > 0 {
        output.push(Instruction::Move(num.parse().unwrap()));
    }
    output
}

fn parse_input(contents: &str) -> (Map, Instructions) {
    let bits: Vec<&str> = contents.split("\n\n").collect();
    let map = parse_map(bits[0]);
    let instructions = parse_instructions(bits[1]);
    (map, instructions)
}

#[allow(dead_code)]
fn print(map: &Map, current: &Point) {
    let arrow = [">", "V", "<", "^"][current.2];
    for y in 0..map.len() {
        for x in 0..map.len() {
            if y == current.0 && x == current.1 {
                print!("{arrow}")
            } else {
                print!("{}", map[y][x].value);
            }
        }
        println!();
    }
}

fn nelly_the_elephant(map: &Map, instructions: Instructions) -> i32 {
    let mut current: Point = (0, 0, 0);

    for x in 0..map.len() {
        if map[0][x].value == "." {
            current = (0, x, 0);
            break;
        }
    }
    for instruction in instructions {
        match instruction {
            Instruction::Turn(t) => {
                if t == "L" {
                    current.2 = (current.2 - 1).rem_euclid(4);
                } else {
                    current.2 = (current.2 + 1).rem_euclid(4);
                }
            }
            Instruction::Move(count) => {
                for _ in 0..count {
                    let next = map[current.0][current.1].next[current.2];
                    let next_value = map[next.0][next.1].value.clone();
                    if next_value == "." {
                        current = (next.0, next.1, (current.2 + next.2).rem_euclid(4));
                    } else {
                        break;
                    }
                }
            }
        }
    }

    ((current.0 + 1) * 1000 + (current.1 + 1) * 4 + current.2) as i32
}

fn packed_her_trunk(map: &mut Map) {
    for y in 0..map.len() {
        let mut first = map.len();
        for x in 0..map.len() {
            if map[y][x].value == " " {
                continue;
            }
            first = first.min(x);
            if x + 1 == map.len() || map[y][x + 1].value == " " {
                map[y][x].next[0] = (y, first, 0);
                map[y][first].next[2] = (y, x, 0);
            } else {
                map[y][x].next[0] = (y, x + 1, 0);
                map[y][x + 1].next[2] = (y, x, 0);
            }
        }
    }

    for x in 0..map.len() {
        let mut first = map.len();
        for y in 0..map.len() {
            if map[y][x].value == " " {
                continue;
            }
            first = first.min(y);
            if y + 1 == map.len() || map[y + 1][x].value == " " {
                map[y][x].next[1] = (first, x, 0);
                map[first][x].next[3] = (y, x, 0);
            } else {
                map[y][x].next[1] = (y + 1, x, 0);
                map[y + 1][x].next[3] = (y, x, 0);
            }
        }
    }
}

#[allow(dead_code)]
fn and(map: &mut Map) {
    if map.len() > 20 {
        return;
    }
    let side_len = 4;

    let mut fx = 0;

    for x in 0..map.len() {
        if map[0][x].value == "." {
            fx = x;
            break;
        }
    }

    // Test
    //   1
    // 532
    //   64
    for i in 0..side_len {
        // Done by input
        // 1 <> 2
        // 2 <> 3
        // 3 <> 5
        // 2 <> 6
        // 4 <> 6

        // 1 <> 5
        map[0][fx + i].next[3] = (4, 0 + i, 2);
        map[4][0 + 1].next[3] = (0, fx + i, 2);

        // 1 <> 3
        map[i][fx].next[2] = (4, 4 + i, 3);
        map[4][4 + i].next[3] = (i, fx, 1);

        // 1 <> 4
        map[i][fx + 3].next[0] = (8 + i, fx + 7, 2);
        map[8 + i][fx + 7].next[0] = (i, fx + 3, 2);

        // 2 <> 4
        map[4 + i][fx + 3].next[0] = (8, fx + 7 - i, 1);
        map[8][fx + 7 - i].next[3] = (4 + i, fx + 3, 3);

        // 3 <> 6
        map[7][4 + i].next[1] = (11 - i, fx, 3);
        map[11 - i][fx].next[2] = (7, 4 + i, 1);

        // 4 <> 5
        map[4 + i][0].next[2] = (11, fx + 7 - i, 3);
        map[11][fx + 7 - i].next[1] = (4 + i, 0, 1);

        // 5 <> 6
        map[7][i].next[1] = (11, fx + 3 - i, 2);
        map[11][fx + 3 - i].next[1] = (7, i, 2);
    }
}

#[allow(dead_code)]
fn said_goodbye_to_the_circus(map: &mut Map) {
    if map.len() < 20 {
        return;
    }

    // Input
    //  14
    //  2
    // 36
    // 5
    for i in 0..50 {
        // Done by input
        // 1 <> 2
        // 1 <> 4
        // 2 <> 6
        // 3 <> 6
        // 3 <> 5

        // 1 <> 3
        map[0 + i][50].next[2] = (149 - i, 0, 2);
        map[149 - i][0].next[2] = (0 + i, 50, 2);

        // 1 <> 5
        map[0][50 + i].next[3] = (150 + i, 0, 1);
        map[150 + i][0].next[2] = (0, 50 + i, 3);

        // 2 <> 4
        map[50 + i][99].next[0] = (49, 100 + i, 3);
        map[49][100 + i].next[1] = (50 + i, 99, 1);

        // 2 <> 3
        map[50 + i][50].next[2] = (100, i, 3);
        map[100][i].next[3] = (50 + i, 50, 1);

        // 4 <> 5
        map[0][100 + i].next[3] = (199, i, 0);
        map[199][i].next[1] = (0, 100 + i, 0);

        // 4 <> 6
        map[i][149].next[0] = (149 - i, 99, 2);
        map[149 - i][99].next[0] = (i, 149, 2);

        // 5 <> 6
        map[150 + i][49].next[0] = (149, 50 + i, 3);
        map[149][50 + i].next[1] = (150 + i, 49, 1);
    }
}

fn part1(contents: &str) -> i32 {
    let (mut map, instructions) = parse_input(contents);
    packed_her_trunk(&mut map);
    nelly_the_elephant(&map, instructions)
}

fn part2(contents: &str) -> i32 {
    let (mut map, instructions) = parse_input(contents);
    packed_her_trunk(&mut map);
    and(&mut map);
    said_goodbye_to_the_circus(&mut map);
    nelly_the_elephant(&map, instructions)
}

pub fn run(contents: &str) {
    println!("Part 1: {}", part1(contents));
    println!("Part 2: {}", part2(contents));
}
