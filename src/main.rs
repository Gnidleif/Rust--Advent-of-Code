use std::{
    result::Result,
    error::Error
};

#[path = "2015/2015.rs"] mod _2015;
#[path = "2021/2021.rs"] mod _2021;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let days: Vec<Box<dyn aoc_lib::Day>> = vec![
        // Box::new(_2015::day1::Day::new().await?),
        // Box::new(_2015::day2::Day::new().await?),
        // Box::new(_2015::day3::Day::new().await?),
        // Box::new(_2015::day4::Day::new().await?),
        // Box::new(_2015::day5::Day::new().await?),
        // Box::new(_2015::day6::Day::new().await?),
        // Box::new(_2015::day7::Day::new().await?),

        Box::new(_2021::day1::Day::new().await?),
        Box::new(_2021::day2::Day::new().await?),
        Box::new(_2021::day3::Day::new().await?),
        Box::new(_2021::day4::Day::new().await?),
    ];

    for day in days.iter() {
        println!("{}", day.fmt_result());
    }

    Ok(())
}