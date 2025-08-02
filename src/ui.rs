use crate::app::BatchApp;
use eframe::egui::{self, Context, Direction, Label, Layout, Vec2};
use rfd::FileDialog;

pub fn show(app: &mut BatchApp, ctx: &Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.horizontal(|ui| {
            if ui.button("Select Input Folder").clicked() {
                if let Some(dir) = FileDialog::new().pick_folder() {
                    app.input_dir = Some(dir.clone());
                    app.load_images();
                }
            }
            if app.input_dir.is_some() {
                ui.label(format!("{} images", app.images.len()));
            }
        });

        ui.separator();

        let card_size = Vec2::splat(100.0);
        let columns = 5;
        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("image_grid")
                .striped(true)
                .spacing([8.0, 8.0])
                .show(ui, |ui| {
                    for (i, path) in app.images.iter().enumerate() {
                        ui.vertical_centered(|ui| {
                            ui.checkbox(&mut app.selected[i], "");
                            ui.allocate_ui_with_layout(
                                card_size,
                                Layout::centered_and_justified(Direction::TopDown),
                                |ui| {
                                    let name =
                                        path.file_name().unwrap().to_string_lossy().to_string();
                                    ui.add(Label::new(name).wrap());
                                },
                            );
                        });
                        if i % columns == columns - 1 {
                            ui.end_row();
                        }
                    }
                });
        });
    });
}
