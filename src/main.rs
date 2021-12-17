use std::{
    result::Result,
    error::Error,
};

#[path = "2015.rs"] mod _2015;
#[path = "2020.rs"] mod _2020;
#[path = "2021.rs"] mod _2021;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    let run_sample = args.len() > 1 && args[1].parse::<i32>().unwrap() == 1;

    // create sample input file for each day of each year
    for y in 2015..=2021 {
        for d in 1..=25 {
            aoc_lib::create_input(y, d, true).await?;
        }
    }

    let days: Vec<Box<dyn aoc_lib::Day>> = vec![
        // Box::new(_2015::day1::Day::new(run_sample).await?),
        // Box::new(_2015::day2::Day::new(run_sample).await?),
        // Box::new(_2015::day3::Day::new(run_sample).await?),
        // Box::new(_2015::day4::Day::new(run_sample).await?),
        // Box::new(_2015::day5::Day::new(run_sample).await?),
        // Box::new(_2015::day6::Day::new(run_sample).await?),
        // Box::new(_2015::day7::Day::new(run_sample).await?),

        // Box::new(_2020::day11::Day::new(run_sample).await?),

        // Box::new(_2021::day1::Day::new(run_sample).await?),
        // Box::new(_2021::day2::Day::new(run_sample).await?),
        // Box::new(_2021::day3::Day::new(run_sample).await?),
        // Box::new(_2021::day4::Day::new(run_sample).await?),
        // Box::new(_2021::day5::Day::new(run_sample).await?),
        // Box::new(_2021::day6::Day::new(run_sample).await?),
        // Box::new(_2021::day7::Day::new(run_sample).await?),
        // Box::new(_2021::day8::Day::new(run_sample).await?),
        // Box::new(_2021::day9::Day::new(run_sample).await?),
        // Box::new(_2021::day10::Day::new(run_sample).await?),
        // Box::new(_2021::day11::Day::new(run_sample).await?),
        // Box::new(_2021::day12::Day::new(run_sample).await?),
        // Box::new(_2021::day13::Day::new(run_sample).await?),
        Box::new(_2021::day14::Day::new(run_sample).await?),
    ];

    for day in days.iter() {
        println!("{}", day.fmt_result());
    }

    Ok(())
}