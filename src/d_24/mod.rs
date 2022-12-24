use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

type Point = (usize, usize);

#[derive(Debug, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone)]
struct Blizzard {
    position: Point,
    direction: Direction,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]

struct Node {
    cycle: usize,
    position: Point,
    distance: usize,
}

fn heuristic(node: &Node) -> usize {
    node.distance + node.cycle
}

impl PartialOrd for Node {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        let l = heuristic(self);
        let r = heuristic(rhs);
        r.partial_cmp(&l)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, Clone)]
struct Map {
    top_gap: Point,
    bottom_gap: Point,
    width: usize,
    height: usize,
    blizzards: HashMap<usize, HashSet<Point>>,
    lcm: usize,
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    return gcd(b, a % b);
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn parse_blizzard(contents: &str) -> Vec<Blizzard> {
    let lines: Vec<&str> = contents.split('\n').collect();

    let mut blizzards = vec![];

    for y in 0..lines.len() {
        let line = lines[y].as_bytes();
        for x in 0..line.len() {
            let c = line[x] as char;
            let mut blizzard = Blizzard {
                position: (y, x),
                direction: Direction::Down,
            };
            match c {
                '>' => blizzard.direction = Direction::Right,
                '<' => blizzard.direction = Direction::Left,
                '^' => blizzard.direction = Direction::Up,
                'v' => blizzard.direction = Direction::Down,
                _ => continue,
            }
            blizzards.push(blizzard);
        }
    }

    blizzards
}

fn parse_height_widith(contents: &str) -> (usize, usize) {
    let lines: Vec<&str> = contents.split('\n').collect();
    let height = lines.len();
    let width = lines[0].as_bytes().len();
    (height, width)
}

fn next_blizzard(last: &Vec<Blizzard>, height: usize, width: usize) -> Vec<Blizzard> {
    last.iter()
        .map(|blizzard| {
            let (y, x) = blizzard.position;

            match blizzard.direction {
                Direction::Right => {
                    return Blizzard {
                        position: (y, (x + width).rem_euclid(width) + 1),
                        direction: Direction::Right,
                    }
                }
                Direction::Left => {
                    return Blizzard {
                        position: (y, (x + width - 2).rem_euclid(width) + 1),
                        direction: Direction::Left,
                    }
                }
                Direction::Up => {
                    return Blizzard {
                        position: ((y - 2 + height).rem_euclid(height) + 1, x),
                        direction: Direction::Up,
                    }
                }
                Direction::Down => {
                    return Blizzard {
                        position: ((y + height).rem_euclid(height) + 1, x),
                        direction: Direction::Down,
                    }
                }
            }
        })
        .collect()
}

fn parse_blizzards(contents: &str, height: usize, width: usize) -> HashMap<usize, HashSet<Point>> {
    let mut blizzards: HashMap<usize, Vec<Blizzard>> = HashMap::new();
    let init = parse_blizzard(contents);
    blizzards.insert(0, init);

    for i in 1..=lcm(height, width) {
        let last = blizzards.get(&(i - 1)).unwrap();
        let cur = next_blizzard(last, height, width);

        blizzards.insert(i, cur);
    }

    let mut output: HashMap<usize, HashSet<Point>> = HashMap::new();
    for i in 0..=lcm(height, width) {
        let list = blizzards.get(&i).unwrap();
        let mut set = HashSet::new();
        for b in list {
            set.insert(b.position);
        }
        output.insert(i, set);
    }

    output
}

fn parse_input(contents: &str) -> Map {
    let (true_height, true_width) = parse_height_widith(contents);
    let height = true_height - 2;
    let width = true_width - 2;

    let blizzards = parse_blizzards(contents, height, width);

    Map {
        top_gap: (0, 1),
        bottom_gap: (true_height - 1, true_width - 2),
        width,
        height,
        blizzards,
        lcm: lcm(height, width),
    }
}

fn distance(a: Point, b: Point) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

fn shortest_path(map: &Map, start: Point, end: Point, cycle: usize) -> usize {
    let mut seen: HashSet<Node> = HashSet::new();
    let mut min_heap: BinaryHeap<Node> = BinaryHeap::new();
    min_heap.push(Node {
        cycle,
        position: start,
        distance: distance(start, end),
    });

    while !min_heap.is_empty() {
        let current = min_heap.pop().unwrap();
        if seen.contains(&current) {
            continue;
        }
        seen.insert(current.clone());

        let (y, x) = current.position;
        let cycle = current.cycle + 1;

        let left = (y, x - 1);
        let right = (y, x + 1);
        let up = (y - 1, x);
        let down = (y + 1, x);
        let stay = (y, x);

        if up == end || down == end {
            return cycle;
        }

        let blizzards = map.blizzards.get(&(cycle % map.lcm)).unwrap();
        let mut add = |p: Point| {
            if p.1 == 0 || p.0 == 0 || p.1 >= map.width + 1 || p.0 >= map.height + 1 {
                return;
            }
            if blizzards.contains(&p) {
                return;
            }

            min_heap.push(Node {
                position: p,
                distance: distance(p, end),
                cycle,
            });
        };
        add(left);
        add(right);
        add(up);
        add(down);

        if !blizzards.contains(&stay) {
            min_heap.push(Node {
                position: stay,
                distance: distance(stay, end),
                cycle,
            });
        }
    }
    0
}

fn part1(contents: &str) -> usize {
    let map = parse_input(contents);
    shortest_path(&map, map.top_gap, map.bottom_gap, 0)
}

fn part2(contents: &str) -> usize {
    let map = parse_input(contents);
    let made_it = shortest_path(&map, map.top_gap, map.bottom_gap, 0);
    let stupid_elf = shortest_path(&map, map.bottom_gap, map.top_gap, made_it);
    shortest_path(&map, map.top_gap, map.bottom_gap, stupid_elf)
}

pub fn run(contents: &str) {
    println!("Part 1: {}", part1(contents));
    println!("Part 2: {}", part2(contents));
}
