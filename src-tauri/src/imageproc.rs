use std::fs::remove_file;
use crate::fs::get_filenames;

#[tauri::command]
pub fn resize_all() {
	let resized = get_filenames(true, true);
	let emojis = get_filenames(false, true);

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
			resize(emoji);
		}
	}
}

fn resize(filename: &str) {

}
