use std::{
    result::Result,
    error::Error
};

#[path = "./days_2021.rs"] mod days_2021;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let days: Vec<Box<dyn aoc_input::Day>> = vec![
        Box::new(days_2021::Day1::new().await?),
    ];

    for day in days.iter() {
        println!("{}", day.part1());
        println!("{}", day.part2());
    }

    Ok(())
}