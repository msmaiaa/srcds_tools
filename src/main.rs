#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use srcds_tools::app_wrapper::AppWrapper;

fn main() {
    let options = eframe::NativeOptions {
        vsync: true,
        ..Default::default()
    };
    eframe::run_native(
        "srcds tools",
        options,
        Box::new(|_cc| Box::new(AppWrapper::default())),
    );
}
