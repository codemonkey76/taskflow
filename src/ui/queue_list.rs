use crate::queue::{ItemState, QueueItem};
use egui::{Color32, Ui};

pub struct QueueListInteraction {
    pub clicked_index: Option<usize>,
    pub ctrl_held: bool,
    pub shift_held: bool,
    pub delete_pressed: bool,
    pub drag_target: Option<usize>,
}

impl Default for QueueListInteraction {
    fn default() -> Self {
        Self {
            clicked_index: None,
            ctrl_held: false,
            shift_held: false,
            delete_pressed: false,
            drag_target: None,
        }
    }
}

pub fn render_queue_list(ui: &mut Ui, items: &mut [QueueItem]) -> QueueListInteraction {
    let mut interaction = QueueListInteraction::default();

    // Check for modifier keys and delete
    ui.input(|i| {
        interaction.ctrl_held = i.modifiers.ctrl;
        interaction.shift_held = i.modifiers.shift;
        interaction.delete_pressed = i.key_pressed(egui::Key::Delete);
    });

    egui::ScrollArea::vertical()
        .id_salt("queue_scroll")
        .max_height(ui.available_height() - 60.0)
        .show(ui, |ui| {
            for (index, item) in items.iter_mut().enumerate() {
                let item_interaction = render_queue_item(ui, index, item);

                if let Some(clicked) = item_interaction.clicked {
                    interaction.clicked_index = Some(clicked);
                }

                if let Some(target) = item_interaction.drag_target {
                    interaction.drag_target = Some(target);
                }
            }
        });

    interaction
}

struct ItemInteraction {
    clicked: Option<usize>,
    drag_target: Option<usize>,
}

fn render_queue_item(ui: &mut Ui, index: usize, item: &mut QueueItem) -> ItemInteraction {
    let mut interaction = ItemInteraction {
        clicked: None,
        drag_target: None,
    };

    let is_locked = item.is_locked();

    // Determine colors based on state
    let (bg_color, text_color) = match &item.state {
        ItemState::Pending if item.selected => (Color32::from_rgb(70, 100, 150), Color32::WHITE),
        ItemState::Pending => (Color32::from_rgb(40, 40, 40), Color32::LIGHT_GRAY),
        ItemState::Processing => (Color32::from_rgb(60, 60, 60), Color32::DARK_GRAY),
        ItemState::Completed => (Color32::from_rgb(50, 80, 50), Color32::LIGHT_GRAY),
        ItemState::Error(_) => (Color32::from_rgb(100, 40, 40), Color32::LIGHT_GRAY),
    };

    let response = ui.horizontal(|ui| {
        ui.style_mut().visuals.widgets.inactive.weak_bg_fill = bg_color;
        ui.style_mut().visuals.widgets.hovered.weak_bg_fill =
            lighten_color(bg_color, if is_locked { 1.1 } else { 1.2 });

        let frame = egui::Frame::default()
            .fill(bg_color)
            .inner_margin(8.0)
            .corner_radius(4.0);

        frame.show(ui, |ui| {
            ui.set_width(ui.available_width());

            // Status indicator
            let status_text = match &item.state {
                ItemState::Pending => "⏸",
                ItemState::Processing => "▶",
                ItemState::Completed => "✓",
                ItemState::Error(_) => "✗",
            };
            ui.colored_label(text_color, status_text);

            // Filename
            ui.colored_label(text_color, item.filename());

            // Error message if present
            if let ItemState::Error(ref msg) = item.state {
                ui.colored_label(Color32::from_rgb(255, 100, 100), format!(" - {}", msg));
            }
        })
    });

    // Handle interactions only if not locked
    if !is_locked {
        if response.response.clicked() {
            interaction.clicked = Some(index);
        }

        // Handle drag and drop
        if response.response.dragged() {
            ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::Grabbing);
        }

        if response.response.drag_stopped() {
            // Determine drop target based on mouse position
            if let Some(_pos) = ui.input(|i| i.pointer.interact_pos()) {
                // Simple approach: find which item the mouse is over
                interaction.drag_target = Some(index);
            }
        }
    }

    interaction
}

fn lighten_color(color: Color32, factor: f32) -> Color32 {
    Color32::from_rgb(
        (color.r() as f32 * factor).min(255.0) as u8,
        (color.g() as f32 * factor).min(255.0) as u8,
        (color.b() as f32 * factor).min(255.0) as u8,
    )
}
