use std::{collections::HashMap, fs};

type Rock = Vec<Vec<char>>;
type Point = (usize, usize);
type Abyss = Vec<Vec<char>>;
type Key = (usize, usize, Vec<char>, Vec<char>, Vec<char>);
type Value = usize;

fn parse_rock(content: &str) -> Rock {
    let rock: Rock = content
        .split("\n")
        .map(|line| line.chars().collect())
        .collect();
    let mut r2 = rock.clone();
    for i in 0..rock.len() {
        r2[i] = rock[rock.len() - 1 - i].clone();
    }
    r2
}

fn get_rocks() -> Vec<Rock> {
    let file_path = "./src/d_17/rocks.txt";
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents = (&file_contents).trim_end();

    contents.split("\n\n").map(parse_rock).collect()
}

fn parse_input(contents: &str) -> Vec<i64> {
    contents
        .chars()
        .map(|c| if c == '<' { -1 } else { 1 })
        .collect()
}

#[allow(dead_code)]
fn print(abyss: &Abyss, rock: &Rock, (rx, ry): Point) {
    let mut copy = abyss.clone();
    for i in 0..rock.len() {
        for j in 0..rock[i].len() {
            if rock[i][j] == '#' {
                copy[i + ry][j + rx] = '@';
            }
        }
    }
    for i in (0..(ry + rock.len())).rev() {
        print!("|");
        for j in 0..7 {
            print!("{}", copy[i][j]);
        }
        println!("|");
    }
    println!("+-------+\n");
}

fn will_hit(abyss: &Abyss, rock: &Rock, (rx, ry): Point, (dx, dy): (i64, i64)) -> bool {
    let nx = rx as i64 + dx;
    let ny = ry as i64 + dy;

    if nx < 0 || ny < 0 || nx + rock[0].len() as i64 > 7 {
        return true;
    }

    for i in 0..rock.len() {
        for j in 0..rock[i].len() {
            if rock[i][j] == '#' && abyss[i + ny as usize][j + nx as usize] == '#' {
                return true;
            }
        }
    }

    return false;
}

fn rock_and_roll(contents: &str, target: usize) -> usize {
    let total_rocks = 100000;
    let wind = parse_input(contents);
    let rocks: Vec<Rock> = get_rocks();

    let mut abyss: Abyss = vec![vec!['.'; 7]; total_rocks * 4];
    let mut highest: usize = 0;
    let mut wct = 0;
    let mut rct = 0;

    let mut cache: HashMap<Key, Value> = HashMap::new();
    let mut counts: Vec<usize> = vec![];

    for _ in 0..100000 {
        let rock = &rocks[rct % rocks.len()];
        rct += 1;

        let (mut rx, mut ry): Point = (2, highest + 3);

        loop {
            let wind = wind[wct % wind.len()];
            if !will_hit(&abyss, rock, (rx, ry), (wind, 0)) {
                rx = (rx as i64 + wind) as usize;
            }
            wct += 1;

            if will_hit(&abyss, rock, (rx, ry), (0, -1)) {
                break;
            }
            ry = (ry as i64 - 1) as usize;
        }

        for i in 0..rock.len() {
            for j in 0..rock[i].len() {
                if rock[i][j] == '#' {
                    abyss[i + ry][j + rx] = '#';
                }
            }
        }
        highest = highest.max(ry + rock.len());
        counts.push(highest);

        if highest < 5 {
            continue;
        }
        let top_row: Vec<char> = (0..7).map(|n| abyss[highest - 1][n]).collect();
        let second_row: Vec<char> = (0..7).map(|n| abyss[highest - 2][n]).collect();
        let third_row: Vec<char> = (0..7).map(|n| abyss[highest - 3][n]).collect();

        let key: Key = (
            rct % rocks.len(),
            wct % wind.len(),
            top_row,
            second_row,
            third_row,
        );
        if cache.contains_key(&key) {
            break;
        }
        cache.insert(key, rct);
    }

    let top_row: Vec<char> = (0..7).map(|n| abyss[highest - 1][n]).collect();
    let second_row: Vec<char> = (0..7).map(|n| abyss[highest - 2][n]).collect();
    let third_row: Vec<char> = (0..7).map(|n| abyss[highest - 3][n]).collect();
    let key: Key = (
        rct % rocks.len(),
        wct % wind.len(),
        top_row,
        second_row,
        third_row,
    );
    let old_rct = cache.get(&(key.clone())).unwrap();

    let smaller_target = target - *old_rct;
    let repeated_rocks = rct - old_rct;

    let multiple = smaller_target.div_euclid(repeated_rocks);
    let repeated_height = highest - counts[old_rct - 1];

    let modulo = smaller_target % repeated_rocks;
    let first_rocks = counts[old_rct + modulo - 1];

    return multiple * repeated_height + first_rocks;
}

fn part1(contents: &str) -> usize {
    rock_and_roll(contents, 2022)
}

fn part2(contents: &str) -> usize {
    rock_and_roll(contents, 1000000000000)
}

pub fn run(contents: &str) {
    println!("Part 1: {}", part1(contents));
    println!("Part 2: {}", part2(contents));
}
