#[tauri::command]
pub fn copy_image(filename: String) {
	let mut ctx = arboard::Clipboard::new().unwrap();

	let img = ::image::io::Reader::open(filename).unwrap().decode().expect("failed to decode image");
    let rgba8 = img.into_rgba8();
    let (w,h) = rgba8.dimensions();
    let bytes = rgba8.into_raw();

    let img_data = ::arboard::ImageData{
		width: w as usize,
		height: h as usize,
		bytes: bytes.into()
	};

    ctx.set_image(img_data).expect("failed to transfer images bytes to clipboard");
}
