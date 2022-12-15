use std::cmp::Ordering;
use std::collections::HashSet;

#[derive(Debug)]
struct Beacon {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Sensor {
    x: i32,
    y: i32,
    distance: i32,
    beacon: Beacon,
}
#[derive(PartialEq, Eq)]
struct Range {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl PartialOrd for Range {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl Ord for Range {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.x1.cmp(&rhs.x1)
    }
}

fn parse_x_and_y(line: &str) -> Vec<i32> {
    line.split(", y=").map(|num| num.parse().unwrap()).collect()
}

fn parse_sensor(line: &str) -> Sensor {
    let remove_head = line.replace("Sensor at x=", "");
    let bits: Vec<&str> = remove_head.split(": closest beacon is at x=").collect();

    let sensor_xy = parse_x_and_y(bits[0]);
    let beacon_xy = parse_x_and_y(bits[1]);
    let distance =
        sensor_xy[0].abs_diff(beacon_xy[0]) as i32 + sensor_xy[1].abs_diff(beacon_xy[1]) as i32;

    let beacon = Beacon {
        x: beacon_xy[0],
        y: beacon_xy[1],
    };

    Sensor {
        x: sensor_xy[0],
        y: sensor_xy[1],
        distance,
        beacon,
    }
}

fn parse_input(contents: &str) -> Vec<Sensor> {
    contents.split("\n").map(parse_sensor).collect()
}

fn cover_spots(sensors: &Vec<Sensor>, y: i32, min_x: i32, max_x: i32) -> i32 {
    let mut count = 0;
    let mut seen: HashSet<i32> = HashSet::new();

    sensors
        .iter()
        .map(|sensor| &sensor.beacon)
        .for_each(|beacon| {
            if beacon.y == HEIGHT {
                seen.insert(beacon.x);
            }
        });

    sensors.iter().for_each(|sensor| {
        let difference = sensor.distance - sensor.y.abs_diff(HEIGHT) as i32;
        if difference < 0 {
            return;
        }
        for x in sensor.x - difference..=sensor.x + difference {
            if !seen.contains(&x) {
                seen.insert(x);
                count += 1;
            }
        }
    });
    return count;
}

static HEIGHT: i32 = 10;
fn part1(contents: &str) -> i32 {
    let sensors = parse_input(contents);
    cover_spots(&sensors, HEIGHT, i32::MIN, i32::MAX)
}

fn part2(contents: &str) -> i32 {
    let sensors = parse_input(contents);
    let mut ct = 0;
    for x in 0..=4000000 {
        let diff = sensors[x % sensors.len()]
            .x
            .abs_diff(sensors[x % sensors.len()].distance);
        if diff % 2 == 0 {
            ct += 1;
        }
    }
    return ct;
}

pub fn run(contents: &str) {
    println!("Part 1: {}", part1(contents));
    println!("Part 2: {}", part2(contents));
}
