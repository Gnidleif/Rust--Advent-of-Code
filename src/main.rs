use std::{
    result::Result,
    error::Error,
};

#[path = "2022.rs"] mod _2022;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    let run_sample = args.len() > 1 && args[1].parse::<i32>().unwrap() == 1;

    // create sample input file for each day of each year
    for y in 2015..=2022 {
        for d in 1..=25 {
            aoc_lib::create_input(y, d, true).await?;
        }
    }

    let days: Vec<Box<dyn aoc_lib::Day>> = vec![
        Box::new(_2022::day1::Day::new(run_sample).await?),
        Box::new(_2022::day2::Day::new(run_sample).await?),
    ];

    for day in days.iter() {
        println!("{}", day.fmt_result());
    }

    Ok(())
}