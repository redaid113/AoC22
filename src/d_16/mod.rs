use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone)]
struct Valve {
    id: String,
    flow_rate: i64,
    leads_to: Vec<String>,
    shortest_path: HashMap<String, i64>,
}

#[derive(Debug, Clone)]
struct Node {
    time_remaining: i64,
    current_node: String,
}

#[derive(Debug, Clone)]

struct Current {
    node: Node,
    elephant: Node,
    current_flow: i64,
    open: HashSet<String>,
}

fn parse_leads_to(line: &str) -> Vec<String> {
    line.split(", ").map(|v| v.to_string()).collect()
}

fn parse_line(line: &str) -> Valve {
    let reduced_line = line
        .replace("Valve ", "")
        .replace(" has flow rate=", ";")
        .replace(" tunnels lead to valves ", "")
        .replace(" tunnel leads to valve ", "");

    let bits: Vec<&str> = reduced_line.split(";").collect();
    Valve {
        id: bits[0].to_string(),
        flow_rate: bits[1].parse().unwrap(),
        leads_to: parse_leads_to(bits[2]),
        shortest_path: HashMap::new(),
    }
}

fn shortest_path(valves: &HashMap<String, Valve>, from: &str, to: &str) -> i64 {
    let mut queue: VecDeque<(&str, i64)> = VecDeque::new();
    queue.push_back((from, 0));

    while queue.len() > 0 {
        let (cur, ct) = queue.pop_front().unwrap();
        let leads_to = &valves.get(cur).unwrap().leads_to;
        for i in leads_to {
            if i == to {
                return ct + 2;
            }
            queue.push_back((i, ct + 1));
        }
    }
    return 0;
}

fn parse_input(contents: &str) -> HashMap<String, Valve> {
    let valves: Vec<Valve> = contents.split("\n").map(parse_line).collect();

    let real_valves: Vec<&str> = valves
        .iter()
        .filter(|v| v.flow_rate != 0)
        .map(|v| &v.id[..])
        .collect();

    let mut output: HashMap<String, Valve> = HashMap::new();
    let mut all_valves = HashMap::<String, Valve>::new();
    valves.iter().for_each(|valve| {
        all_valves.insert(valve.id.clone(), (*valve).clone());
        if valve.flow_rate > 0 {
            output.insert(valve.id.clone(), (*valve).clone());
        }
    });

    for i in 0..real_valves.len() - 1 {
        for j in i + 1..real_valves.len() {
            let distance = shortest_path(&all_valves, real_valves[i], real_valves[j]);
            let valve_i = output.get_mut(real_valves[i]).unwrap();
            valve_i
                .shortest_path
                .insert(real_valves[j].to_string(), distance);

            let valve_j = output.get_mut(real_valves[j]).unwrap();
            valve_j
                .shortest_path
                .insert(real_valves[i].to_string(), distance);
        }
    }

    let mut start = all_valves.get("AA").unwrap().clone();
    for i in 0..real_valves.len() {
        let distance = shortest_path(&all_valves, "AA", real_valves[i]);
        start
            .shortest_path
            .insert(real_valves[i].to_string(), distance);
    }
    output.insert("AA".to_string(), start);

    return output;
}

fn get_next(valves: &HashMap<String, Valve>, open: &HashSet<String>, current: &Node) -> Vec<Node> {
    let valve = valves.get(&current.current_node).unwrap();
    valve
        .shortest_path
        .iter()
        .filter(|(a, _b)| !open.contains(*a))
        .map(|(a, b)| {
            let mut next = current.clone();
            next.time_remaining -= b;
            next.current_node = a.clone();
            next
        })
        .filter(|next| next.time_remaining > 0)
        .collect()
}

