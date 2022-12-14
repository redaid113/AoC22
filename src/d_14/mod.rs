fn parse_point(point: &str) -> (usize, usize) {
    let arr: Vec<usize> = point.split(",").map(|num| num.parse().unwrap()).collect();
    (arr[0], arr[1])
}

fn parse_line(line: &str) -> Vec<(usize, usize)> {
    line.split(" -> ").map(parse_point).collect()
}

fn parse_input(contents: &str) -> Vec<Vec<(usize, usize)>> {
    contents.split("\n").map(parse_line).collect()
}

fn draw_line(map: &mut Vec<Vec<&str>>, start_point: (usize, usize), end_point: (usize, usize)) {
    for x in start_point.0.min(end_point.0)..=start_point.0.max(end_point.0) {
        for y in start_point.1.min(end_point.1)..=start_point.1.max(end_point.1) {
            map[x][y] = "#";
        }
    }
}

fn build_map(contents: &str) -> Vec<Vec<&str>> {
    let points_lists = parse_input(contents);
    let mut map = vec![vec!["."; 200]; 1000];

    points_lists.iter().for_each(|vec_of_points| {
        for i in 0..vec_of_points.len() - 1 {
            draw_line(&mut map, vec_of_points[i], vec_of_points[i + 1])
        }
    });
    return map;
}

fn lowest_point(map: &Vec<Vec<&str>>) -> usize {
    let mut max_y = 0;

    map.iter().for_each(|line| {
        line.iter().enumerate().for_each(|(y, value)| {
            if value != &"." {
                max_y = max_y.max(y);
            }
        })
    });
    return max_y;
}

#[allow(dead_code)]
fn print_map(map: &Vec<Vec<&str>>) {
    let mut min_x = 1000;
    let mut max_x = 0;

    map.iter().enumerate().for_each(|(x, line)| {
        line.iter().for_each(|value| {
            if value != &"." {
                min_x = min_x.min(x - 1);
                max_x = max_x.max(x + 1);
            }
        })
    });

    for y in 0..=lowest_point(&map) + 1 {
        for x in min_x..=max_x {
            print!("{}", map[x][y]);
        }
        println!("")
    }
    for _ in min_x..=max_x {
        print!("#");
    }
    println!("")
}

fn part1(contents: &str) -> i32 {
    let mut map = build_map(contents);
    let lowest_y = lowest_point(&map);
    let mut sand_count = 0;
    loop {
        let mut cur_x = 500;
        let mut cur_y = 0;
        loop {
            if cur_y >= lowest_y {
                // print_map(&map);
                return sand_count;
            }
            if map[cur_x][cur_y + 1] == "." {
                cur_y += 1;
            } else if map[cur_x - 1][cur_y + 1] == "." {
                cur_x -= 1;
                cur_y += 1;
            } else if map[cur_x + 1][cur_y + 1] == "." {
                cur_x += 1;
                cur_y += 1;
            } else {
                break;
            }
        }
        map[cur_x][cur_y] = "+";
        sand_count += 1;
    }
}

fn part2(contents: &str) -> usize {
    let mut map = build_map(contents);
    let lowest_y = lowest_point(&map);
    let mut sand_count = 0;
    loop {
        let mut cur_x = 500;
        let mut cur_y = 0;
        loop {
            if cur_y >= lowest_y + 1 {
                break;
            }
            if map[cur_x][cur_y + 1] == "." {
                cur_y += 1;
            } else if map[cur_x - 1][cur_y + 1] == "." {
                cur_x -= 1;
                cur_y += 1;
            } else if map[cur_x + 1][cur_y + 1] == "." {
                cur_x += 1;
                cur_y += 1;
            } else {
                break;
            }
        }
        map[cur_x][cur_y] = "+";
        sand_count += 1;
        if cur_x == 500 && cur_y == 0 {
            // print_map(&map);
            return sand_count;
        }
    }
}

pub fn run(contents: &str) {
    println!("Part 1: {}", part1(contents));
    println!("Part 2: {}", part2(contents));
}
