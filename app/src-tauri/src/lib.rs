#[allow(dead_code)] // PBI 3: chỉ định nghĩa interface, chưa có cài đặt thật dùng tới
pub mod core;

use core::{
    AudioDeviceInfo, CaptureSessionState, CpalAudioHost, LoopbackAudioSource, MicAudioSource,
    MixedAudioSource, ModelErrorKind, RecentTranscriptContext, SidecarTranslator,
    TranscriptPipeline, Translator, VadPipeline, WhisperSpeechToText, list_input_devices,
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

/// Payload sự kiện Tauri `subtitle-ready` phát ra UI sau khi dịch xong 1 câu transcript final
/// (camelCase, PBI 19; xem data-model.md).
#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct SubtitlePayload {
    start_frame_index: usize,
    end_frame_index: usize,
    source_text: String,
    translated_text: String,
}

impl SubtitlePayload {
    fn from(segment: &core::SegmentReady, source_text: &str, translated_text: &str) -> Self {
        Self {
            start_frame_index: segment.start_frame_index,
            end_frame_index: segment.end_frame_index,
            source_text: source_text.to_string(),
            translated_text: translated_text.to_string(),
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
                ModelErrorKind::Translate => "translate",
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

/// Nối 1 `TranscriptChunk` (kết quả STT cho 1 segment) vào bước dịch, tách khỏi `start_capture`
/// để test được bằng mock `Translator`, không cần `AppHandle`/Tauri runtime (PBI 19).
///
/// Bỏ qua (không gọi `on_subtitle`/`on_error`, không `push` vào `recent_ctx`) khi `chunk` là
/// partial hoặc text rỗng/khoảng trắng — FR-005, FR-006. Khi dịch thành công, gọi
/// `on_subtitle(source_text, translated_text)`. Khi dịch lỗi, gọi `on_error(&err)` — không dừng,
/// vẫn `push` câu gốc vào `recent_ctx` để làm ngữ cảnh cho câu sau (research.md quyết định 3, 4;
/// FR-001, FR-002, FR-003, FR-007).
async fn translate_final_chunk(
    translator: &dyn Translator,
    recent_ctx: &mut RecentTranscriptContext,
    chunk: &core::TranscriptChunk,
    mut on_subtitle: impl FnMut(&str, &str),
    mut on_error: impl FnMut(&core::CoreError),
) {
    if !chunk.is_final || chunk.text.trim().is_empty() {
        return;
    }
    let context = recent_ctx.recent();
    match translator.translate(&chunk.text, &context).await {
        Ok(translated_text) => on_subtitle(&chunk.text, &translated_text),
        Err(err) => on_error(&err),
    }
    recent_ctx.push(&chunk.text);
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
    // Dịch tự động mỗi câu transcript final: tái dùng SidecarTranslator (PBI 16/17), optional
    // giống stt — thiếu cấu hình thì bỏ qua bước dịch, phiên vẫn chạy (PBI 19, FR-008; research.md
    // quyết định 5).
    let translator = match (
        std::env::var("SORA_TRANSLATE_SIDECAR_PATH"),
        std::env::var("SORA_TRANSLATE_MODEL_PATH"),
    ) {
        (Ok(exe), Ok(model_dir)) => SidecarTranslator::new(&exe, &model_dir).ok(),
        _ => None,
    };
    // Ngữ cảnh 1-2 câu transcript final gần nhất trong phiên (PBI 17), reset mỗi lần
    // start_capture — research.md quyết định 2.
    let mut recent_ctx = RecentTranscriptContext::new();
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
                                vec![segment.clone()],
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
                            match result {
                                Ok(chunks) => {
                                    if let Some(translator) = &translator {
                                        for chunk in &chunks {
                                            rt.block_on(translate_final_chunk(
                                                translator,
                                                &mut recent_ctx,
                                                chunk,
                                                |source_text, translated_text| {
                                                    let _ = app_handle.emit(
                                                        "subtitle-ready",
                                                        SubtitlePayload::from(
                                                            &segment,
                                                            source_text,
                                                            translated_text,
                                                        ),
                                                    );
                                                },
                                                |err| {
                                                    let _ = app_handle.emit(
                                                        "model-error",
                                                        ModelErrorPayload::from(
                                                            core::ModelErrorEvent {
                                                                kind: ModelErrorKind::Translate,
                                                                message: err.to_string(),
                                                                segment_start_frame_index: Some(
                                                                    segment.start_frame_index,
                                                                ),
                                                            },
                                                        ),
                                                    );
                                                },
                                            ));
                                        }
                                    }
                                }
                                Err(err) => {
                                    eprintln!("lỗi xử lý TranscriptPipeline: {err}");
                                }
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

    // PBI 19 — T009/T010: Translator giả ghi lại (text, context) mỗi lần được gọi, trả
    // "[dich:N] text" với N là số câu context nhận được.
    struct RecordingTranslator {
        calls: std::sync::Mutex<Vec<(String, Vec<String>)>>,
    }

    impl RecordingTranslator {
        fn new() -> Self {
            Self {
                calls: std::sync::Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait::async_trait]
    impl Translator for RecordingTranslator {
        async fn translate(
            &self,
            text: &str,
            context: &[String],
        ) -> Result<String, core::CoreError> {
            self.calls
                .lock()
                .unwrap()
                .push((text.to_string(), context.to_vec()));
            Ok(format!("[dich:{}] {}", context.len(), text))
        }
    }

    // PBI 19 — T013: Translator giả trả lỗi đúng vào các lần gọi nằm trong `fail_at_calls`
    // (0-based), còn lại trả Ok.
    struct FlakyTranslator {
        call_count: std::sync::atomic::AtomicUsize,
        fail_at_calls: Vec<usize>,
    }

    impl FlakyTranslator {
        fn new(fail_at_calls: Vec<usize>) -> Self {
            Self {
                call_count: std::sync::atomic::AtomicUsize::new(0),
                fail_at_calls,
            }
        }
    }

    #[async_trait::async_trait]
    impl Translator for FlakyTranslator {
        async fn translate(
            &self,
            text: &str,
            _context: &[String],
        ) -> Result<String, core::CoreError> {
            let n = self
                .call_count
                .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            if self.fail_at_calls.contains(&n) {
                Err(core::CoreError::Provider(format!(
                    "lỗi dịch giả lập tại lần gọi {n}"
                )))
            } else {
                Ok(format!("[dich] {text}"))
            }
        }
    }

    fn final_chunk(text: &str) -> core::TranscriptChunk {
        core::TranscriptChunk {
            text: text.to_string(),
            is_final: true,
        }
    }

    fn partial_chunk(text: &str) -> core::TranscriptChunk {
        core::TranscriptChunk {
            text: text.to_string(),
            is_final: false,
        }
    }

    // T009: nhiều câu final liên tiếp -> đúng số lượng + đúng thứ tự subtitle phát ra.
    #[tokio::test]
    async fn nhieu_cau_final_phat_subtitle_dung_thu_tu() {
        let translator = RecordingTranslator::new();
        let mut recent_ctx = RecentTranscriptContext::new();
        let subtitles: std::sync::Mutex<Vec<(String, String)>> = std::sync::Mutex::new(Vec::new());

        for text in ["Cau mot.", "Cau hai.", "Cau ba."] {
            translate_final_chunk(
                &translator,
                &mut recent_ctx,
                &final_chunk(text),
                |src, tr| {
                    subtitles
                        .lock()
                        .unwrap()
                        .push((src.to_string(), tr.to_string()));
                },
                |_| panic!("không kỳ vọng lỗi"),
            )
            .await;
        }

        let subtitles = subtitles.into_inner().unwrap();
        assert_eq!(subtitles.len(), 3);
        assert_eq!(subtitles[0].0, "Cau mot.");
        assert_eq!(subtitles[1].0, "Cau hai.");
        assert_eq!(subtitles[2].0, "Cau ba.");
    }

    // T010: câu final thứ 2+ nhận đúng tối đa 2 câu context gần nhất, theo thứ tự cũ->mới.
    #[tokio::test]
    async fn cau_final_sau_nhan_dung_toi_da_2_cau_context_gan_nhat() {
        let translator = RecordingTranslator::new();
        let mut recent_ctx = RecentTranscriptContext::new();

        for text in ["Cau 1.", "Cau 2.", "Cau 3.", "Cau 4."] {
            translate_final_chunk(
                &translator,
                &mut recent_ctx,
                &final_chunk(text),
                |_, _| {},
                |_| panic!("không kỳ vọng lỗi"),
            )
            .await;
        }

        let calls = translator.calls.into_inner().unwrap();
        assert_eq!(calls.len(), 4);
        assert_eq!(calls[0], ("Cau 1.".to_string(), vec![]));
        assert_eq!(calls[1], ("Cau 2.".to_string(), vec!["Cau 1.".to_string()]));
        assert_eq!(
            calls[2],
            ("Cau 3.".to_string(), vec!["Cau 1.".to_string(), "Cau 2.".to_string()])
        );
        assert_eq!(
            calls[3],
            ("Cau 4.".to_string(), vec!["Cau 2.".to_string(), "Cau 3.".to_string()])
        );
    }

    // T011: chunk partial không gọi translate(), không phát subtitle.
    #[tokio::test]
    async fn chunk_partial_khong_goi_dich_khong_phat_subtitle() {
        let translator = RecordingTranslator::new();
        let mut recent_ctx = RecentTranscriptContext::new();
        let mut subtitle_count = 0;

        translate_final_chunk(
            &translator,
            &mut recent_ctx,
            &partial_chunk("dang noi..."),
            |_, _| subtitle_count += 1,
            |_| panic!("không kỳ vọng lỗi"),
        )
        .await;

        assert_eq!(subtitle_count, 0);
        assert!(translator.calls.into_inner().unwrap().is_empty());
        assert!(recent_ctx.recent().is_empty());
    }

    // T012: chunk final rỗng/khoảng trắng không gọi translate(), không phát subtitle, không
    // được push vào context.
    #[tokio::test]
    async fn chunk_final_rong_khong_goi_dich_khong_phat_subtitle_khong_push_context() {
        let translator = RecordingTranslator::new();
        let mut recent_ctx = RecentTranscriptContext::new();
        let mut subtitle_count = 0;

        for text in ["", "   "] {
            translate_final_chunk(
                &translator,
                &mut recent_ctx,
                &final_chunk(text),
                |_, _| subtitle_count += 1,
                |_| panic!("không kỳ vọng lỗi"),
            )
            .await;
        }

        assert_eq!(subtitle_count, 0);
        assert!(translator.calls.into_inner().unwrap().is_empty());
        assert!(recent_ctx.recent().is_empty());
    }

    // T013: 1 câu lỗi dịch giữa nhiều câu final liên tiếp -> không panic, số subtitle phát ra
    // = số câu dịch thành công, đúng 1 lỗi, các câu sau vẫn tiếp tục được dịch.
    #[tokio::test]
    async fn loi_dich_1_cau_khong_lam_dung_cac_cau_khac() {
        let translator = FlakyTranslator::new(vec![1]);
        let mut recent_ctx = RecentTranscriptContext::new();
        let subtitle_count = std::sync::atomic::AtomicUsize::new(0);
        let error_count = std::sync::atomic::AtomicUsize::new(0);

        for text in ["Cau 1.", "Cau 2.", "Cau 3."] {
            translate_final_chunk(
                &translator,
                &mut recent_ctx,
                &final_chunk(text),
                |_, _| {
                    subtitle_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                },
                |_| {
                    error_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                },
            )
            .await;
        }

        assert_eq!(subtitle_count.load(std::sync::atomic::Ordering::SeqCst), 2);
        assert_eq!(error_count.load(std::sync::atomic::Ordering::SeqCst), 1);
        // Câu lỗi dịch vẫn được push làm ngữ cảnh (bản thân câu gốc vẫn hợp lệ).
        assert_eq!(recent_ctx.recent(), vec!["Cau 2.", "Cau 3."]);
    }
}
