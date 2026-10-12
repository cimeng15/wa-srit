# Wa-Srit!

Aplikasi desktop **WhatsApp Web ringan untuk Windows** — dibangun dengan **Tauri 2 + WebView2** (system webview), bukan browser penuh, sehingga konsumsi RAM jauh lebih rendah dibanding membuka Chrome/Edge untuk WhatsApp Web.

> Wa-Srit! adalah wrapper resmi dari sisi teknologi: aplikasi tetap memuat `https://web.whatsapp.com` di jendela aplikasi sendiri (tanpa toolbar browser). Tidak ada implementasi protokol WhatsApp.

## Kenapa ringan?

- **Tauri + WebView2**: memakai WebView2 yang sudah terinstall di Windows 10/11 — tidak ada Chromium bundel (binary ~3–10 MB, RAM hanya untuk satu webview).
- **Tanpa devtools** di build rilis.
- **Minimize-to-tray**: tombol X hanya sembunyikan ke tray agar notifikasi tetap hidup.

## Dev

Prasyarat: Rust stable, Node.js 20+, dan di Linux: `libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev`.

```bash
npm install
npm run tauri -- dev   # atau: npm run dev
```

## Build installer Windows

Jalur build utama adalah **GitHub Actions** (runner `windows-latest`) — tidak perlu PC Windows:

1. Push repo ke GitHub.
2. Buat tag rilis:

   ```bash
   git tag v0.1.0
   git push origin v0.1.0
   ```

3. Tab **Actions**: workflow `build` dan `release` berjalan (~15 menit pertama kali).
   - Artifact `wa-srit-windows-installer`: `Wa-Srit_0.1.0_x64_en-US.msi` + `Wa-Srit_0.1.0_x64-setup.exe`
   - Halaman **Releases** menampilkan rilis `v0.1.0` dengan kedua installer.

Build manual juga bisa dijalankan dari tab **Actions → build → Run workflow** (`workflow_dispatch`), tanpa membuat tag.

> Installer tidak ditandatangani → SmartScreen menampilkan "Windows protected your PC" sekali saat install pertama → klik **More info → Run anyway**.

## Penggunaan

- Tombol **X** = sembunyikan ke tray (app tetap hidup untuk notifikasi).
- Klik ikon tray = tampilkan window lagi.
- Menu tray → **Quit** = keluar penuh.

## Benchmark RAM

Lihat hasil pengukuran dan metode di [`tests/ram-smoke.md`](tests/ram-smoke.md).

Ekspektasi realistis: browser penuh ~700–1200 MB aktif untuk satu tab WhatsApp Web; Tauri+WebView2 umumnya ~250–500 MB (target: < 60% dari jalur browser).
