use std::{
    result::Result,
    error::Error
};

#[path = "days_2021.rs"] mod days_2021;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let days: Vec<Box<dyn days_2021::Day>> = vec![
        Box::new(days_2021::Day1::new().await?),
        Box::new(days_2021::Day2::new().await?),
    ];

    for day in days.iter() {
        println!("{}", day.part1());
        println!("{}", day.part2());
    }

    Ok(())
}

// #[cfg(test)]
// mod main_testing {
//     use super::days_2021;
//     use days_2021::Day;

//     #[test]
//     fn d1p1_old() {
//         let d = days_2021::Day1::new().unwrap();
//         assert_eq!(d.part1_old(), 1791);
//     }

//     #[test]
//     fn d1p2_old() {
//         let d = days_2021::Day1::new().unwrap();
//         assert_eq!(d.part2_old(), 1822);
//     }

//     #[test]
//     fn d1p1() {
//         let d = days_2021::Day1::new().unwrap();
//         assert_eq!(d.part1(), 1791);
//     }

//     #[test]
//     fn d1p2() {
//         let d = days_2021::Day1::new().unwrap();
//         assert_eq!(d.part2(), 1822);
//     }
// }