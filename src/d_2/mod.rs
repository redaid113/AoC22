fn score1(a: char, b: char) -> i32 {
    let aa = a as i32 - 'A' as i32;
    let bb = b as i32 - 'X' as i32;

    let you_choose = bb + 1;
    let win_lose = ((bb-aa+4)%3)*3;

    return you_choose + win_lose;
}

fn score2(a: char, b: char) -> i32 {
    let aa = a as i32 - 'A' as i32;
    let bb = b as i32 - 'X' as i32;

    let you_choose =( aa + bb+2) % 3 +1 ;
    let win_lose = bb*3;

    return you_choose + win_lose;
}

pub fn run(contents: &str) {
    let arr: Vec<&str> = contents.trim().split("\n").collect();
    let arr2 = arr.clone();

    let sum1: i32 = arr
    .into_iter()
    .map(|group: &str| score1(group.chars().nth(0).unwrap(), group.chars().nth(2).unwrap()))
    .sum();

    let sum2: i32 = arr2
    .into_iter()
    .map(|group: &str| score2(group.chars().nth(0).unwrap(), group.chars().nth(2).unwrap()))
    .sum();

    println!("Part 1: {}", sum1);
    println!("Part 2: {}", sum2);
}
