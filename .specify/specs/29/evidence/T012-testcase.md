# Test case T012 — App.vue tích hợp chọn ngôn ngữ

**File test**: `app/src/App.test.ts` (describe "App - chọn cặp ngôn ngữ nguồn-đích (PBI 29)")

| Kịch bản | Expected | Actual | Kết quả |
|---|---|---|---|
| App mount, tải xong danh sách cặp ngôn ngữ | `<select>` tự chọn cặp đầu tiên (`en-vi`) | `value === "en-vi"` | PASS (FR-006 mặc định) |
| Trạng thái `idle` | `<select>` không có `disabled` | Không có | PASS (FR-004) |
| Nhấn Start (trạng thái chuyển `running`) | `<select>` có `disabled` | Có | PASS (FR-004, SC-003) |
| Đổi sang cặp `vi-en` rồi nhấn Start | `invoke("start_capture", { pairId: "vi-en" })` | Gọi đúng tham số | PASS (FR-005, SC-001) |

Xem log đầy đủ: `T013-vitest-result.log`.
