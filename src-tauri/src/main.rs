#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // tanpa console window di rilis

fn main() {
    walite_lib::run()
}
