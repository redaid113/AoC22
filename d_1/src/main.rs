use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let arr: Vec<&str> = contents.trim().split("\n\n").collect();
    let mut calories: Vec<i32> = vec![];
    for values in arr.into_iter() {
        let mut cur = 0;
        let sub: Vec<&str> = values.split("\n").collect();

        for num in sub.into_iter() {
            cur += num.parse::<i32>().unwrap();
        }
        calories.push(cur);
    }
    calories.sort_by(|a, b| b.cmp(a));
    println!("Biggest: {}", calories[0]);
    println!("Top 3: {}", calories[0]+ calories[1]+  calories[2]);

}
