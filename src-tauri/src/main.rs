#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod clipboard;

fn main() {
    tauri::Builder::default()
		.invoke_handler(tauri::generate_handler![
			clipboard::copy_image,
		])
        .run(tauri::generate_context!())
        .expect("failed to run Nitrate application");
}

fn show_window() {
	//also create Clipboard ctx

}

fn hide_window() {

}
