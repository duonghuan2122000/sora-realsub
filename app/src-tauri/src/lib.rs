#[allow(dead_code)] // PBI 3: chỉ định nghĩa interface, chưa có cài đặt thật dùng tới
mod core;

use core::{AudioDeviceInfo, CpalAudioHost, list_input_devices};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn list_input_devices_cmd() -> Result<Vec<AudioDeviceInfo>, String> {
    list_input_devices(&CpalAudioHost).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, list_input_devices_cmd])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greet_returns_greeting_with_name() {
        assert_eq!(greet("Sora"), "Hello, Sora! You've been greeted from Rust!");
    }
}
