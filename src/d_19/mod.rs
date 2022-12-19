use std::collections::{HashMap, VecDeque};

type Price = Vec<u32>;
type Robots = Vec<Price>;
#[derive(Debug)]
struct Blueprint {
    id: u32,
    robots: Robots,
}

struct State {
    robot_count: Vec<u32>,
    ore_count: Vec<u32>,
    iterations: u32,
}

fn parse_price(line: &str) -> Price {
    let bits: Vec<&str> = line.split(" ").collect();
    let mut price = vec![0; 3];
    for i in 0..bits.len() / 2 {
        let num: u32 = bits[i * 2].parse().unwrap();
        let lode = bits[i * 2 + 1];
        match lode {
            "ore" => price[0] += num,
            "clay" => price[1] += num,
            "obsidian" => price[2] += num,
            _ => {}
        }
    }

    price
}

fn parse_id(line: &str) -> u32 {
    line.parse().unwrap()
}

// Blueprint 1: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 15 clay. Each geode robot costs 2 ore and 8 obsidian.
fn parse_blueprint(line: &str) -> Blueprint {
    let formatted = line
        .replace("Blueprint ", "")
        .replace(": Each ore robot costs ", ",")
        .replace(" Each clay robot costs ", ",")
        .replace(" Each obsidian robot costs ", ",")
        .replace(" Each geode robot costs ", ",")
        .replace(".", "")
        .replace(" and", "");
    let sections: Vec<&str> = formatted.split(',').collect();

    let robots = vec![
        parse_price(sections[1]),
        parse_price(sections[2]),
        parse_price(sections[3]),
        parse_price(sections[4]),
    ];
    Blueprint {
        id: parse_id(sections[0]),
        robots,
    }
}

fn parse_input(contents: &str) -> Vec<Blueprint> {
    contents.split('\n').map(parse_blueprint).collect()
}

fn can_build(price: &Price, ores: &Vec<u32>) -> bool {
    for i in 0..3 {
        if price[i] > ores[i] {
            return false;
        }
    }
    true
}

fn build_robots(robots: &Robots, ores: &Vec<u32>) -> Vec<Vec<u32>> {
    if can_build(&robots[3], ores) {
        return vec![vec![0, 0, 0, 1]];
    }

    if can_build(&robots[2], ores) {
        return vec![vec![0, 0, 1, 0]];
    }

    let mut output: Vec<Vec<u32>> = vec![vec![0, 0, 0, 0]];
    if can_build(&robots[1], ores) {
        output.push(vec![0, 1, 0, 0]);
    }

    if can_build(&robots[0], ores) {
        output.push(vec![1, 0, 0, 0]);
    }
    output
}

fn mine_time(robots: &Robots, max_iteration: u32) -> u32 {
    let mut queue: VecDeque<State> = VecDeque::new();
    let mut max = 0;

    let mut max_robots = [0; 3];
    for i in 0..3 {
        max_robots[i] = robots.iter().map(|r| r[i]).max().unwrap();
    }

    queue.push_back(State {
        robot_count: vec![1, 0, 0, 0],
        ore_count: vec![0, 0, 0, 0],
        iterations: 0,
    });

    let mut cache: HashMap<String, Vec<u32>> = HashMap::new();

    while !queue.is_empty() {
        let state = queue.pop_front().unwrap();
        if state.iterations == max_iteration {
            max = max.max(*state.ore_count.last().unwrap());
            continue;
        }
        if (0..3).any(|i| state.robot_count[i] > max_robots[i]) {
            continue;
        }

        let key = format!("{}.{:?}", state.iterations, state.robot_count);
        if cache.contains_key(&key) {
            let prev = cache.get(&key).unwrap();
            let nope = state
                .ore_count
                .iter()
                .enumerate()
                .all(|(i, o)| *o <= prev[i]);
            if nope {
                continue;
            }
        }
        cache.insert(key, state.ore_count.clone());

        let mined = state.robot_count.clone();
        build_robots(robots, &state.ore_count)
            .iter()
            .for_each(|built_robots| {
                let mut next_ore_count = vec![0; 4];
                let mut next_robots = vec![0; 4];

                for i in 0..4 {
                    next_ore_count[i] = mined[i] + state.ore_count[i];
                }

                for i in 0..4 {
                    next_robots[i] = built_robots[i] + state.robot_count[i];

                    for o in 0..3 {
                        let ore_cost = robots[i][o];
                        next_ore_count[o] -= built_robots[i] * ore_cost;
                    }
                }

                queue.push_back(State {
                    robot_count: next_robots,
                    ore_count: next_ore_count,
                    iterations: state.iterations + 1,
                });
            })
    }
    max
}

fn part1(contents: &str) -> u32 {
    let blueprints = parse_input(contents);

    blueprints
        .iter()
        .map(|blueprint| blueprint.id * mine_time(&blueprint.robots, 24))
        .sum()
}

fn part2(contents: &str) -> u32 {
    let blueprints = parse_input(contents);
    mine_time(&blueprints[0].robots, 32)
        * mine_time(&blueprints[1].robots, 32)
        * mine_time(&blueprints[2].robots, 32)
}

pub fn run(contents: &str) {
    println!("Part 1: {}", part1(contents));
    println!("Part 2: {}", part2(contents));
}
