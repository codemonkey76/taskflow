use egui::{Align2, Color32, Pos2, Rect, Sense, Stroke, Vec2};
use std::path::PathBuf;

pub enum DropZoneResult {
    DroppedFiles(Vec<PathBuf>),
    BrowseClicked,
    None,
}

pub fn render_drop_zone(ui: &mut egui::Ui) -> DropZoneResult {
    let desired_size = Vec2::new(ui.available_width(), 120.0);
    let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click_and_drag());

    // Check if files are being dragged
    let being_dragged = ui.input(|i| !i.raw.hovered_files.is_empty());

    // Visual styling
    let visuals = ui.style().interact(&response);
    let stroke = if being_dragged || response.hovered() {
        Stroke::new(2.0, Color32::from_rgb(100, 149, 237))
    } else {
        Stroke::new(1.5, visuals.bg_stroke.color)
    };

    // Draw drop zone with highlight if dragging
    let fill_color = if being_dragged {
        Color32::from_rgba_premultiplied(100, 149, 237, 30)
    } else {
        Color32::TRANSPARENT
    };

    ui.painter().rect(
        rect,
        8.0,
        fill_color,
        egui::epaint::Stroke::NONE,
        egui::epaint::StrokeKind::Outside,
    );
    ui.painter()
        .rect_stroke(rect, 8.0, stroke, egui::epaint::StrokeKind::Outside);

    // Draw dashed border for visual clarity
    draw_dashed_rect(ui, rect, stroke.color);

    // Text
    let text = "Drop files here or click Browse";
    ui.painter().text(
        rect.center() - Vec2::new(0.0, 10.0),
        Align2::CENTER_CENTER,
        text,
        egui::FontId::proportional(16.0),
        if being_dragged || response.hovered() {
            Color32::from_rgb(100, 149, 237)
        } else {
            Color32::GRAY
        },
    );

    // Browse button centered below text
    let button_size = Vec2::new(120.0, 30.0);
    let button_pos = rect.center() + Vec2::new(-60.0, 20.0);
    let button_rect = Rect::from_min_size(button_pos, button_size);

    let button_response = ui.put(
        button_rect,
        egui::Button::new("Browse Files").fill(Color32::from_rgb(70, 100, 150)),
    );

    // Check for button click first
    if button_response.clicked() {
        return DropZoneResult::BrowseClicked;
    }

    // Then check for file drops
    if let Some(paths) = handle_file_drop(ui) {
        return DropZoneResult::DroppedFiles(paths);
    }

    DropZoneResult::None
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
        // Debug: check what's in hovered and dropped
        if !i.raw.hovered_files.is_empty() {
            println!("Hovered files: {:?}", i.raw.hovered_files);
        }
        if !i.raw.dropped_files.is_empty() {
            println!("Dropped files: {:?}", i.raw.dropped_files);
            let paths = i
                .raw
                .dropped_files
                .iter()
                .filter_map(|file| file.path.clone())
                .collect::<Vec<_>>();

            println!("Extracted paths: {:?}", paths);

            if !paths.is_empty() {
                return Some(paths);
            }
        }
        None
    })
}
