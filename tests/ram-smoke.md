# RAM Smoke Test — WaLite vs Browser

Alat: Task Manager > tab Details, kolom "Memory (active)".

1. Bersihkan: tutup semua window. Catat baseline RAM.
2. Buka Edge/Chrome -> https://web.whatsapp.com, login, tunggu 60 detik,
   catat total RAM semua proses browser terkait (jumlahkan semua child process
   tab tersebut, atau catat delta RAM sebelum/sesudah).
3. Tutup browser. Jalankan WaLite, login akun yang sama, tunggu 60 detik,
   catat RAM proses "WaLite.exe" + proses "msedpwebview2" terkait.
4. Ekspektasi: WaLite total < 60% dari browser path. Jika tidak, catat angka.

Hasil pengukuran:
- Browser: ____ MB
- WaLite: ____ MB
