# Sora RealSub - Style Guide

Quy ước UI dùng chung cho toàn app. Mọi component chỉ dùng token/class theo quy ước dưới, không hardcode giá trị, không tự chế style riêng cho từng màn hình.

## Nguyên tắc

- **Thống nhất**: mọi màn hình/cửa sổ dùng chung token, component, layout pattern.
- **SCSS** thay CSS thuần. Không viết file `.css` tay, chỉ `.scss`.
- **Đặt tên class theo BEM**, tiền tố bắt buộc `sora-`:
  - Block: `sora-block`
  - Element: `sora-block__element`
  - Modifier: `sora-block--modifier` hoặc `sora-block__element--modifier`
  - Ví dụ: `sora-subtitle`, `sora-subtitle__text`, `sora-subtitle__text--active`, `sora-button--danger`

## Cấu trúc file

```
ui/src/styles/
  tokens.scss     # design tokens (màu, spacing, typography, radius, shadow, z-index)
  base.scss       # reset, base element style
  components/     # 1 file scss / component, theo tên block BEM
```

## Token

### Màu (semantic, hỗ trợ light/dark)

`--color-bg`, `--color-surface`, `--color-text`, `--color-text-muted`, `--color-primary`, `--color-danger`, `--color-border`...

### Typography

- 1 font UI + 1 font mono.
- Thang cỡ chữ cố định: `xs, sm, base, lg, xl`.

### Spacing

Thang 4px: `4, 8, 12, 16, 24, 32`.

### Khác

- Bo góc, đổ bóng, z-index: định nghĩa theo token, không hardcode số.

## Phụ đề (subtitle overlay)

- Cỡ chữ, độ đậm, màu nền/viền, độ trong suốt: chỉnh được qua settings, map vào token/CSS variable.
- Mặc định độ tương phản cao để đọc khi chia sẻ màn hình.

## Trạng thái UI

Quy ước class chung cho các trạng thái, tiền tố `sora-is-`:

- `sora-is-loading`, `sora-is-error`, `sora-is-empty`, `sora-is-disconnected`

## Trợ năng

- Tương phản tối thiểu đạt WCAG AA.
- Mọi control tương tác được hỗ trợ bàn phím (focus state rõ ràng, dùng token riêng cho `:focus-visible`).

## Hiệu năng UI

- Không block UI thread khi khởi động; phần UI hiện ngay, model/engine nặng load sau (lazy), để đạt mục tiêu khởi động app dưới 1 giây.
