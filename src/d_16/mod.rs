use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone)]
struct Valve {
    id: String,
    flow_rate: i64,
    leads_to: Vec<String>,
    shortest_path: HashMap<String, i64>,
}

#[derive(Debug, Clone)]
struct Current {
    time_remaining: i64,
    current_flow: i64,
    current_node: String,
    open: HashSet<String>,
    path: Vec<String>,
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
                return ct + 1;
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

fn open_valve(valves: &HashMap<String, Valve>, current: &Current) -> Current {
    let mut next = current.clone();
    next.time_remaining -= 1;
    next.current_flow += next.time_remaining * valves.get(&current.current_node).unwrap().flow_rate;
    next.open.insert(next.current_node.clone());
    next
}

static TOTAL_TIME: i64 = 31;
fn part1(contents: &str) -> i64 {
    let valves = parse_input(contents);
    // println!("{:?}", valves);
    // println!("\n{:?}", valves.get("AA").unwrap());
    // println!("{:?}", valves.get("EE").unwrap());
    // println!("{:?}", valves.get("CC").unwrap());
    // println!("{:?}", valves.get("HH").unwrap());
    // println!("{:?}", valves.get("JJ").unwrap());
    // println!("{:?}", valves.get("DD").unwrap());
    // println!("{:?}\n", valves.get("BB").unwrap());

    let mut max: i64 = 0;
    let mut queue: VecDeque<Current> = VecDeque::new();
    queue.push_back(Current {
        time_remaining: TOTAL_TIME,
        current_flow: 0,
        current_node: "AA".to_string(),
        open: HashSet::new(),
        path: vec!["AA".to_string()],
    });

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        if max < current.current_flow {
            println!("{:?}", current.path);
        }
        max = max.max(current.current_flow);

        if !current.open.contains(&current.current_node) {
            let next = open_valve(&valves, &current);
            if next.time_remaining > 0 {
                queue.push_back(next);
            }
        } else {
            let valve = valves.get(&current.current_node).unwrap();
            valve
                .shortest_path
                .iter()
                .filter(|(a, _b)| !current.open.contains(*a))
                .for_each(|(a, b)| {
                    let mut next = current.clone();
                    next.time_remaining -= b;
                    next.current_node = a.clone();
                    next.path.push(a.clone());
                    if next.time_remaining > 0 {
                        queue.push_back(next);
                    }
                })
        }
    }
    // The master plan
    //   - Identify real valves
    //   - Calculate shortest path between all real valves
    //       - fn shortest_path(valves, from: "", to: "") -> i64
    //   - Calculate shortest path from start to real valves
    //   - DFS should work at this point
    //       - time_remaining=30
    //       - time_remaining=time_remaining - distances
    //       - score += flow_rate * time_remaining--
    //       - while time_remaining > 30
    return max;
}

fn part2(_contents: &str) -> i64 {
    return 0;
}

pub fn run(contents: &str) {
    println!("Part 1: {}", part1(contents));
    println!("Part 2: {}", part2(contents));
}
