use std::fs;
use std::sync::atomic::{AtomicBool, Ordering};

use tauri::{
    menu::{CheckMenuItem, Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, WindowEvent,
};

// Script Privacy Blur — dipasang ulang setiap kali halaman WhatsApp Web
// selesai dimuat (halaman bisa navigasi/reload, jadi inject sekali tidak cukup).
const PRIVACY_BLUR_JS: &str = include_str!("privacy-blur.js");

// Sumber kebenaran state blur ada di Rust (bukan di halaman WhatsApp).
static BLUR_ON: AtomicBool = AtomicBool::new(false);

// File tempat state blur dipersist antar-restart.
fn blur_state_path(app: &AppHandle) -> Option<std::path::PathBuf> {
    app.path().app_config_dir().ok().map(|mut p| {
        p.push("privacy-blur.state");
        p
    })
}

fn load_blur_state(app: &AppHandle) -> bool {
    blur_state_path(app)
        .and_then(|p| fs::read_to_string(p).ok())
        .map(|s| s.trim() == "1")
        .unwrap_or(false)
}

fn save_blur_state(app: &AppHandle, on: bool) {
    if let Some(p) = blur_state_path(app) {
        if let Some(dir) = p.parent() {
            let _ = fs::create_dir_all(dir);
        }
        let _ = fs::write(&p, if on { "1" } else { "0" });
    }
}

fn apply_blur_to_page(app: &AppHandle, on: bool) {
    if let Some(win) = app.get_webview_window("main") {
        let cmd = if on { "on" } else { "off" };
        let _ = win.eval(&format!("window.__waSritBlur && window.__waSritBlur('{cmd}')"));
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .on_page_load(|window, _payload| {
            // Setiap kali halaman selesai dimuat: pasang script blur lalu
            // terapkan state saat ini (menangani reload/navigasi WhatsApp Web).
            let on = BLUR_ON.load(Ordering::Relaxed);
            let cmd = if on { "on" } else { "off" };
            let _ = window.eval(PRIVACY_BLUR_JS);
            let _ = window.eval(&format!("window.__waSritBlur && window.__waSritBlur('{cmd}')"));
        })
        .on_window_event(|window, event| match event {
            // Hemat RAM & nyaman: tombol X hanya sembunyikan ke tray, app tetap
            // bisa menerima notifikasi; window tetap satu, tidak bertambah.
            WindowEvent::CloseRequested { api, .. } => {
                window.hide().ok();
                api.prevent_close();
            }
            _ => {}
        })
        .setup(|app| {
            // Restore state blur terakhir dari disk.
            let initial = load_blur_state(app.handle());
            BLUR_ON.store(initial, Ordering::Relaxed);

            let show = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
            let blur = CheckMenuItem::with_id(
                app,
                "blur",
                "Privacy Blur",
                true,
                initial, // checkmark awal = state tersimpan
                None::<&str>,
            )?;
            let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show, &blur, &quit])?;

            let _tray = TrayIconBuilder::with_id("main")
                .tooltip("Wa-Srit!")
                .menu(&menu)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(win) = app.get_webview_window("main") {
                            win.show().ok();
                            win.set_focus().ok();
                        }
                    }
                    "blur" => {
                        // Toggle: state Rust = sumber kebenaran. Persist lalu
                        // terapkan ke halaman, sinkronkan checkmark tray.
                        let next = !BLUR_ON.load(Ordering::Relaxed);
                        BLUR_ON.store(next, Ordering::Relaxed);
                        save_blur_state(app, next);
                        apply_blur_to_page(app, next);
                        if let Some(item) = app.menu().and_then(|m| m.get("blur")) {
                            if let Some(check) = item.as_check_menuitem() {
                                let _ = check.set_checked(next);
                            }
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        if let Some(win) = tray.app_handle().get_webview_window("main") {
                            win.show().ok();
                            win.set_focus().ok();
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
