use crate::app::BatchApp;
use eframe::egui::{self, Context};
use rfd::FileDialog;

impl BatchApp {
    /// (Re)load image paths from the selected input directory
    pub fn load_images(&mut self) {
        self.images.clear();
        if let Some(ref dir) = self.input_dir {
            for entry in walkdir::WalkDir::new(dir) {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                            let ext = ext.to_lowercase();
                            if ["png", "jpg", "jpeg", "bmp", "gif", "tiff"].contains(&ext.as_str())
                            {
                                self.images.push(path.to_path_buf());
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn show(app: &mut BatchApp, ctx: &Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.horizontal(|ui| {
            if ui.button("Select Input Folder").clicked() {
                if let Some(dir) = FileDialog::new().pick_folder() {
                    app.input_dir = Some(dir.clone());
                    app.load_images();
                }
            }
            if let Some(ref dir) = app.input_dir {
                ui.label(dir.display().to_string());
            } else {
                ui.label("No folder selected");
            }
        });

        ui.separator();
        ui.label("Discovered images:");
        egui::ScrollArea::vertical().show(ui, |ui| {
            for path in &app.images {
                ui.label(path.file_name().unwrap().to_string_lossy());
            }
        });
    });
}
