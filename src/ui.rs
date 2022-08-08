use eframe::egui::Context;
use eframe::egui::{self, style::Margin};

use crate::{
    app_wrapper::{AppWrapper, Tab},
    OpenableKind,
};

pub struct Ui {}

impl Ui {
    pub fn handle_main_panel(app: &mut AppWrapper, ctx: &Context, ui: &mut eframe::egui::Ui) {
        match app.state.selected_tab {
            Tab::Main => render_main_tab(app, ctx, ui),
            Tab::Settings => render_settings_tab(app, ctx, ui),
        }
        ()
    }

    pub fn handle_top_panel(app: &mut AppWrapper, _: &Context, ui: &mut eframe::egui::Ui) {
        ui.horizontal(|ui| {
            for tab in app.state.tabs.iter() {
                let mut btn = egui::Button::new(tab.to_string());
                if app.state.selected_tab == *tab {
                    btn = btn.fill(egui::Color32::from_rgb(76, 89, 69));
                }
                if ui.add(btn).clicked() {
                    app.state.selected_tab = *tab;
                }
            }
        });
        ()
    }
}

fn render_main_tab(app: &mut AppWrapper, ctx: &Context, ui: &mut eframe::egui::Ui) {
    let dir = match app.state.server_directory.as_ref() {
        Some(dir) => dir,
        None => return,
    };

    let mut folders = Vec::new();
    let mut files = Vec::new();
    for openable in dir.openables() {
        match openable.kind {
            OpenableKind::File => files.push(openable),
            OpenableKind::Folder => folders.push(openable),
        }
    }

    //add padding on the ui
    //ui.spacing_mut().item_spacing = egui::vec2(10., 10.);

    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.heading("Folders");
            ui.add_space(10.0);
            for folder in folders.iter() {
                if ui.button(folder.label()).clicked() {
                    // TODO: handle this properly
                    folder.open().unwrap();
                }
                ui.add_space(2.);
            }
        });
        ui.add_space(15.0);
        ui.vertical(|ui| {
            ui.heading("Files");
            ui.add_space(10.0);
            for file in files.iter() {
                if ui.button(file.label()).clicked() {
                    // TODO: handle this properly
                    file.open().unwrap();
                }
                ui.add_space(2.);
            }
        });
    });
}

fn render_settings_tab(app: &mut AppWrapper, ctx: &Context, ui: &mut eframe::egui::Ui) {
    use crate::ServerDirectory;
    ui.horizontal(|ui| {
        if ui.button("Open server folder").clicked() {
            let res = rfd::FileDialog::new().pick_folder();
            if let Some(path) = res {
                let server_directory = ServerDirectory::new(&path.to_string_lossy().to_string());
                if let Ok(server_directory) = server_directory {
                    app.state.server_directory = Some(server_directory);
                } else {
                    println!("{:?}", server_directory.unwrap_err());
                    app.state.server_directory = None
                }
            }
        };

        let root_dir = app
            .state
            .server_directory
            .as_ref()
            .map_or_else(|| "Invalid server directory", |s| &s.dir_root.path());
        ui.label(root_dir);
    });
}
