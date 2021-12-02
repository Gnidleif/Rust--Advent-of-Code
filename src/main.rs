use std::{
    result::Result,
    error::Error
};

#[path = "2021/day1.rs"] mod day1_2021;
#[path = "2021/day2.rs"] mod day2_2021;
use day1_2021::Day1;
use day2_2021::Day2;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let days: Vec<Box<dyn aoc_lib::Day>> = vec![
        Box::new(Day1::new().await?),
        Box::new(Day2::new().await?),
    ];

    for (i, day) in days.iter().enumerate() {
        println!("Day #{}: ({}, {})", i+1, day.part1(), day.part2());
    }

    Ok(())
}