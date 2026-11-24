# Hợp đồng sự kiện Tauri: `vad-segment`

**Hướng**: Core → UI (Tauri event, không phải command)
**Tên sự kiện**: `vad-segment`
**Tần suất**: phát đúng 1 lần cho mỗi segment speech vừa đóng, trong lúc phiên capture đang chạy; ngừng sau `stop_capture`

## Payload (JSON)

```json
{
  "startFrameIndex": 0,
  "endFrameIndex": 7,
  "samples": [0.0, 0.01, -0.02],
  "sampleRate": 16000
}
```

| Trường | Kiểu | Bắt buộc | Ghi chú |
|---|---|---|---|
| `startFrameIndex` | number (int ≥ 0) | có | chỉ số frame VAD bắt đầu đoạn |
| `endFrameIndex` | number (int ≥ `startFrameIndex`) | có | chỉ số frame VAD kết thúc đoạn (bao gồm) |
| `samples` | number[] | có | mẫu âm thanh 16kHz mono của đoạn, có thể rỗng nếu đoạn quá ngắn (không kỳ vọng thực tế do ngưỡng VAD_CHUNK_SIZE) |
| `sampleRate` | number | có | luôn `16000` |

## Đảm bảo thứ tự & không trùng (FR-003, FR-004)

- UI nhận các sự kiện `vad-segment` theo đúng thứ tự `startFrameIndex` tăng dần, khớp thời gian phát sinh thực tế.
- Mỗi segment chỉ xuất hiện trong đúng 1 sự kiện.
