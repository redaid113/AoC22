use std::collections::{HashSet, VecDeque};

type Point = Vec<i64>;
type Range = Vec<(i64, i64)>;

fn parse_point(line: &str) -> Point {
    line.split(',').map(|c| c.parse().unwrap()).collect()
}

fn parse_input(contents: &str) -> Vec<Point> {
    contents.split('\n').map(parse_point).collect()
}

fn touching(a: &Point, b: &Point) -> bool {
    (a[0].abs_diff(b[0]) + a[1].abs_diff(b[1]) + a[2].abs_diff(b[2])) == 1
}

fn surface_area(points: &Vec<Point>) -> usize {
    let mut sides = points.len() * 6;
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            if touching(&points[i], &points[j]) {
                sides -= 2;
            }
        }
    }
    sides
}

fn part1(contents: &str) -> usize {
    let points = parse_input(contents);
    surface_area(&points)
}

fn get_range(points: &Vec<Point>) -> Range {
    let mut range: Range = vec![];
    for i in 0..3 {
        let mut min = points[0][i] - 1;
        let mut max = points[0][i] + 1;
        for point in points {
            min = min.min(point[i] - 1);
            max = max.max(point[i] + 1);
        }
        range.push((min, max));
    }
    range
}

fn in_range(point: &Point, range: &Range) -> bool {
    for i in 0..3 {
        if point[i] < range[i].0 || point[i] > range[i].1 {
            return false;
        }
    }
    return true;
}

fn find_outside(points: &Vec<Point>, range: &Range) -> HashSet<Point> {
    let mut seen: HashSet<Point> = HashSet::new();
    for point in points {
        seen.insert(point.clone());
    }

    let mut outside: HashSet<Point> = HashSet::new();
    let mut queue: VecDeque<Point> = VecDeque::new();
    queue.push_back(vec![range[0].0, range[1].0, range[2].0]);
    while queue.len() > 0 {
        let next = queue.pop_front().unwrap();
        if seen.contains(&next) || !in_range(&next, range) {
            continue;
        }
        seen.insert(next.clone());
        outside.insert(next.clone());
        queue.push_back(vec![next[0] + 1, next[1], next[2]]);
        queue.push_back(vec![next[0] - 1, next[1], next[2]]);
        queue.push_back(vec![next[0], next[1] + 1, next[2]]);
        queue.push_back(vec![next[0], next[1] - 1, next[2]]);
        queue.push_back(vec![next[0], next[1], next[2] + 1]);
        queue.push_back(vec![next[0], next[1], next[2] - 1]);
    }

    outside
}

fn generate_ball(points: &Vec<Point>) -> Vec<Point> {
    let range = get_range(points);
    let mut ball = vec![];

    let outside: HashSet<Point> = find_outside(points, &range);

    for x in range[0].0..range[0].1 {
        for y in range[1].0..range[1].1 {
            for z in range[2].0..range[2].1 {
                let point: Point = vec![x, y, z];
                if !outside.contains(&point) {
                    ball.push(point);
                }
            }
        }
    }

    ball
}

fn part2(contents: &str) -> usize {
    let points = parse_input(contents);
    let ball = generate_ball(&points);
    surface_area(&ball)
}

pub fn run(contents: &str) {
    println!("Part 1: {}", part1(contents));
    println!("Part 2: {}", part2(contents));
}
