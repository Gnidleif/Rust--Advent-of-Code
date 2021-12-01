use std::{
    result::Result, 
    error::Error,
    path::Path,
    fs,
};
use http::{header::{HeaderMap}, HeaderValue};

pub async fn create_input(year: u16, day: u8) -> Result<String, Box<dyn Error>> {
    let dir_path = format!("./input/{}", year);
    let file_path = format!("{}/{}.log", dir_path, day);

    if !Path::new(&file_path).is_file() {
        if !Path::new(&dir_path).is_dir() {
            fs::create_dir_all(dir_path)?;
        }

        let session = fs::read_to_string("session.log")?;
        let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
        let content = request_content(session.trim(), url).await?;
        fs::write(&file_path, &content.trim())?;
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