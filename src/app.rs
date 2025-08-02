use crate::ui;
use eframe::egui::Context;
use eframe::{App, Frame};
use std::path::PathBuf;

pub struct BatchApp {
    pub input_dir: Option<PathBuf>,
    pub images: Vec<PathBuf>,
}

impl Default for BatchApp {
    fn default() -> Self {
        Self {
            input_dir: None,
            images: Vec::new(),
        }
    }
}

impl App for BatchApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        ui::show(self, ctx);
    }
}
