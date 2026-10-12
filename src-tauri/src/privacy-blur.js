// Wa-Srit! Privacy Blur — di-inject ke WhatsApp Web setiap kali halaman
// selesai dimuat (Rust memanggil via on_page_load handler).
// Sumber kebenaran state = Rust (checkmark tray + config file persist).
// Script ini hanya: memasang CSS blur dan fungsi penerima perintah.

(function () {
  const CSS = `
/* ==== Wa-Srit! Privacy Blur ==== */
/* Isi bubble pesan */
html.wa-srit-blur span.selectable-text,
html.wa-srit-blur span.copyable-text,
/* Preview pesan di chat list */
html.wa-srit-blur [data-testid="cell-frame-message-text"],
html.wa-srit-blur [data-testid="message-text"],
/* Nama kontak di chat list */
html.wa-srit-blur [data-testid="chat-list"] [role="gridcell"] span[dir="auto"],
html.wa-srit-blur [data-testid="cell-frame-title"] span,
/* Foto profil di chat list, header, panel info */
html.wa-srit-blur [data-testid="chat-list"] img,
html.wa-srit-blur [data-testid="conversation-info"] img,
html.wa-srit-blur [data-testid="conversation-panel-header"] img,
html.wa-srit-blur header img {
  filter: blur(6px) !important;
  transition: filter 0.15s ease !important;
}

/* Hover pada pesan/cell = konten jelas kembali */
html.wa-srit-blur span.selectable-text:hover,
html.wa-srit-blur span.copyable-text:hover,
html.wa-srit-blur [data-testid="cell-frame-message-text"]:hover,
html.wa-srit-blur [data-testid="message-text"]:hover,
html.wa-srit-blur [data-testid="chat-list"] [role="gridcell"]:hover span[dir="auto"],
html.wa-srit-blur [data-testid="chat-list"] [role="gridcell"]:hover img,
html.wa-srit-blur [data-testid="conversation-info"]:hover img,
html.wa-srit-blur [data-testid="conversation-panel-header"]:hover img,
html.wa-srit-blur header:hover img {
  filter: none !important;
}
`;

  function apply(state) {
    document.documentElement.classList.toggle("wa-srit-blur", state === true);
  }

  // Sederhana dan cukup: dipanggil Rust dengan "on" / "off".
  window.__waSritBlur = function (cmd) {
    if (cmd === "on") apply(true);
    else if (cmd === "off") apply(false);
    else return document.documentElement.classList.contains("wa-srit-blur");
  };

  const style = document.createElement("style");
  style.id = "wa-srit-blur-style";
  style.textContent = CSS;
  (document.head || document.documentElement).appendChild(style);

  apply(false); // Rust mengirim "on" setelah inject jika blur aktif
})();
