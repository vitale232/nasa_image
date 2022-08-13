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
    let url = format!("https://api.nasa.gov/planetary/apod?api_key={}", api_key);
    let curs = get_image_as_curs(&url).await?;

    let img = ImageReader::new(curs)
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();

    let smaller = resize_img(img);
    smaller.save("test.jpg").unwrap();
    Ok(())
}

fn resize_img(img: DynamicImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let h = img.height();
    let w = img.width();

    if h > w {
        rotate90(&resize(&img, 135, 240, FilterType::Gaussian))
    } else {
        resize(&img, 240, 130, FilterType::Gaussian)
    }
}

async fn get_image_as_curs(url: &str) -> Result<Cursor<bytes::Bytes>, Error> {
    let res = Client::new().get(url).send().await?.text().await?;
    let json: Value = serde_json::from_str(&res).unwrap();

    let img_url = json
        .get("url")
        .expect("NASA API did not return `url`")
        .as_str()
        .unwrap();
    let img = Client::new().get(img_url).send().await?.bytes().await?;

    Ok(Cursor::new(img))
}
