use std::collections::{HashMap, HashSet};

type Elf = (i32, i32);

fn parse_input(contents: &str) -> HashSet<Elf> {
    let mut elves = HashSet::<Elf>::new();
    contents.split('\n').enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c == '#' {
                elves.insert((y as i32, x as i32));
            }
        })
    });
    elves
}

fn has_neighbour(elves: &HashSet<Elf>, elf: &Elf) -> bool {
    for y in -1..=1 {
        for x in -1..=1 {
            if x == 0 && y == 0 {
                continue;
            }
            if elves.contains(&(elf.0 + y, elf.1 + x)) {
                return true;
            }
        }
    }
    false
}

fn get_proposed(elves: &HashSet<Elf>, elf: &Elf, i: i32) -> Elf {
    let north = [(-1, -1), (-1, 0), (-1, 1)];
    let south = [(1, -1), (1, 0), (1, 1)];
    let west = [(-1, -1), (0, -1), (1, -1)];
    let east = [(-1, 1), (0, 1), (1, 1)];

    let check = |list: [(i32, i32); 3]| -> bool {
        list.iter()
            .map(|e| (e.0 + elf.0, e.1 + elf.1))
            .all(|e| !elves.contains(&e))
    };

    let list = [north, south, west, east];
    for dir in 0..4 {
        let d = list[(dir + i as usize) % 4];
        if check(d) {
            return (elf.0 + d[1].0, elf.1 + d[1].1);
        }
    }

    elf.clone()
}

fn i_am_the_lord_of_dance(elves: &HashSet<Elf>, i: i32) -> HashSet<Elf> {
    let mut next_elves = HashSet::<Elf>::new();

    let mut proposed = HashMap::<Elf, Vec<Elf>>::new();

    elves
        .iter()
        .filter(|elf| {
            if has_neighbour(elves, &elf) {
                return true;
            }
            next_elves.insert(*elf.clone());
            return false;
        })
        .for_each(|elf| {
            let to = get_proposed(elves, elf, i);
            let list = proposed.get_mut(&to);
            if list.is_none() {
                proposed.insert(to, vec![elf.clone()]);
            } else {
                list.unwrap().push(elf.clone());
            }
        });

    let smaller_proposed: Vec<(&Elf, &Vec<Elf>)> = proposed
        .iter()
        .filter(|(key, value)| {
            if value.len() > 1 || next_elves.contains(key) {
                for e in value.iter() {
                    next_elves.insert(e.clone());
                }

                return false;
            }
            return true;
        })
        .collect();

    smaller_proposed.iter().for_each(|(key, _)| {
        next_elves.insert(*key.clone());
    });
    next_elves
}

fn i_will_lead_you_all(elves: &HashSet<Elf>) -> i32 {
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;

    elves.iter().for_each(|(y, x)| {
        min_x = min_x.min(*x);
        max_x = max_x.max(*x);
        min_y = min_y.min(*y);
        max_y = max_y.max(*y);
    });

    (max_x - min_x + 1) * (max_y - min_y + 1) - elves.len() as i32
}

#[allow(dead_code)]
fn print(elves: &HashSet<Elf>) {
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;

    elves.iter().for_each(|(y, x)| {
        min_x = min_x.min(*x);
        max_x = max_x.max(*x);
        min_y = min_y.min(*y);
        max_y = max_y.max(*y);
    });

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if elves.contains(&(y, x)) {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!()
    }
    println!()
}

fn dance_dance_wherever_you_may_be(elves: HashSet<Elf>) -> i32 {
    let mut next = elves.clone();

    for i in 0..10 {
        next = i_am_the_lord_of_dance(&next, i);
    }
    i_will_lead_you_all(&next)
}

fn always_read_instructions_carefully(elves: HashSet<Elf>) -> i32 {
    let mut next = elves.clone();

    let mut i = 0;
    loop {
        let could_be_next = i_am_the_lord_of_dance(&next, i);
        i += 1;
        if could_be_next.iter().all(|elf| next.contains(elf)) {
            break;
        }
        next = could_be_next;
    }
    i
}

fn part1(contents: &str) -> i32 {
    let elves = parse_input(contents);
    dance_dance_wherever_you_may_be(elves)
}

fn part2(contents: &str) -> i32 {
    let elves = parse_input(contents);
    always_read_instructions_carefully(elves)
}

pub fn run(contents: &str) {
    println!("Part 1: {}", part1(contents));
    println!("Part 2: {}", part2(contents));
}
