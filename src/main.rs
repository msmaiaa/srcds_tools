#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use srcds_tools::ServerDirectory;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "srcds tools",
        options,
        Box::new(|_cc| Box::new(App::default())),
    );
}

struct App {
    server_directory: Option<ServerDirectory>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            server_directory: None,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Open server folder").clicked() {
                    let res = rfd::FileDialog::new().pick_folder();
                    if let Some(path) = res {
                        let server_directory =
                            ServerDirectory::new(&path.to_string_lossy().to_string());
                        if let Ok(server_directory) = server_directory {
                            self.server_directory = Some(server_directory);
                        } else {
                            println!("{:?}", server_directory.unwrap_err());
                            self.server_directory = None
                        }
                    }
                };
                let root_dir = self
                    .server_directory
                    .as_ref()
                    .map_or_else(|| "Invalid server directory", |s| &s.dir_root);
                ui.label(root_dir);
            })
        });
    }
}
