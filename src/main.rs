use image::imageops::resize;
use image::io::Reader as ImageReader;
use reqwest::{Client, Error};
use serde_json::Value;

use std::env;
use std::io::Cursor;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let curs = get_image_as_curs().await?;
    let img = ImageReader::new(curs)
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();

    let smaller = resize(
        &img,
        img.width() / 2,
        img.height() / 2,
        image::imageops::FilterType::Nearest,
    );
    smaller.save("test.jpg").unwrap();
    Ok(())
}

async fn get_image_as_curs() -> Result<Cursor<bytes::Bytes>, Error> {
    dotenv::dotenv().ok();
    let api_key =
        String::from(env::var("NASA_API_KEY").expect("NASA_API_KEY must be set in `.env` file"));
    let url = format!("https://api.nasa.gov/planetary/apod?api_key={}", api_key);
    let res = Client::new().get(url).send().await?.text().await?;
    let json: Value = serde_json::from_str(&res).unwrap();

    let imgurl = json
        .get("url")
        .expect("NASA API did not return `url`")
        .as_str()
        .unwrap();
    let img = Client::new().get(imgurl).send().await?.bytes().await?;

    Ok(Cursor::new(img))
}
