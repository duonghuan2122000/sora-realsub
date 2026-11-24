# Kế hoạch triển khai: Chốt cặp ngôn ngữ mặc định

**Mã PBI**: 2
**Liên kết spec**: .specify/specs/2/spec.md
**Ngày tạo**: 2026-11-24

## Ngữ cảnh kỹ thuật

| Hạng mục | Giá trị |
|---|---|
| Ngôn ngữ / Runtime | Không áp dụng (PBI tài liệu/quyết định, không viết code) |
| Framework / Thư viện chính | Không áp dụng — ảnh hưởng gián tiếp tới trait `Translator`, `SpeechToText` (M3/M4) sau này |
| Lưu trữ dữ liệu | Không áp dụng |
| Kiểm thử | Review thủ công nội dung tài liệu (không có test tự động) |
| Nền tảng triển khai | Không áp dụng |
| Ràng buộc khác | Mã ngôn ngữ phải theo chuẩn BCP-47; quyết định phải nêu lý do chọn |

## Giai đoạn 0 — Nghiên cứu

- **Quyết định**: Cặp ngôn ngữ mặc định là nguồn `en-US` (English) → đích `vi-VN` (Vietnamese). **Lý do**: đối tượng dùng chính của Sora RealSub (họp/stream) thường nghe nội dung tiếng Anh và cần phụ đề tiếng Việt; đây cũng là gợi ý có sẵn trong `roadmap.md` M0. **Phương án khác**: để trống bắt người dùng chọn ngay lần đầu (loại — tăng ma sát trải nghiệm, không có giá trị mặc định hợp lý nào tốt hơn); mặc định đa hướng (nguồn = đích người dùng chọn tự do, không có cặp mặc định) — loại vì roadmap M6 đã tách riêng việc đổi cặp ngôn ngữ, PBI này chỉ cần 1 giá trị khởi tạo.

## Giai đoạn 1 — Thiết kế

- Không có mô hình dữ liệu, hợp đồng giao diện hay quickstart mới — PBI này chỉ ghi quyết định vào tài liệu dự án (`docs/tech-stack.md` và/hoặc wiki) làm fallback cho M3/M4 sau này, không tạo code.

## Cấu trúc dự án dự kiến

```text
docs/tech-stack.md   # bổ sung mục "Cặp ngôn ngữ mặc định" nếu chưa có
wiki/                # sora-wiki sẽ ingest quyết định này ở bước cuối của /sora-new-feature
```

## Rủi ro & ngoại lệ có lý do

- Rủi ro: nếu đối tượng người dùng mở rộng sang nhiều cặp ngôn ngữ khác ngay từ đầu, giá trị mặc định `en→vi` chỉ là fallback, không giới hạn — chấp nhận được vì M6 đã có kế hoạch cho người dùng tự đổi cặp ngôn ngữ.
