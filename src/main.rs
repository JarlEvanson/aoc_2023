#![feature(iter_array_chunks, get_many_mut)]

use std::{fmt::Write, path::PathBuf, time::Duration};

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_20;
mod day_21;
mod day_22;
mod day_23;
mod day_24;
mod day_25;
#[allow(unused)]
mod util;

const FILL_IN: Option<String> = None;

#[allow(clippy::type_complexity)]
const LOOKUP: [fn(&str) -> (usize, usize); 25] = [
    day_01::solve,
    day_02::solve,
    day_03::solve,
    day_04::solve,
    day_05::solve,
    day_06::solve,
    day_07::solve,
    day_08::solve,
    day_09::solve,
    day_10::solve,
    day_11::solve,
    day_12::solve,
    day_13::solve,
    day_14::solve,
    day_15::solve,
    day_16::solve,
    day_17::solve,
    day_18::solve,
    day_19::solve,
    day_20::solve,
    day_21::solve,
    day_22::solve,
    day_23::solve,
    day_24::solve,
    day_25::solve,
];

fn main() {
    let Some(mut folder) = std::env::args().nth(1).map(PathBuf::from) else {
        println!("a folder must be specified to load inputs from");
        println!("each day's input file must be of the form 'day_[day].txt");
        return;
    };

    folder.push("day_00.txt");

    let mut inputs = [FILL_IN; 25];

    let mut input_filename = String::with_capacity(12);

    for (day, input) in inputs.iter_mut().enumerate() {
        input_filename.clear();
        write!(&mut input_filename, "day_{:02}.txt", day + 1).unwrap();

        folder.set_file_name(&input_filename);

        *input = std::fs::read_to_string(&folder).ok();
    }

    let mut total_time = Duration::new(0, 0);

    for (day, solution) in LOOKUP.iter().enumerate() {
        let Some(input) = &inputs[day] else {
            println!("Skipping day {:02}", day + 1);
            continue;
        };

        let start = std::time::Instant::now();

        let (part_1, part_2) = solution(&input);

        let end = std::time::Instant::now();

        let elapsed = end.duration_since(start);

        total_time += elapsed;

        println!(
            "Day {:02}: ({:15}, {:15}); Elapsed: {:13?}: Total Time: {:13?}",
            day + 1,
            part_1,
            part_2,
            elapsed,
            total_time
        );
    }

    println!("Total Processing Time: {:?}", total_time);
}
