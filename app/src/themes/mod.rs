use eframe::egui::Visuals;

pub mod dark;
pub mod light;

pub enum AppTheme {
    Dark,
    Light,
}
impl AppTheme {
    pub fn visuals(&self) -> Visuals {
        match self {
            AppTheme::Dark => dark::VISUALS,
            AppTheme::Light => light::visuals(),
        }
    }
}
