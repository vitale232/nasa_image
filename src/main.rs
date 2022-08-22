use chrono::prelude::*;
use image::imageops::{resize, rotate90, FilterType};
use image::io::Reader as ImageReader;
use image::{DynamicImage, ImageBuffer, Rgba};
use reqwest::{Client, Error};
use serde_json::Value;

use std::env;
use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    let api_key = env::var("NASA_API_KEY").expect("NASA_API_KEY must be set in `.env` file");

    let imgcurs = get_imgcurs(&format!(
        "https://api.nasa.gov/planetary/apod?thumbs=true&api_key={}",
        api_key
    ))
    .await?;
    let img = ImageReader::new(imgcurs)
        .with_guessed_format()
        .expect("Could not guess img format")
        .decode()
        .expect("Could not decode img");
    save_dyn_image(&img);

    let smaller = resize_img(&img);
    save_img_buf(smaller);

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

fn resize_img(img: &DynamicImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    if img.height() > img.width() {
        rotate90(&resize(img, 135, 240, FilterType::Gaussian))
    } else {
        resize(img, 240, 130, FilterType::Gaussian)
    }
}

fn path_to_fname(path: &Path) -> PathBuf {
    fs::create_dir_all(path).expect("Could not make dir \"./imgs\"");
    let now: DateTime<Local> = Local::now();
    let fname = format!("{}.jpg", now.format("%Y%m%d"));
    path.join(fname)
}

fn save_dyn_image(img: &DynamicImage) {
    let path = Path::new("./imgs/raw");
    let fname = path_to_fname(path);
    img.save(fname).expect("Could not save img!")
}

fn save_img_buf(img: ImageBuffer<Rgba<u8>, Vec<u8>>) {
    let path = Path::new("./imgs/thumb");
    let fname = path_to_fname(path);
    img.save(fname).expect("Could not save img!");
}
