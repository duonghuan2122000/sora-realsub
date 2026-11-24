#[allow(dead_code)] // PBI 3: chỉ định nghĩa interface, chưa có cài đặt thật dùng tới
pub mod core;

use core::{
    AudioDeviceInfo, CaptureSessionState, CpalAudioHost, LoopbackAudioSource, MicAudioSource,
    MixedAudioSource, VadPipeline, list_input_devices,
};
use serde::Serialize;
use tauri::{AppHandle, Emitter, State};

/// Payload sự kiện Tauri `vad-segment` phát ra UI (camelCase, xem contracts/vad-segment-event.md PBI 11).
#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct VadSegmentPayload {
    start_frame_index: usize,
    end_frame_index: usize,
    samples: Vec<f32>,
    sample_rate: u32,
}

impl From<core::SegmentReady> for VadSegmentPayload {
    fn from(segment: core::SegmentReady) -> Self {
        Self {
            start_frame_index: segment.start_frame_index,
            end_frame_index: segment.end_frame_index,
            samples: segment.samples,
            sample_rate: segment.sample_rate,
        }
    }
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn list_input_devices_cmd() -> Result<Vec<AudioDeviceInfo>, String> {
    list_input_devices(&CpalAudioHost).map_err(|e| e.to_string())
}

#[tauri::command]
fn start_capture(app_handle: AppHandle, session: State<CaptureSessionState>) -> Result<(), String> {
    let source = MixedAudioSource::new(
        Box::new(MicAudioSource::new(None)),
        Box::new(LoopbackAudioSource::new()),
    );
    let mut pipeline = VadPipeline::new();
    session
        .start(
            &source,
            Box::new(move |frame| match pipeline.push_frame(frame) {
                Ok(ready_segments) => {
                    for segment in ready_segments {
                        let _ = app_handle.emit("vad-segment", VadSegmentPayload::from(segment));
                    }
                }
                Err(err) => eprintln!("lỗi xử lý VAD pipeline: {err}"),
            }),
        )
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn stop_capture(session: State<CaptureSessionState>) -> Result<(), String> {
    session.stop().map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(CaptureSessionState::new())
        .invoke_handler(tauri::generate_handler![
            greet,
            list_input_devices_cmd,
            start_capture,
            stop_capture
        ])
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
