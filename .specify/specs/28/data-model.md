# Mô hình trạng thái: Nút Start/Stop (PBI 28)

## `SessionButtonState` (frontend, `App.vue`)

Enum 4 giá trị, chỉ sống ở phía UI (Core không cần biết enum này, chỉ trả `Result<(), String>` cho 2 lệnh):

| Giá trị | Nhãn hiển thị | Nút bấm được? | Đổi thiết bị được? |
|---|---|---|---|
| `idle` | Start | Có (nếu đủ thiết bị) | Có |
| `starting` | Đang chạy... | Không | Không |
| `running` | Stop | Có | Không |
| `stopping` | Đang dừng... | Không | Không |

## Chuyển trạng thái

```text
idle --(bấm Start, invoke start_capture)--> starting
starting --(start_capture resolve)--> running
starting --(start_capture reject)--> idle   (+ hiển thị lỗi)

running --(bấm Stop, invoke stop_capture)--> stopping
stopping --(stop_capture resolve)--> idle
stopping --(stop_capture reject)--> idle    (+ hiển thị lỗi; NotRunning coi như đã dừng)
```

Không có transition nào nhận input bấm nút khi đang ở `starting`/`stopping` (nút bị khoá, FR-002 + SC-003).
