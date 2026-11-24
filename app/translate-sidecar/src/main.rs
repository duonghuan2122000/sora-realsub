//! CLI dịch local bằng NLLB-200 (CTranslate2 qua `ct2rs`). Chạy như tiến trình độc lập
//! (sidecar) được `NllbTranslator` (app-core) gọi qua `std::process::Command` — xem
//! wiki PBI-16-dich-local-sidecar để biết tại sao không gọi ct2rs in-process trong app chính.
//!
//! Dùng: translate-sidecar --model-dir <dir> --source-lang <mã> --target-lang <mã> --text <câu>

use ct2rs::{Config, TranslationOptions, Translator};
use std::process::ExitCode;

struct Args {
    model_dir: String,
    // Model Marian-MT (opus-mt-en-vi) là model 1 cặp ngôn ngữ cố định, không cần mã ngôn ngữ
    // FLORES-200 như NLLB — giữ 2 flag này chỉ để tương thích CLI với NllbTranslator (gọi
    // sidecar), không dùng tới giá trị.
    #[allow(dead_code)]
    source_lang: String,
    #[allow(dead_code)]
    target_lang: String,
    text: String,
}

fn parse_args() -> Result<Args, String> {
    let mut model_dir = None;
    let mut source_lang = None;
    let mut target_lang = None;
    let mut text = None;

    let mut it = std::env::args().skip(1);
    while let Some(flag) = it.next() {
        let value = it
            .next()
            .ok_or_else(|| format!("thiếu giá trị cho {flag}"))?;
        match flag.as_str() {
            "--model-dir" => model_dir = Some(value),
            "--source-lang" => source_lang = Some(value),
            "--target-lang" => target_lang = Some(value),
            "--text" => text = Some(value),
            other => return Err(format!("flag không nhận diện được: {other}")),
        }
    }

    Ok(Args {
        model_dir: model_dir.ok_or("thiếu --model-dir")?,
        source_lang: source_lang.ok_or("thiếu --source-lang")?,
        target_lang: target_lang.ok_or("thiếu --target-lang")?,
        text: text.ok_or("thiếu --text")?,
    })
}

fn main() -> ExitCode {
    let args = match parse_args() {
        Ok(args) => args,
        Err(msg) => {
            eprintln!("lỗi tham số: {msg}");
            return ExitCode::FAILURE;
        }
    };

    let translator = match Translator::new(&args.model_dir, &Config::default()) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("không nạp được model từ '{}': {e}", args.model_dir);
            return ExitCode::FAILURE;
        }
    };

    // Marian-MT (opus-mt-en-vi) là model 1 cặp ngôn ngữ cố định — không cần source/target
    // prefix token như NLLB, gọi translate_batch thường.
    let results = match translator.translate_batch(
        &[args.text.as_str()],
        &TranslationOptions::default(),
        None,
    ) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("lỗi infer dịch: {e}");
            return ExitCode::FAILURE;
        }
    };

    let Some((translated, _score)) = results.into_iter().next() else {
        eprintln!("model dịch không trả kết quả");
        return ExitCode::FAILURE;
    };

    println!("{}", translated.trim());
    ExitCode::SUCCESS
}
