use crate::{ui::Ui, ServerDirectory};

#[derive(strum_macros::Display, serde::Deserialize, serde::Serialize, PartialEq, Copy, Clone)]
pub enum Tab {
    Main,
    Settings,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
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

#[derive(serde::Deserialize, serde::Serialize)]
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

impl AppWrapper {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for AppWrapper {
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        eframe::set_value(_storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(eframe::egui::Visuals::dark());

        eframe::egui::TopBottomPanel::top("top_bar")
            .show(ctx, |ui| Ui::handle_top_panel(self, ctx, ui));
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            Ui::handle_main_panel(self, ctx, ui);
        });
    }
}
