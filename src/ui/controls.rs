use egui::Ui;
use std::path::PathBuf;

pub struct ControlsState {
    pub script_changed: bool,
    pub output_changed: bool,
    pub logging_changed: bool,
    pub start_clicked: bool,
    pub cancel_clicked: bool,
    pub add_script_clicked: bool,
    pub browse_output_clicked: bool,
    pub script_to_remove: Option<usize>,
}

impl Default for ControlsState {
    fn default() -> Self {
        Self {
            script_changed: false,
            output_changed: false,
            logging_changed: false,
            start_clicked: false,
            cancel_clicked: false,
            add_script_clicked: false,
            browse_output_clicked: false,
            script_to_remove: None,
        }
    }
}

pub fn render_controls(
    ui: &mut Ui,
    scripts: &[String],
    selected_script: &mut Option<String>,
    output_dir: &mut Option<PathBuf>,
    logging_enabled: &mut bool,
    is_processing: bool,
) -> ControlsState {
    let mut state = ControlsState::default();

    // Script list with add/remove
    ui.label("Scripts:");

    // Show existing scripts with remove button
    for (index, script) in scripts.iter().enumerate() {
        ui.horizontal(|ui| {
            // Script name (selectable)
            let is_selected = selected_script.as_ref() == Some(script);
            if ui.selectable_label(is_selected, script).clicked() {
                *selected_script = Some(script.clone());
                state.script_changed = true;
            }

            // Remove button (X)
            if ui.button("✖").clicked() {
                state.script_to_remove = Some(index);
            }
        });
    }

    // Add script button
    if ui.button("+ Add Script").clicked() {
        state.add_script_clicked = true;
    }

    ui.add_space(10.0);

    ui.horizontal(|ui| {
        ui.label("Output:");

        let dir_text = output_dir
            .as_ref()
            .and_then(|p| p.to_str())
            .unwrap_or("Not selected");

        ui.add(egui::Label::new(dir_text).truncate());

        if ui.button("Browse...").clicked() {
            state.browse_output_clicked = true;
        }
    });

    ui.horizontal(|ui| {
        if ui.checkbox(logging_enabled, "Enable logging").changed() {
            state.logging_changed = true;
        }
    });

    ui.separator();

    // Start/Cancel button
    let button_text = if is_processing { "Cancel" } else { "Start" };
    let button = egui::Button::new(button_text).fill(if is_processing {
        egui::Color32::from_rgb(200, 50, 50)
    } else {
        egui::Color32::from_rgb(50, 150, 50)
    });

    if ui.add_sized([ui.available_width(), 32.0], button).clicked() {
        if is_processing {
            state.cancel_clicked = true;
        } else {
            state.start_clicked = true;
        }
    }

    state
}
