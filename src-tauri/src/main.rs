#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod clipboard;
mod imageproc;
mod fs;

fn main() {
    tauri::Builder::default()
		.invoke_handler(tauri::generate_handler![
			clipboard::copy_image,
			fs::get_filenames,
			imageproc::resize_all,
		])
        .run(tauri::generate_context!())
        .expect("failed to run Nitrate application");
}
