use crate::bin::day_1::Day1;
use crate::day_2::Day2;
use common::base_day::BaseDay;
use crate::day_3::Day3;
use crate::day_4::Day4;
use crate::day_5::Day5;
use crate::day_6::Day6;
use crate::day_7::Day7;
use crate::day_8::Day8;
use crate::day_9::Day9;
use crate::day_10::Day10;
use crate::day_11::Day11;
use crate::day_12::Day12;
use crate::day_13::Day13;
use crate::day_14::Day14;
use crate::day_15::Day15;
use crate::day_16::Day16;
use crate::day_17::Day17;
use crate::day_18::Day18;
use crate::day_19::Day19;
use crate::day_20::Day20;
use crate::bin::day_21::Day21;
use crate::bin::day_22::Day22;
use bin::day_23::Day23;
use bin::day_24::Day24;
use bin::day_25::Day25;

use env_logger::Env;
use log::info;
use clap::Parser;

mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
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
mod bin;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Days to include in the run
    #[arg(short, long)]
    day: Option<u32>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let include_days = if let Some(day_to_run) = args.day {
        vec![day_to_run]
    } else {
        vec![1]
    };

    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "info")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    let days: Vec<Box<dyn BaseDay>> = vec![
        Box::new(Day1::new()),
        Box::new(Day2::new()),
        Box::new(Day3::new()),
        Box::new(Day4::new()),
        Box::new(Day5::new()),
        Box::new(Day6::new()),
        Box::new(Day7::new()),
        Box::new(Day8::new()),
        Box::new(Day9::new()),
        Box::new(Day10::new()),
        Box::new(Day11::new()),
        Box::new(Day12::new()),
        Box::new(Day13::new()),
        Box::new(Day14::new()),
        Box::new(Day15::new()),
        Box::new(Day16::new()),
        Box::new(Day17::new()),
        Box::new(Day18::new()),
        Box::new(Day19::new()),
        Box::new(Day20::new()),
        Box::new(Day21::new()),
        Box::new(Day22::new()),
        Box::new(Day23::new()),
        Box::new(Day24::new()),
        Box::new(Day25::new()),
    ];

    for mut day in days {
        let day_number = day.get_day_number();
        if include_days.is_empty() || include_days.contains(&day_number) {
            common::file::download_input_file(2024, day_number).await?;

            let result = day.run_day()?;
            info!("Day {} - part 1: {:?}", result.day, result.part_1);
            info!("Day {} - part 2: {:?}", result.day, result.part_2);
        }
    }

    Ok(())
}
