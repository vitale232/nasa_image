use image::imageops::{resize, rotate90, FilterType};
use image::io::Reader as ImageReader;
use image::{DynamicImage, ImageBuffer, Rgba};
use reqwest::{Client, Error};
use serde_json::Value;

use std::env;
use std::io::Cursor;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();

    let api_key =
        String::from(env::var("NASA_API_KEY").expect("NASA_API_KEY must be set in `.env` file"));
    let url = format!(
        "https://api.nasa.gov/planetary/apod?thumbs=true&api_key={}",
        api_key
    );

    let imgcurs = get_imgcurs(&url).await?;
    let img = ImageReader::new(imgcurs)
        .with_guessed_format()
        .expect("Could not guess img format")
        .decode()
        .expect("Could not decode img");

    let smaller = resize_img(img);
    smaller.save("test.jpg").unwrap();

    Ok(())
}

async fn get_imgcurs(url: &str) -> Result<Cursor<bytes::Bytes>, Error> {
    let client = Client::new();

    let res = client.get(url).send().await?.text().await?;
    let json: Value = serde_json::from_str(&res).unwrap();

    let img_url = match json.get("thumbnail_url") {
        Some(url) => url.as_str().unwrap(),
        None => json
            .get("url")
            .expect("NASA API did not return `url`")
            .as_str()
            .unwrap(),
    };

    let img = client.get(img_url).send().await?.bytes().await?;
    Ok(Cursor::new(img))
}

fn resize_img(img: DynamicImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    if img.height() > img.width() {
        rotate90(&resize(&img, 135, 240, FilterType::Gaussian))
    } else {
        resize(&img, 240, 130, FilterType::Gaussian)
    }
}
