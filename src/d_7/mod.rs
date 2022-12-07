use std::collections::HashMap;

fn parse_file_system(contents: &str) -> HashMap<String, u32> {
    let mut sizes: HashMap<String, u32> = HashMap::new();

    let mut cur_dir = [].to_vec();
    let mut cur_value = 0;

    for line in contents.split("\n").into_iter() {
        let bits: Vec<&str> = line.split(' ').collect();
        match bits[0] {
            "$" => {
                if bits[1] == "cd" {
                    if bits[2] == "/" {
                        cur_dir = ["~"].to_vec();
                    } else if bits[2] == ".." {
                        sizes.insert(cur_dir.clone().join("/"), cur_value);
                        cur_dir.pop();
                        cur_value += sizes.get(&cur_dir.clone().join("/")).expect("Nope")
                    } else {
                        sizes.insert(cur_dir.clone().join("/"), cur_value);
                        cur_value = 0;
                        cur_dir.push(bits[2])
                    }
                }
            }
            "dir" => {
                continue;
            }
            _ => {
                let size: u32 = bits[0].clone().parse().expect("");
                cur_value += size;
            }
        }
    }
    while cur_dir.len() > 0 {
        sizes.insert(cur_dir.clone().join("/"), cur_value);
        cur_dir.pop();
        if cur_dir.len() > 0 {
            cur_value += sizes.get(&cur_dir.clone().join("/")).expect("Nope")
        }
    }

    return sizes;
}

fn part1(sizes: HashMap<String, u32>) -> u32 {
    return sizes.values().copied().filter(|&v| v <= 100000).sum();
}

fn part2(sizes: HashMap<String, u32>) -> u32 {
    let total_size: u32 = *sizes.get("~").expect("Bad");
    let current_size = 70000000 - total_size;
    let mut sizes_list: Vec<u32> = sizes.values().copied().collect();
    sizes_list.sort();

    for val in sizes_list {
        if current_size + val >= 30000000 {
            return val;
        }
    }
    return 0;
}

pub fn run(contents: &str) {
    let sizes = parse_file_system(contents);

    println!("Part 1: {}", part1(sizes.clone()));
    println!("Part 2: {}", part2(sizes.clone()));
}
