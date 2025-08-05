use crate::app::BatchApp;
use eframe::egui::{
    self, ColorImage, Context, Direction, Layout, ScrollArea, TextureOptions, Ui, Vec2,
};
use image::load_from_memory;
use rfd::FileDialog;

const PLACEHOLDER_PNG: &[u8] = include_bytes!("../assets/place-holder.jpg");

fn get_placeholder_texture(ctx: &Context) -> eframe::egui::TextureHandle {
    let dyn_img = load_from_memory(PLACEHOLDER_PNG).expect("Failed to decode placeholder image");
    let rgba_img = dyn_img.to_rgba8();
    let (width, height) = rgba_img.dimensions();
    let size = [width as usize, height as usize];
    let color_image = ColorImage::from_rgba_unmultiplied(size, rgba_img.as_raw());
    ctx.load_texture("cached_placeholder", color_image, TextureOptions::default())
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
            if app.input_dir.is_some() {
                ui.label(format!("{} images", app.images.len()));
            }
        });

        ui.separator();

        let card_size = Vec2::splat(100.0);
        let columns = 5;
        ScrollArea::vertical().show(ui, |ui| {
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
                                    show_placeholder(ui, ctx);
                                    // Show filename below
                                    let name = path
                                        .file_name()
                                        .map(|n| n.to_string_lossy().to_string())
                                        .unwrap_or_default();
                                    ui.label(egui::RichText::new(name));
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

pub fn show_placeholder(ui: &mut Ui, ctx: &Context) {
    let texture = get_placeholder_texture(ctx);
    ui.add(egui::Image::new(&texture).fit_to_exact_size(Vec2::new(100.0, 100.0)));
}
