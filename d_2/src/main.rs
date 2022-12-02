use std::env;
use std::fs;



fn score1(a: char, b: char) -> i32 {
    let A = a as i32 - 'A' as i32;
    let B = b as i32 - 'X' as i32;

    let you_choose = B + 1;
    let win_lose = ((B-A+4)%3)*3;

    return you_choose + win_lose;
}

fn score2(a: char, b: char) -> i32 {
    let A = a as i32 - 'A' as i32;
    let B = b as i32 - 'X' as i32;

    let you_choose =( A + B+2) % 3 +1 ;
    let win_lose = B*3;

    return you_choose + win_lose;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
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
