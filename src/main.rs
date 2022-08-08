#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use srcds_tools::app_wrapper::AppWrapper;

fn main() {
    let window_size = eframe::egui::vec2(840., 400.);
    let options = eframe::NativeOptions {
        vsync: true,
        max_window_size: Some(window_size),
        min_window_size: Some(window_size),
        initial_window_size: Some(window_size),
        ..Default::default()
    };
    eframe::run_native(
        "srcds tools",
        options,
        Box::new(|_cc| Box::new(AppWrapper::new(_cc))),
    );
}
