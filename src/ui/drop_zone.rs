use egui::{Align2, Color32, Pos2, Rect, Sense, Stroke, Vec2};

pub fn render_drop_zone(ui: &mut egui::Ui) -> Option<Vec<std::path::PathBuf>> {
    let desired_size = Vec2::new(ui.available_width(), 120.0);
    let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click_and_drag());

    // Visual styling
    let visuals = ui.style().interact(&response);
    let stroke = if response.hovered() {
        Stroke::new(2.0, Color32::from_rgb(100, 149, 237))
    } else {
        Stroke::new(1.5, visuals.bg_stroke.color)
    };

    // Draw drop zone
    ui.painter()
        .rect_stroke(rect, 8.0, stroke, egui::epaint::StrokeKind::Outside);

    // Draw dashed border for visual clarity
    draw_dashed_rect(ui, rect, stroke.color);

    // Text
    let text = "Drop files here";
    ui.painter().text(
        rect.center(),
        Align2::CENTER_CENTER,
        text,
        egui::FontId::proportional(16.0),
        if response.hovered() {
            Color32::from_rgb(100, 149, 237)
        } else {
            Color32::GRAY
        },
    );

    // Handle file drops
    handle_file_drop(ui)
}

fn draw_dashed_rect(ui: &mut egui::Ui, rect: Rect, color: Color32) {
    let dash_length = 10.0;
    let gap_length = 5.0;
    let stroke = Stroke::new(1.5, color);

    // Top edge
    draw_dashed_line(
        ui,
        rect.left_top(),
        rect.right_top(),
        dash_length,
        gap_length,
        stroke,
    );
    // Right edge
    draw_dashed_line(
        ui,
        rect.right_top(),
        rect.right_bottom(),
        dash_length,
        gap_length,
        stroke,
    );
    // Bottom edge
    draw_dashed_line(
        ui,
        rect.right_bottom(),
        rect.left_bottom(),
        dash_length,
        gap_length,
        stroke,
    );
    // Left edge
    draw_dashed_line(
        ui,
        rect.left_bottom(),
        rect.left_top(),
        dash_length,
        gap_length,
        stroke,
    );
}

fn draw_dashed_line(
    ui: &mut egui::Ui,
    start: Pos2,
    end: Pos2,
    dash_length: f32,
    gap_length: f32,
    stroke: Stroke,
) {
    let dir = end - start;
    let length = dir.length();
    let dir_normalized = dir / length;

    let mut current_pos = 0.0;
    let segment_length = dash_length + gap_length;

    while current_pos < length {
        let dash_start = start + dir_normalized * current_pos;
        let dash_end_pos = (current_pos + dash_length).min(length);
        let dash_end = start + dir_normalized * dash_end_pos;

        ui.painter().line_segment([dash_start, dash_end], stroke);
        current_pos += segment_length;
    }
}

fn handle_file_drop(ui: &egui::Ui) -> Option<Vec<std::path::PathBuf>> {
    ui.input(|i| {
        if !i.raw.dropped_files.is_empty() {
            let paths = i
                .raw
                .dropped_files
                .iter()
                .filter_map(|file| file.path.clone())
                .collect::<Vec<_>>();

            if !paths.is_empty() {
                return Some(paths);
            }
        }
        None
    })
}
