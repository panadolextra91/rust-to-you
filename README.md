<div align="center">

# 🦀 rust-to-you

**🇻🇳 Tiếng Việt** · [🇬🇧 English](README-en.md)

*Ferris phi tới một repository để bạn khỏi phải lọ mọ.*

<img src="https://res.cloudinary.com/duy8dombh/image/upload/v1780487432/ferris_r0xhkh.png" alt="Ferris 🦀" width="360" />

</div>

---

Chào bạn 👋 Mình là **Ferris** — con cua mascot của Rust 🦀

Bạn có bao giờ phải mở cả tá tab GitHub, bấm tới bấm lui chỉ để hiểu một cái repo lạ chưa? Mệt lắm đúng không. Để Ferris lo! Bạn đưa mình **một dòng lệnh**, mình phi tới repo đó, đào bới git history, lục lọi cấu trúc, hạ tầng, rồi kể lại cho bạn nghe dưới dạng một **bản báo cáo điều tra dễ thương** — song ngữ Việt–Anh.

Đây **không phải** dashboard phân tích khô khan đâu nha. Đây là **khảo cổ & buôn chuyện về repository** 🤣

```sh
rust-to-you tokio-rs/axum
```

## 🔍 Ferris kể cho bạn nghe gì?

Một báo cáo cuộn dọc, 9 mục, đọc như hồ sơ vụ án:

1. 🌱 **Báo cáo ban đầu** — tuổi repo, nhánh mặc định, sao, fork, người đóng góp, hoạt động gần nhất
2. ☠️ **Tội ác commit** — tổng commit, commit tháng này, trùm commit, hệ số xe buýt
3. 🔥 **Rừng rậm um tùm** — tổng / hoạt động / bỏ hoang / nhánh cổ nhất
4. 🏺 **Cổ vật vô giá** — file cổ nhất, file bị sửa nhiều nhất, người cũ nhất, nhánh lâu đời nhất
5. 🌿 **Súp ngôn ngữ** — tỉ lệ ngôn ngữ kèm thanh bar
6. ⚙️ **Dấu vết hạ tầng** — Docker, Terraform, GitHub Actions, GitLab CI, CircleCI, Jenkins, Dependabot, Renovate
7. 🔮 **Khí chất repository** — Ferris chấm "tính cách" repo (Phù thủy đơn độc 🧙, Đền cổ 🏛️, …) kèm bằng chứng
8. 🔎 **Phát hiện thú vị** — mấy chuyện đáng buôn
9. 🦀 **Phán quyết của Ferris** — điểm mạnh, rủi ro, và lời phán cuối cùng

## 🚀 Cài đặt nhanh

**🍺 Homebrew (gọn nhất cho macOS):**
```sh
brew install panadolextra91/tap/rust-to-you
```

**🐚 Shell — không cần cài Rust (macOS/Linux):**
```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/panadolextra91/rust-to-you/releases/latest/download/rust-to-you-installer.sh | sh
```

**📦 Có sẵn Rust thì:**
```sh
cargo install --git https://github.com/panadolextra91/rust-to-you
```

## 🎮 Dùng thế nào

```sh
rust-to-you owner/repo                      # gõ gọn
rust-to-you https://github.com/owner/repo   # hoặc URL đầy đủ
rust-to-you owner/repo --deep               # repo bự (>500MB)? Ferris vẫn đào, chấp nhận chờ lâu
rust-to-you owner/repo | less              # bản text thuần (khi pipe / không phải terminal)
```

Trong giao diện TUI: cuộn bằng **trackpad / lăn chuột**, hoặc phím `↓ ↑ j k PgUp PgDn g G`; thoát bằng `q`.

> 🛡️ **Repo quá to (trên ~500MB)?** Ferris từ chối **trước khi clone** để khỏi treo máy bạn — kèm lời nhắn cho biết repo nặng bao nhiêu và cách dùng `--deep` nếu bạn vẫn muốn đào.

> 💡 Đặt biến môi trường `GITHUB_TOKEN` để nới giới hạn API (không bắt buộc — chỉ sao/fork mới dùng API, còn lại Ferris đọc từ bản clone local).

## 🧠 Ferris làm việc kiểu gì?

Tò mò về cách Ferris điều tra (kiến trúc, luồng dữ liệu, sơ đồ)? Mời bạn đọc 👉 **[ARCHITECTURE.md](ARCHITECTURE.md)** — tài liệu kỹ thuật chi tiết.

## 🦀 Vài điều Ferris hứa
- **Chỉ đọc, không đụng chạm** — Ferris không bao giờ sửa repo của bạn.
- **Chỉ repo GitHub public** (V1).
- **Riêng tư & sạch sẽ** — chạy trên máy bạn, clone vào thư mục tạm rồi tự dọn sạch — **kể cả khi bạn Ctrl-C giữa chừng hoặc máy có trục trặc**. Lần chạy sau, Ferris còn quét dọn luôn mấy thư mục tạm sót lại từ trước.
- **An toàn từ cổng vào** — input kỳ lạ / không an toàn bị chặn ngay ở parser, trước khi đụng tới mạng hay git. Chi tiết: **[docs/THREAT-MODEL.md](docs/THREAT-MODEL.md)**.

## 📄 Giấy phép

Chọn một trong hai: **MIT** ([LICENSE-MIT](LICENSE-MIT)) hoặc **Apache-2.0** ([LICENSE-APACHE](LICENSE-APACHE)).

<div align="center">

*Made with 🦀 and a lot of gossip — by Ferris.*

</div>
