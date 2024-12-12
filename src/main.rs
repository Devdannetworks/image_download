use anyhow::Result;
use dirs_next::download_dir;
use std::env;
use std::fs;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    let download_dir = download_dir().expect("Error getting the download directory");
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("No image URL provided.");
        return Ok(());
    }

    let url = &args[1];
    let raw_image_name = url.split('/').last().unwrap_or("downloaded_image");
    let mut image_name: &str =
        &raw_image_name.replace(|c: char| !c.is_alphanumeric() && c != '.', "_");
    if image_name.len() > 100 {
        image_name = &image_name[..100];
    }

    let response = reqwest::get(url).await?;
    let image_bytes = response.bytes().await?;
    let file_path = download_dir.join(image_name);

    match fs::write(&file_path, &image_bytes) {
        Ok(_) => {
            println!("Image downloaded to: {:?}", file_path.display());
        }
        Err(e) => {
            eprintln!("Error saving the image: {:?}", e);
        }
    }

    Ok(())
}
