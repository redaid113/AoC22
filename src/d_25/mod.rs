fn parse_input(contents: &str) -> Vec<&str> {
    contents.split("\n").collect()
}

fn snafu_to_decimal(snafu: &str) -> i64 {
    let mut decimal: i64 = 0;
    let temp = snafu.clone();
    temp.chars().rev().enumerate().for_each(|(i, c)| {
        let val: i64;
        if c == '=' {
            val = -2;
        } else if c == '-' {
            val = -1;
        } else {
            val = c.to_digit(10).unwrap() as i64;
        }
        decimal += val * 5_i64.pow(i as u32)
    });
    decimal
}

fn decimal_to_snafu(decimal: i64) -> String {
    let mut snafu: Vec<String> = vec![];
    let mut left = decimal;
    while left > 0 {
        let rem = left % 5;
        if rem == 3 {
            snafu.push("=".to_string());
            left += 2;
        } else if rem == 4 {
            snafu.push("-".to_string());
            left += 1;
        } else {
            snafu.push(rem.to_string());
            left -= rem;
        }
        left /= 5;
    }
    snafu.reverse();
    snafu.join("")
}

fn part1(contents: &str) -> String {
    let total = parse_input(contents)
        .iter()
        .map(|snafu| snafu_to_decimal(snafu))
        .sum();
    decimal_to_snafu(total)
}

pub fn run(contents: &str) {
    println!("Part 1: {}", part1(contents));
}
