mod app;
mod config;
mod logger;
mod processor;
mod queue;
mod ui;

use app::TaskFlowApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 700.0])
            .with_min_inner_size([500.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "TaskFlow",
        options,
        Box::new(|cc| Ok(Box::new(TaskFlowApp::new(cc)))),
    )
}
