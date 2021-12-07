use std::{
    result::Result, 
    error::Error,
    path::Path,
    fs,
};
use http::{header::{HeaderMap}, HeaderValue};

pub async fn create_input(year: u16, day: u8, run_sample: bool) -> Result<String, Box<dyn Error>> {
    let dir_path = if !run_sample { format!("./input/{}", year) } else { format!("./sample/{}", year) };
    let file_path = format!("{}/{}.log", dir_path, day);

    if !Path::new(&file_path).is_file() {
        if !Path::new(&dir_path).is_dir() {
            fs::create_dir_all(dir_path)?;
        }

        if !run_sample {
            let session = fs::read_to_string("session.log")?;
            let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
            let content = request_content(session.trim(), url).await?;
            fs::write(&file_path, &content.trim())?;
        }
    }

    Ok(fs::read_to_string(file_path)?.parse()?)
}

async fn request_content(session: &str, url: String) -> Result<String, Box<dyn Error>> {
    let cookie = format!("session={}", session);
    let mut request_headers = HeaderMap::new();
    request_headers.insert(
        http::header::COOKIE,
        HeaderValue::from_str(&cookie)?,
    );

    let client = reqwest::Client::builder()
        .default_headers(request_headers)
        .build()?
        .get(url)
        .send();

    match client.await {
        Ok(resp) => {
            match resp.status() {
                reqwest::StatusCode::OK => Ok(resp.text().await?),
                _ => Err(format!("Request error: {:?}", resp).into())
            }
        },
        Err(err) => Err(err.into())
    }
}

// Day definitions

#[macro_export]
macro_rules! aw {
    ($e:expr) => {
        tokio_test::block_on($e)
    };
}

pub trait Day {
    fn part1(&self) -> usize;
    fn part2(&self) -> usize;
    fn fmt_result(&self) -> String;
}

// Range

use std::{
    ops::RangeInclusive,
    iter::Rev,
};

pub enum Range {
    Forward(RangeInclusive<usize>),
    Backwards(Rev<RangeInclusive<usize>>),
}

impl Range {
    pub fn from(start: usize, end: usize) -> Self {
        if start < end {
            Range::Forward(start..=end)
        }
        else {
            Range::Backwards((end..=start).rev())
        }
    }
}

impl Iterator for Range {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        match self {
            Range::Forward(range) => range.next(),
            Range::Backwards(range) => range.next(),
        }
    }
}

// Points

#[derive(Clone, Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

use itertools::{
    Itertools,
    EitherOrBoth::{Both, Left, Right},
};

pub fn line_between_points<'a>(p1: &'a Point, p2: &'a Point, w: &'a usize) -> Box<dyn Iterator<Item=usize> + 'a> {
    Box::new(Range::from(p1.x, p2.x).zip_longest(Range::from(p1.y, p2.y))
        .map(|i| match i {
            Both(x, y) => (x, y),
            Left(x) => (x, p1.y),
            Right(y) => (p1.x, y),
        }).map(move |(x, y)| (y * w) + x))
}

pub fn index_from_point(p: &Point, w: &usize) -> usize {
    (p.y * w) + p.x
}