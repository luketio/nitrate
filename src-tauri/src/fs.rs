use tauri::api::path::data_dir;

#[tauri::command]
pub fn get_filenames(resized: bool, absolute: bool) -> Vec<String> {
    if let Some(data_dir) = data_dir() {
        let dir = data_dir.join("Nitrate");
        let dir = if resized {
            dir.join("resized")
        } else {
            dir.join("emojis")
        };

        if dir.exists() {
            let mut files = Vec::new();
            for entry in dir.read_dir().expect("failed to read directory") {
                let entry = entry.expect("failed to read entry");
                let path = entry.path();
                if path.is_file() {
                    if absolute {
                        files.push(path.to_str().unwrap().to_string());
                    } else {
                        files.push(entry.file_name().to_string_lossy().to_string());
                    }
                }
            }
            files
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    }
}
