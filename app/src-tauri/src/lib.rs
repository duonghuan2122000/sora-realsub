#[allow(dead_code)] // PBI 3: chỉ định nghĩa interface, chưa có cài đặt thật dùng tới
pub mod core;

use core::{
    AudioDeviceInfo, CaptureSessionState, CpalAudioHost, LoopbackAudioSource, MicAudioSource,
    MixedAudioSource, ModelErrorKind, TranscriptPipeline, VadPipeline, WhisperSpeechToText,
    list_input_devices,
};
use serde::Serialize;
use tauri::{AppHandle, Emitter, State};

/// Payload sự kiện Tauri `transcript-partial`/`transcript-final` phát ra UI (camelCase, xem
/// contracts/transcript-events.md PBI 14).
#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct TranscriptEventPayload {
    start_frame_index: usize,
    end_frame_index: usize,
    text: String,
    is_final: bool,
}

impl TranscriptEventPayload {
    fn from(segment: &core::SegmentReady, chunk: &core::TranscriptChunk) -> Self {
        Self {
            start_frame_index: segment.start_frame_index,
            end_frame_index: segment.end_frame_index,
            text: chunk.text.clone(),
            is_final: chunk.is_final,
        }
    }
}

/// Payload sự kiện Tauri `model-error` phát ra UI khi model STT lỗi nạp hoặc lỗi infer
/// (camelCase, PBI 15; xem data-model.md).
#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct ModelErrorPayload {
    kind: &'static str,
    message: String,
    segment_start_frame_index: Option<usize>,
}

impl From<core::ModelErrorEvent> for ModelErrorPayload {
    fn from(event: core::ModelErrorEvent) -> Self {
        Self {
            kind: match event.kind {
                ModelErrorKind::Load => "load",
                ModelErrorKind::Infer => "infer",
            },
            message: event.message,
            segment_start_frame_index: event.segment_start_frame_index,
        }
    }
}

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
    // STT local whisper.cpp: không bắt buộc để chạy capture/VAD; nếu thiếu model, chỉ bỏ qua
    // bước phát transcript-partial/transcript-final, vad-segment vẫn phát như cũ (PBI 14).
    // Nếu có cấu hình đường dẫn nhưng nạp lỗi (thiếu file/file hỏng), báo đúng 1 lần cho UI
    // qua event `model-error` loại "load" trước khi tiếp tục phiên không có STT (PBI 15,
    // FR-001/FR-004; xem research.md quyết định 3).
    let stt = std::env::var("WHISPER_MODEL_PATH").ok().and_then(|path| {
        match WhisperSpeechToText::new(&path) {
            Ok(stt) => Some(stt),
            Err(err) => {
                let _ = app_handle.emit(
                    "model-error",
                    ModelErrorPayload::from(core::ModelErrorEvent {
                        kind: ModelErrorKind::Load,
                        message: err.to_string(),
                        segment_start_frame_index: None,
                    }),
                );
                None
            }
        }
    });
    let rt = tokio::runtime::Builder::new_current_thread()
        .build()
        .map_err(|e| e.to_string())?;
    session
        .start(
            &source,
            Box::new(move |frame| match pipeline.push_frame(frame) {
                Ok(ready_segments) => {
                    for segment in ready_segments {
                        let _ = app_handle
                            .emit("vad-segment", VadSegmentPayload::from(segment.clone()));

                        if let Some(stt) = &stt {
                            let transcript_pipeline = TranscriptPipeline::new(stt);
                            let result = rt.block_on(transcript_pipeline.process_segments(
                                vec![segment],
                                |seg, chunk| {
                                    let event = if chunk.is_final {
                                        "transcript-final"
                                    } else {
                                        "transcript-partial"
                                    };
                                    let _ = app_handle
                                        .emit(event, TranscriptEventPayload::from(seg, chunk));
                                },
                                |seg, err| {
                                    let _ = app_handle.emit(
                                        "model-error",
                                        ModelErrorPayload::from(core::ModelErrorEvent {
                                            kind: ModelErrorKind::Infer,
                                            message: err.to_string(),
                                            segment_start_frame_index: Some(
                                                seg.start_frame_index,
                                            ),
                                        }),
                                    );
                                },
                            ));
                            if let Err(err) = result {
                                eprintln!("lỗi xử lý TranscriptPipeline: {err}");
                            }
                        }
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
