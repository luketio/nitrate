use std::fs::remove_file;
use std::io::Cursor;
use tauri::api::path::data_dir;
use image::imageops;
use image::io::Reader;

use crate::fs::get_filenames;

#[tauri::command]
pub fn get_image_data(filename: &str) -> String {
    let img = Reader::open(filename)
        .unwrap()
        .decode()
        .expect("Failed to decode or open image");

    let mut buf: Vec<u8> = Vec::new();

    img.write_to(&mut Cursor::new(&mut buf), image::ImageOutputFormat::Png)
        .unwrap();
    let res_base64 = base64::encode(&buf);

    format!("data:image/png;base64,{res_base64}")
}

#[tauri::command]
pub fn resize_all() {
    let resized = get_filenames(true, false);
    let emojis = get_filenames(false, false);

    // if not in emojis, delete
	if let Some(data_dir) = data_dir() {
		for r in &resized {
			if !emojis.contains(r) {
				remove_file(data_dir.join("Nitrate").join("resized").join(r)).expect("failed to remove file");
			}
		}
	}

    // if in emojis but not in resized, add
    for emoji in &emojis {
        if !resized.contains(emoji) {
            println!("Does not contain {emoji}, resizing");
            resize(emoji);
        }
    }
}

fn resize(filename: &str) {
    if let Some(data_dir) = data_dir() {
        let fin = data_dir.join("Nitrate").join("emojis").join(filename);
        let fout = data_dir.join("Nitrate").join("resized").join(filename);

        let img = image::open(&fin).expect("Failed to open filename");

        imageops::resize(&img, 64, 64, image::imageops::FilterType::Lanczos3)
            .save(fout)
            .expect("Failed to save resized image");
    }
}
