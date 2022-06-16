
/// Copies image bytes to the clipboard so they can be pasted automatically
#[tauri::command]
pub fn copy_image(filename: String) {
    let mut ctx = ::arboard::Clipboard::new().expect("failed to create Clipboard context");

    let img = ::image::io::Reader::open(filename)
        .expect("failed to open image")
        .decode()
        .expect("failed to decode image");

    let rgba8 = img.into_rgba8();
    let (w, h) = rgba8.dimensions();
    let bytes = rgba8.into_raw();

    let img_data = ::arboard::ImageData {
        width: w as usize,
        height: h as usize,
        bytes: bytes.into(),
    };

    ctx.set_image(img_data)
        .expect("failed to transfer images bytes to clipboard");
}
