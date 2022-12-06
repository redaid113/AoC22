fn part1(line: &str) -> i32 {
    let bytes = line.as_bytes();
    for i in 0..line.len() - 3 {
        if bytes[i] == bytes[i + 1] || bytes[i] == bytes[i + 2] || bytes[i] == bytes[i + 3] {
            continue;
        }
        if bytes[i + 1] == bytes[i + 2] || bytes[i + 1] == bytes[i + 3] {
            continue;
        }
        if bytes[i + 2] == bytes[i + 3] {
            continue;
        }
        return i as i32 + 4;
    }
    return 0;
}

fn part2(line: &str) -> i32 {
    let bytes = line.as_bytes();
    for i in 0..line.len() - 13 {
        let mut bad = false;
        for j in i..i + 13 {
            for k in j + 1..i + 14 {
                if bytes[j] == bytes[k] {
                    bad = true;
                    break;
                }
            }
            if bad {
                break;
            }
        }
        if !bad {
            return i as i32 + 14;
        }
    }
    return 0;
}

pub fn run(contents: &str) {
    println!("Part 1: {}", part1(contents));
    println!("Part 2: {}", part2(contents));
}
