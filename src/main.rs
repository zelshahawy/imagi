mod app;
mod ui;
use app::BatchApp;
use eframe::{CreationContext, NativeOptions, run_native};

fn main() {
    let native_options = NativeOptions::default();
    run_native(
        "Image Batch Processor",
        native_options,
        Box::new(|_cc: &CreationContext| -> Result<Box<dyn eframe::App>, Box<dyn std::error::Error + Send + Sync>> {
            Ok(Box::new(BatchApp::default()))
        }),
    ).expect("Failed to run the application");
}
