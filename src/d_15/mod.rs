use std::cmp::Ordering;

#[derive(Debug)]
struct Beacon {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Sensor {
    x: i64,
    y: i64,
    distance: i64,
    beacon: Beacon,
}
#[derive(Debug, PartialEq, Eq, Clone)]
struct Range {
    x1: i64,
    x2: i64,
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

fn parse_x_and_y(line: &str) -> Vec<i64> {
    line.split(", y=").map(|num| num.parse().unwrap()).collect()
}

fn parse_sensor(line: &str) -> Sensor {
    let remove_head = line.replace("Sensor at x=", "");
    let bits: Vec<&str> = remove_head.split(": closest beacon is at x=").collect();

    let sensor_xy = parse_x_and_y(bits[0]);
    let beacon_xy = parse_x_and_y(bits[1]);
    let distance =
        sensor_xy[0].abs_diff(beacon_xy[0]) as i64 + sensor_xy[1].abs_diff(beacon_xy[1]) as i64;

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

fn cover_spots(sensors: &Vec<Sensor>, y: i64, min_x: i64, max_x: i64) -> Vec<Range> {
    let mut ranges: Vec<Range> = sensors
        .iter()
        .map(|sensor| {
            let difference = sensor.distance - sensor.y.abs_diff(y) as i64;
            if difference < 0 {
                return Range { x1: 0, x2: 0 };
            }
            Range {
                x1: (sensor.x - difference).max(min_x),
                x2: (sensor.x + difference).min(max_x),
            }
        })
        .filter(|range| range.x1 != range.x2)
        .collect();
    ranges.sort();
    if ranges.len() == 0 {
        return vec![];
    };

    let mut output: Vec<Range> = vec![ranges[0].clone()];
    for i in 1..ranges.len() {
        let last_range = output.pop().unwrap();
        if last_range.x2 < ranges[i].x1 {
            output.push(last_range);
            output.push(ranges[i].clone());
        } else {
            output.push(Range {
                x1: last_range.x1,
                x2: ranges[i].x2.max(last_range.x2),
            });
        }
    }

    return output;
}

// static HEIGHT: i64 = 10;
static HEIGHT: i64 = 2000000;
fn part1(contents: &str) -> i64 {
    let sensors = parse_input(contents);
    let ranges = cover_spots(&sensors, HEIGHT, i64::MIN, i64::MAX);

    let mut beacons_x: Vec<i64> = sensors
        .iter()
        .map(|sensor| &sensor.beacon)
        .filter(|beacon| beacon.y == HEIGHT)
        .map(|beacon| beacon.x)
        .collect();
    beacons_x.dedup();

    ranges
        .iter()
        .map(|range| {
            let total = range.x2.abs_diff(range.x1) as i64 + 1;
            let found: Vec<&i64> = beacons_x
                .iter()
                .filter(|x| **x >= range.x1 && **x <= range.x2)
                .collect();
            return total - found.len() as i64;
        })
        .sum()
}

static MIN_X: i64 = 0;
// static MAX_X: i64 = 20;
static MAX_X: i64 = 4000000;
fn part2(contents: &str) -> i64 {
    let sensors = parse_input(contents);
    for y in 0..=MAX_X {
        let ranges = cover_spots(&sensors, y, MIN_X, MAX_X);
        if ranges.len() > 1 || ranges[0].x2 < MAX_X {
            return (ranges[0].x2 + 1) * 4000000 + y;
        } else if ranges[0].x1 == 1 {
            return y;
        }
    }
    return 0;
}

pub fn run(contents: &str) {
    println!("Part 1: {}", part1(contents));
    println!("Part 2: {}", part2(contents));
}
