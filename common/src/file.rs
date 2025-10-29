use crate::config::get_config;
use std::path::{Path, PathBuf};
use std::{env, fs, io};

pub fn project_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

pub fn get_data_dir(year: u32, day: u32) -> PathBuf {
    project_root().join(format!("data/{}/day_{}", year, day))
}

pub fn get_input_path(year: u32, day: u32) -> PathBuf {
    get_data_dir(year, day).join("input.txt")
}

pub async fn download_input_file(year: u32, day: u32) -> Result<(), Box<dyn std::error::Error>> {
    let dir = get_data_dir(year, day);
    let path = get_input_path(year, day);

    if fs::exists(&path).expect("unable to read file") {
        return Ok(());
    }
    let config = get_config()?;

    let client = reqwest::Client::new();
    let response = client
        .get(&format!(
            "https://adventofcode.com/{}/day/{}/input",
            year, day
        ))
        .header("Cookie", config.cookie.as_str())
        .send()
        .await?;

    let body = response.text().await?;

    if !fs::exists(&dir).expect("unable to read dir") {
        fs::create_dir_all(&dir).expect("unable to create dir");
    }

    let output_file = fs::File::create_new(&path);
    io::copy(&mut body.as_bytes(), &mut output_file.unwrap()).expect("unable to write to file");

    Ok(())
}
