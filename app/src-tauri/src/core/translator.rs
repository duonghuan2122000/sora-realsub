use super::error::CoreError;
use async_trait::async_trait;

/// Dịch văn bản, có thể kèm ngữ cảnh (1-2 câu trước) để dịch chính xác hơn.
#[async_trait]
pub trait Translator: Send + Sync {
    async fn translate(&self, text: &str, context: &[String]) -> Result<String, CoreError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Cài đặt giả: ghép tiền tố "[dịch]" + số câu ngữ cảnh nhận được.
    struct MockTranslator;

    #[async_trait]
    impl Translator for MockTranslator {
        async fn translate(&self, text: &str, context: &[String]) -> Result<String, CoreError> {
            Ok(format!("[dịch:{}] {}", context.len(), text))
        }
    }

    #[tokio::test]
    async fn dich_kem_ngu_canh() {
        let translator = MockTranslator;
        let context = vec!["câu trước".to_string()];
        let result = translator.translate("hello", &context).await.unwrap();
        assert_eq!(result, "[dịch:1] hello");
    }

    #[tokio::test]
    async fn goi_qua_trait_object() {
        let translator: Box<dyn Translator> = Box::new(MockTranslator);
        let result = translator.translate("hi", &[]).await.unwrap();
        assert_eq!(result, "[dịch:0] hi");
    }
}
