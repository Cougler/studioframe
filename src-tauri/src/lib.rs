use std::process::Command;
use tauri::command;

#[command]
fn convert_to_mp4(input_path: String, output_path: String) -> Result<String, String> {
    let result = Command::new("ffmpeg")
        .args([
            "-i", &input_path,
            "-c:v", "libx264",
            "-preset", "fast",
            "-crf", "23",
            "-c:a", "aac",
            "-b:a", "128k",
            "-movflags", "+faststart",
            "-y",
            &output_path,
        ])
        .output()
        .map_err(|e| format!("Failed to run ffmpeg: {}. Make sure ffmpeg is installed.", e))?;

    if result.status.success() {
        Ok(output_path)
    } else {
        let stderr = String::from_utf8_lossy(&result.stderr);
        Err(format!("ffmpeg error: {}", stderr))
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![convert_to_mp4])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
