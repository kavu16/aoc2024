use std::fs;

mod puzzles;

use crate::puzzles::*;

// use std::env;

fn main() {
    // env::set_var("RUST_BACKTRACE", "1");
    let args = std::env::args().collect::<Vec<String>>();
    match args.len() {
        1 => {panic!("Not enough args!")},
        _ => {
            let day = args[1].as_str();
            let data = fs::read_to_string(format!("data/{}.txt", day)).unwrap();
            match day {
                "day1" => {
                    day1::solve1(&data);
                    day1::solve2(&data);
                }
                "day2" => {
                    day2::solve1(&data);
                    day2::solve2(&data);
                }
                "day3" => {
                    day3::solve1(&data);
                    day3::solve2(&data);
                }
                "day4" => {
                    day4::solve1(&data);
                    day4::solve2(&data);
                }
                "day5" => {
                    day5::solve1(&data);
                    day5::solve2(&data);
                }
                "day6" => {
                    day6::solve1(&data);
                    day6::solve2(&data);
                }
                _ => {
                    panic!("Invalid Day!");
                }
            }
        }
    }
}