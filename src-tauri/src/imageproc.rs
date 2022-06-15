use std::path::Path;
use std::io::Cursor;
use std::fs::remove_file;
use image::io::Reader;
use crate::fs::get_filenames;

#[tauri::command]
pub fn get_image_data(filename: &str) -> String {
	let img = Reader::open(filename).unwrap().decode().expect("Failed to decode or open image");

	let mut buf: Vec<u8> = Vec::new();

	img.write_to(&mut Cursor::new(&mut buf), image::ImageOutputFormat::Png).unwrap();
	let res_base64 = base64::encode(&buf);

	format!("data:image/png;base64,{res_base64}")
}

#[tauri::command]
pub fn resize_all() {
	let resized = get_filenames(true, false);
	let emojis = get_filenames(false, false);

	// if not in emojis, delete
	for r in resized.iter() {
		if !emojis.contains(r) {
			remove_file(r).expect("failed to remove file");
		}
	}

	// if in emojis but not in resized, add
	for emoji in emojis.iter() {
		if !resized.contains(emoji) {
			println!("Does not contain {emoji}, resizing");
			// resize(emoji);
		}
	}
}

fn resize(filename: &str) {
	let img = image::open(&Path::new(filename)).unwrap();

}
