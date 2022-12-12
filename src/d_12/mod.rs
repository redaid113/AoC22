use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

#[derive(Debug, PartialEq, Eq)]
struct Location {
    height: i32,
    place: (isize, isize),
    steps: i32,
}

impl Ord for Location {
    fn cmp(&self, other: &Self) -> Ordering {
        return other.steps.cmp(&self.steps);
    }
}

impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_char(c: char) -> i32 {
    if c == 'S' {
        return 'a' as i32 - 'a' as i32;
    }
    if c == 'E' {
        return 'z' as i32 - 'a' as i32;
    }
    return c as i32 - 'a' as i32;
}

fn parse_line(line: &str) -> Vec<i32> {
    return line.chars().map(parse_char).collect();
}

fn parse_map(contents: &str) -> Vec<Vec<i32>> {
    return contents.split("\n").map(parse_line).collect();
}

fn find_char(contents: &str, ch: char) -> (isize, isize) {
    let mut start = (0, 0);
    contents.split("\n").enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, c)| {
            if c == ch {
                start = (i as isize, j as isize);
            }
        })
    });
    return start;
}

fn shortest_path(map: &Vec<Vec<i32>>, start: (isize, isize), end: (isize, isize)) -> i32 {
    let max_x = map.len() as isize - 1;
    let max_y = map[0].len() as isize - 1;

    let mut current: Location = Location {
        height: 0,
        place: start,
        steps: 0,
    };

    let mut seen = HashSet::<(isize, isize)>::new();
    let mut min_heap = BinaryHeap::<Location>::new();
    min_heap.push(current);

    while min_heap.len() > 0 {
        current = min_heap.pop().unwrap();

        if current.place == end {
            return current.steps;
        }
        if seen.contains(&current.place) {
            continue;
        }
        seen.insert(current.place);

        [
            (current.place.0 - 1, current.place.1),
            (current.place.0 + 1, current.place.1),
            (current.place.0, current.place.1 - 1),
            (current.place.0, current.place.1 + 1),
        ]
        .iter()
        .filter(|(x, y)| {
            return x >= &0 && x <= &max_x && y >= &0 && y <= &max_y;
        })
        .filter(|place| {
            return !seen.contains(place);
        })
        .filter(|(x, y)| return map[*x as usize][*y as usize] - current.height <= 1)
        .for_each(|(x, y)| {
            let location = Location {
                place: (*x, *y),
                height: map[*x as usize][*y as usize],
                steps: current.steps + 1,
            };

            min_heap.push(location);
        });
    }

    return 0;
}

fn part1(contents: &str) -> i32 {
    let map = parse_map(contents);
    let start = find_char(contents, 'S');
    let end = find_char(contents, 'E');

    return shortest_path(&map, start, end);
}

fn part2(contents: &str) -> i32 {
    let map = parse_map(contents);
    let end = find_char(contents, 'E');

    let mut starting_points: Vec<(isize, isize)> = vec![];
    map.iter().enumerate().for_each(|(i, list)| {
        list.iter().enumerate().for_each(|(j, height)| {
            if height == &0 {
                starting_points.push((i as isize, j as isize));
            }
        })
    });

    let mut min: i32 = i32::MAX;
    starting_points.iter().for_each(|start| {
        let cur = shortest_path(&map, *start, end);
        if cur > 0 {
            min = min.min(cur)
        }
    });
    return min;
}

pub fn run(contents: &str) {
    println!("Part 1: {}", part1(contents.clone()));
    println!("Part 2: {}", part2(contents.clone()));
}
