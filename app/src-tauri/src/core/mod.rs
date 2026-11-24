#![allow(unused_imports)] // PBI 3: interface-only, chưa có code tiêu thụ public API này

mod audio_source;
mod error;
mod speech_to_text;
mod subtitle_sink;
mod translator;
mod types;

pub use audio_source::AudioSource;
pub use error::CoreError;
pub use speech_to_text::SpeechToText;
pub use subtitle_sink::SubtitleSink;
pub use translator::Translator;
pub use types::{AudioFrame, StreamHandle, Subtitle, TranscriptChunk};
