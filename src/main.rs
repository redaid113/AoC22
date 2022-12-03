use std::env;
use std::fs;

mod d_1;
mod d_2;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day: &str = &args[1];
    let file_name = &args[2];

    let file_path = &format!("./src/{day}/{file_name}");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    match day {
        "d_1" => d_1::run(&contents),
        "d_2" => d_2::run(&contents),
        _ => println!("nothing"),
    }
}
