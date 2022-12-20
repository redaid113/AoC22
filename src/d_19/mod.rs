type Price = Vec<u32>;
type Robots = Vec<Price>;
#[derive(Debug)]
struct Blueprint {
    id: u32,
    robots: Robots,
}

#[derive(Clone, Debug)]
struct State {
    robot_count: Vec<u32>,
    ore_count: Vec<u32>,
}

fn parse_price(line: &str) -> Price {
    let bits: Vec<&str> = line.split(" ").collect();
    let mut price = vec![0; 4];
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

fn ceil(a: u32, b: u32) -> u32 {
    let div = a.div_euclid(b);
    let m = a % b;
    if m > 0 {
        return div + 1;
    }
    return div;
}

fn calculate_step(price: &Price, ore_count: &Vec<u32>, robot_count: &Vec<u32>) -> u32 {
    let mut step = 0;
    for p in 0..3 {
        if price[p] > 0 && robot_count[p] == 0 {
            return 0;
        } else if price[p] > ore_count[p] {
            step = step.max(ceil(price[p] - ore_count[p], robot_count[p]));
        }
    }
    step + 1
}

fn hit_the_mother_lode(robots: &Robots, max_iteration: usize) -> u32 {
    let mut iterations: Vec<Vec<State>> = vec![];
    for _ in 0..=max_iteration {
        iterations.push(vec![]);
    }
    iterations[0].push(State {
        ore_count: vec![0, 0, 0, 0],
        robot_count: vec![1, 0, 0, 0],
    });

    let mut max_robots = [u32::MAX; 4];
    for i in 0..3 {
        max_robots[i] = robots.iter().map(|r| r[i]).max().unwrap();
    }

    let mut max = 0;
    for i in 0..max_iteration {
        let geodudes = iterations[i]
            .iter()
            .map(|iteration| iteration.robot_count[3])
            .max();

        for j in 0..iterations[i].len() {
            let iteration = iterations[i][j].clone();
            if iteration.robot_count[3] != geodudes.unwrap()
                && iteration.robot_count[3] != geodudes.unwrap() - 1
            {
                continue;
            }
            max = max.max(
                iteration.ore_count[3]
                    + iteration.robot_count[3] * (max_iteration as u32 - i as u32),
            );
            (0..4).for_each(|r| {
                if max_robots[r] == iteration.robot_count[r] {
                    return;
                }
                let price = &robots[r];

                let step = calculate_step(price, &iteration.ore_count, &iteration.robot_count);
                if step == 0 {
                    return;
                }

                if i + step as usize > max_iteration {
                    return;
                }

                let mut ore_count = iteration.ore_count.clone();
                for o in 0..4 {
                    ore_count[o] += iteration.robot_count[o] * step - price[o];
                }
                let mut robot_count = iteration.robot_count.clone();
                robot_count[r] += 1;

                max = max.max(ore_count[3]);

                iterations[i + step as usize].push(State {
                    ore_count,
                    robot_count,
                });
            })
        }
    }

    max
}

fn part1(contents: &str) -> u32 {
    let blueprints = parse_input(contents);

    blueprints
        .iter()
        .map(|blueprint| blueprint.id * hit_the_mother_lode(&blueprint.robots, 24))
        .sum()
}

fn part2(contents: &str) -> u32 {
    let blueprints = parse_input(contents);
    blueprints
        .iter()
        .take(3)
        .map(|blueprint| hit_the_mother_lode(&blueprint.robots, 32))
        .product()
}

pub fn run(contents: &str) {
    println!("Part 1: {}", part1(contents));
    println!("Part 2: {}", part2(contents));
}
