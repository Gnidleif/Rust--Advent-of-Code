use std::{
    result::Result, 
    error::Error,
    path::Path,
};
use http::{header::{HeaderMap}, HeaderValue};

pub trait Day {
    fn part1(&self) -> i64;
    fn part2(&self) -> i64;
}

pub async fn create_input(year: u16, day: u8) -> Result<String, Box<dyn Error>> {
    let dir_path = format!("./input/{}", year);
    let file_path = format!("{}/{}.log", dir_path, day);

    if !Path::new(&file_path).is_file() {
        if !Path::new(&dir_path).is_dir() {
            std::fs::create_dir_all(dir_path)?;
        }

        let session = std::fs::read_to_string("session.log")?.trim().to_string();
        let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
        let content = request_content(session, url).await?;
        std::fs::write(&file_path, &content.trim())?;
    }

    Ok(std::fs::read_to_string(file_path)?.parse()?)
}

async fn request_content(session: String, url: String) -> Result<String, Box<dyn Error>> {
    let cookie = format!("session={}", session);
    let mut request_headers = HeaderMap::new();
    request_headers.insert(
        http::header::COOKIE,
        HeaderValue::from_str(&cookie[..])?,
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