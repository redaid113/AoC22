fn parse_num(num: &str) -> i32 {
    return num.to_string().parse::<i32>().unwrap();
}

fn parse_line(line: &str) -> Vec<i32> {
    return line
        .replace("-", ",")
        .split(",")
        .into_iter()
        .map(|val| parse_num(val))
        .collect();
}

fn parse_input(input: Vec<&str>) -> Vec<Vec<i32>> {
    return input.into_iter().map(|line| parse_line(line)).collect();
}

fn part1(input: Vec<Vec<i32>>) -> i32 {
    return input
        .into_iter()
        .map(|values| {
            if values[0] >= values[2] && values[1] <= values[3] {
                return 1;
            }
            if values[2] >= values[0] && values[3] <= values[1] {
                return 1;
            }
            return 0;
        })
        .sum();
}

fn part2(input: Vec<Vec<i32>>) -> i32 {
    return input
        .into_iter()
        .map(|values| {
            if values[0] >= values[2] && values[0] <= values[3] {
                return 1;
            }
            if values[1] >= values[2] && values[1] <= values[3] {
                return 1;
            }
            if values[2] >= values[0] && values[2] <= values[1] {
                return 1;
            }
            if values[3] >= values[0] && values[3] <= values[1] {
                return 1;
            }
            return 0;
        })
        .sum();
}

pub fn run(contents: &str) {
    let arr: Vec<&str> = contents.trim().split("\n").collect();
    let input_array = parse_input(arr);

    println!("Part 1: {}", part1(input_array.clone()));
    println!("Part 2: {}", part2(input_array.clone()));
}