static TOTAL_TIME: i64 = 30;
fn part1(contents: &str) -> i64 {
    let valves = parse_input(contents);

    let mut max: i64 = 0;
    let mut queue: VecDeque<Current> = VecDeque::new();
    let mut max_visits: HashMap<(String, i64, String, i64), i64> = HashMap::new();

    queue.push_back(Current {
        node: Node {
            time_remaining: TOTAL_TIME,
            current_node: "AA".to_string(),
        },
        elephant: Node {
            time_remaining: TOTAL_TIME,
            current_node: "AA".to_string(),
        },
        current_flow: 0,
        open: HashSet::new(),
    });

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        let key1 = (
            current.node.current_node.clone(),
            current.node.time_remaining,
            current.elephant.current_node.clone(),
            current.elephant.time_remaining,
        );
        let key2 = (
            current.elephant.current_node.clone(),
            current.elephant.time_remaining,
            current.node.current_node.clone(),
            current.node.time_remaining,
        );
        if max_visits.contains_key(&key1) {
            if max_visits.get(&key1).unwrap() >= &current.current_flow {
                continue;
            }
        }
        max_visits.insert(key1, current.current_flow);
        max_visits.insert(key2, current.current_flow);

        max = max.max(current.current_flow);

        get_next(&valves, &current.open, &current.node)
            .iter()
            .for_each(|node| {
                let mut next = current.clone();
                next.node = node.clone();
                next.current_flow +=
                    node.time_remaining * valves.get(&node.current_node).unwrap().flow_rate;
                next.open.insert(node.current_node.clone());

                queue.push_back(next);
            });
    }
    return max;
}

fn part2(contents: &str) -> i64 {
    let valves = parse_input(contents);

    let mut max: i64 = 0;
    let mut queue: VecDeque<Current> = VecDeque::new();
    let mut max_visits: HashMap<(String, i64, String, i64), i64> = HashMap::new();

    queue.push_back(Current {
        node: Node {
            time_remaining: TOTAL_TIME - 4,
            current_node: "AA".to_string(),
        },
        elephant: Node {
            time_remaining: TOTAL_TIME - 4,
            current_node: "AA".to_string(),
        },
        current_flow: 0,
        open: HashSet::new(),
    });

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        let key1 = (
            current.node.current_node.clone(),
            current.node.time_remaining,
            current.elephant.current_node.clone(),
            current.elephant.time_remaining,
        );
        let key2 = (
            current.elephant.current_node.clone(),
            current.elephant.time_remaining,
            current.node.current_node.clone(),
            current.node.time_remaining,
        );
        if max_visits.contains_key(&key1) {
            if max_visits.get(&key1).unwrap() >= &current.current_flow {
                continue;
            }
        }
        max_visits.insert(key1, current.current_flow);
        max_visits.insert(key2, current.current_flow);

        max = max.max(current.current_flow);

        let current_next = get_next(&valves, &current.open, &current.node);
        let elephant_next = get_next(&valves, &current.open, &current.elephant);

        if current_next.len() == 0 {
            elephant_next.iter().for_each(|node| {
                let mut next = current.clone();
                next.elephant = node.clone();
                next.current_flow +=
                    node.time_remaining * valves.get(&node.current_node).unwrap().flow_rate;
                next.open.insert(node.current_node.clone());
                queue.push_back(next);
            });
            continue;
        }
        if elephant_next.len() == 0 {
            current_next.iter().for_each(|node| {
                let mut next = current.clone();
                next.node = node.clone();
                next.current_flow +=
                    node.time_remaining * valves.get(&node.current_node).unwrap().flow_rate;
                next.open.insert(node.current_node.clone());

                queue.push_back(next);
            });
            continue;
        }

        for c in current_next {
            for e in &elephant_next {
                if c.current_node != e.current_node {
                    let mut next = current.clone();

                    next.node = c.clone();
                    next.elephant = e.clone();

                    next.current_flow +=
                        c.time_remaining * valves.get(&c.current_node).unwrap().flow_rate;
                    next.current_flow +=
                        e.time_remaining * valves.get(&e.current_node).unwrap().flow_rate;
                    next.open.insert(next.node.current_node.clone());
                    next.open.insert(next.elephant.current_node.clone());

                    queue.push_back(next);
                }
            }
        }
    }
    return max;
}

pub fn run(contents: &str) {
    println!("Part 1: {}", part1(contents));
    println!("Part 2: {}", part2(contents));
}
