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
mod d_9;

mod d_10;
mod d_11;
mod d_12;
mod d_13;
mod d_14;
mod d_15;
mod d_16;
mod d_17;
mod d_18;
mod d_19;
mod d_20;
mod d_21;
mod d_22;

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
        "d_9" => d_9::run(&contents),
        "d_10" => d_10::run(&contents),
        "d_11" => d_11::run(&contents),
        "d_12" => d_12::run(&contents),
        "d_13" => d_13::run(&contents),
        "d_14" => d_14::run(&contents),
        "d_15" => d_15::run(&contents),
        "d_16" => d_16::run(&contents),
        "d_17" => d_17::run(&contents),
        "d_18" => d_18::run(&contents),
        "d_19" => d_19::run(&contents),
        "d_20" => d_20::run(&contents),
        "d_21" => d_21::run(&contents),
        "d_22" => d_22::run(&contents),

        _ => println!("nothing"),
    }
}
