# Bằng chứng kiểm thử — PBI 1: Chốt OS mục tiêu

## Test case

**Bước**: Mở [`docs/tech-stack.md`](../../../../docs/tech-stack.md), tìm mục "Nền tảng hỗ trợ".
**Kỳ vọng (SC-001)**: Đọc thấy OS mục tiêu đầu tiên (Windows) và kế hoạch OS thứ hai (macOS, Backlog) trong vòng 1 phút, kèm lý do.
**Kết quả**: PASS — mục "Nền tảng hỗ trợ" có bảng Windows/macOS/Linux + dòng "Quyết định (PBI 1)" nêu rõ Windows trước, lý do (đối tượng dùng Windows, WASAPI loopback sẵn có), macOS ở Backlog.

## Snapshot nội dung đã cập nhật (docs/tech-stack.md, mục "Nền tảng hỗ trợ")

```
## Nền tảng hỗ trợ

| OS | Trạng thái | Ghi chú |
|---|---|---|
| Windows 10/11 | Hỗ trợ (giai đoạn 1) | WASAPI loopback, WebView2 |
| macOS | Roadmap | ScreenCaptureKit hoặc BlackHole để lấy âm thanh hệ thống |
| Linux | Roadmap | PipeWire/PulseAudio monitor |

Giữ phần đặc thù OS sau trait `AudioSource` và module `platform/` để mở rộng sau này không phải sửa pipeline.

**Quyết định (PBI 1)**: Windows là OS mục tiêu đầu tiên — đối tượng dùng chính (họp/stream công ty) phổ biến Windows, và WASAPI loopback là API sẵn có, ổn định, không cần driver phụ như macOS (BlackHole). macOS triển khai sau, nằm ở Backlog, không block các milestone M1–M8.
```

## Kiểm tra roadmap.md (T003)

`roadmap.md` mục Backlog đã có dòng "Hỗ trợ macOS (BlackHole/ScreenCaptureKit cho loopback)" — khớp với quyết định, không cần sửa.
