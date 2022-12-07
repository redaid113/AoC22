use std::env;
use std::fs;

mod d_1;
mod d_2;
mod d_3;
mod d_4;
mod d_5;
mod d_6;
mod d_7;
mod d_8;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day: &str = &args[1];
    let file_name = &args[2];

    let file_path = &format!("./src/{day}/{file_name}");
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents = (&file_contents).trim_end();
    match day {
        "d_1" => d_1::run(&contents),
        "d_2" => d_2::run(&contents),
        "d_3" => d_3::run(&contents),
        "d_4" => d_4::run(&contents),
        "d_5" => d_5::run(&contents),
        "d_6" => d_6::run(&contents),
        "d_7" => d_7::run(&contents),
        "d_8" => d_8::run(&contents),

        _ => println!("nothing"),
    }
}
