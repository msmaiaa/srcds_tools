use crate::{ui::Ui, ServerDirectory};

#[derive(strum_macros::Display, PartialEq, Copy, Clone)]
pub enum Tab {
    Main,
    Settings,
}

#[derive(Clone)]
pub struct AppState {
    pub server_directory: Option<ServerDirectory>,
    pub selected_tab: Tab,
    pub tabs: Vec<Tab>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            server_directory: None,
            selected_tab: Tab::Main,
            tabs: vec![Tab::Main, Tab::Settings],
        }
    }
}

pub struct AppWrapper {
    pub state: AppState,
}

impl Default for AppWrapper {
    fn default() -> Self {
        Self {
            state: AppState::default(),
        }
    }
}

impl eframe::App for AppWrapper {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(eframe::egui::Visuals::dark());

        eframe::egui::TopBottomPanel::top("top_bar")
            .show(ctx, |ui| Ui::handle_top_panel(self, ctx, ui));
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            Ui::handle_main_panel(self, ctx, ui);
        });
    }
}
