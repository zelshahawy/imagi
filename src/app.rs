use eframe::egui::Context;
use eframe::{App, Frame};
use std::path::PathBuf;
use walkdir::WalkDir;

pub struct BatchApp {
    pub input_dir: Option<PathBuf>,
    pub images: Vec<PathBuf>,
    pub selected: Vec<bool>,
}

impl Default for BatchApp {
    fn default() -> Self {
        Self {
            input_dir: None,
            images: Vec::new(),
            selected: Vec::new(),
        }
    }
}

impl BatchApp {
    pub fn load_images(&mut self) {
        self.images.clear();
        if let Some(ref dir) = self.input_dir {
            for entry in WalkDir::new(dir).into_iter().filter_map(Result::ok) {
                let path = entry.path();
                if path.is_file() {
                    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                        let ext = ext.to_lowercase();
                        if matches!(
                            ext.as_str(),
                            "png" | "jpg" | "jpeg" | "bmp" | "gif" | "tiff"
                        ) {
                            self.images.push(path.to_path_buf());
                        }
                    }
                }
            }
        }
        self.selected = vec![true; self.images.len()];
    }
}

impl App for BatchApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        crate::ui::show(self, ctx);
    }
}
