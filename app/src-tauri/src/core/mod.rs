#![allow(unused_imports)] // PBI 3: interface-only, chưa có code tiêu thụ public API này

mod audio_source;
mod capture_session;
mod devices;
mod error;
mod loopback_audio_source;
mod mic_audio_source;
mod mixed_audio_source;
mod segment;
mod speech_to_text;
mod subtitle_sink;
mod translator;
mod types;
mod vad;
mod vad_pipeline;

pub use audio_source::AudioSource;
pub use capture_session::CaptureSessionState;
pub use devices::{AudioDeviceInfo, AudioHost, CpalAudioHost, list_input_devices};
pub use error::CoreError;
pub use loopback_audio_source::LoopbackAudioSource;
pub use mic_audio_source::MicAudioSource;
pub use mixed_audio_source::MixedAudioSource;
pub use mixed_audio_source::resample_linear;
pub use segment::{Segment, segment, segment_with_min_silence};
pub use speech_to_text::SpeechToText;
pub use subtitle_sink::SubtitleSink;
pub use translator::Translator;
pub use types::{AudioFrame, StreamHandle, Subtitle, TranscriptChunk};
pub use vad::{FrameLabel, detect, detect_with_threshold};
pub use vad_pipeline::{SegmentReady, VadPipeline};
