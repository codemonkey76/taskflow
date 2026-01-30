use crate::config::Config;
use crate::logger::Logger;
use crate::processor::{Processor, ProcessorResult};
use crate::queue::{ItemState, Queue};
use crate::ui::{
    ControlsState, QueueListInteraction, render_controls, render_drop_zone, render_queue_list,
};

pub struct TaskFlowApp {
    queue: Queue,
    config: Config,
    logger: Logger,
    processor: Processor,

    // UI state
    is_processing: bool,
    status_message: String,
    last_clicked_index: Option<usize>,

    // Available scripts (TODO: load from config/directory)
    available_scripts: Vec<String>,
}

impl TaskFlowApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let config = Config::load();
        let logger = Logger::new(config.output_directory.clone(), config.logging_enabled);

        Self {
            queue: Queue::new(),
            config,
            logger,
            processor: Processor::new(),
            is_processing: false,
            status_message: "Ready".to_string(),
            last_clicked_index: None,
            available_scripts: vec![
                "/path/to/script1.sh".to_string(),
                "/path/to/script2.sh".to_string(),
            ],
        }
    }

    fn handle_file_drops(&mut self, paths: Vec<std::path::PathBuf>) {
        self.queue.add_multiple(paths);
    }

    fn handle_controls(&mut self, state: ControlsState) {
        if state.script_changed || state.output_changed {
            let _ = self.config.save();
        }

        if state.logging_changed {
            self.logger.update(
                self.config.output_directory.clone(),
                self.config.logging_enabled,
            );
            let _ = self.config.save();
        }

        if state.start_clicked {
            self.start_processing();
        }

        if state.cancel_clicked {
            self.cancel_processing();
        }
    }

    fn handle_queue_interaction(&mut self, interaction: QueueListInteraction) {
        if interaction.delete_pressed {
            self.queue.remove_selected();
        }

        if let Some(clicked_index) = interaction.clicked_index {
            if interaction.ctrl_held {
                self.queue.toggle_select(clicked_index);
                self.last_clicked_index = Some(clicked_index);
            } else if interaction.shift_held {
                if let Some(last_index) = self.last_clicked_index {
                    self.queue.clear_selection();
                    self.queue.select_range(last_index, clicked_index);
                } else {
                    self.queue.clear_selection();
                    self.queue.select(clicked_index);
                }
                self.last_clicked_index = Some(clicked_index);
            } else {
                self.queue.clear_selection();
                self.queue.select(clicked_index);
                self.last_clicked_index = Some(clicked_index);
            }
        }

        if let Some(target_index) = interaction.drag_target {
            self.queue.move_selected(target_index);
        }
    }

    fn start_processing(&mut self) {
        if self.config.selected_script.is_none() {
            self.status_message = "Error: No script selected".to_string();
            return;
        }

        if self.config.output_directory.is_none() {
            self.status_message = "Error: No output directory selected".to_string();
            return;
        }

        if self.queue.is_empty() {
            self.status_message = "Error: Queue is empty".to_string();
            return;
        }

        self.is_processing = true;
        self.process_next_item();
    }

    fn process_next_item(&mut self) {
        if let Some(index) = self.queue.get_next_pending() {
            let item = &self.queue.items()[index];
            let script = self.config.selected_script.clone().unwrap();
            let output_dir = self.config.output_directory.clone().unwrap();

            self.logger.log_start(&item.filename(), &script);

            if let Err(e) = self
                .processor
                .process(index, item.path.clone(), script, output_dir)
            {
                self.logger.log_error(&item.filename(), &e);
                self.queue.set_state(index, ItemState::Error(e));
                self.process_next_item();
            }
        } else {
            self.is_processing = false;
            self.status_message = "All items processed".to_string();
        }
    }

    fn cancel_processing(&mut self) {
        let _ = self.processor.cancel();
        self.is_processing = false;
        self.status_message = "Processing cancelled".to_string();
    }

    fn poll_processor(&mut self) {
        while let Some(result) = self.processor.try_recv_result() {
            match result {
                ProcessorResult::Started(index) => {
                    self.queue.set_state(index, ItemState::Processing);
                    let filename = self.queue.items()[index].filename();
                    self.status_message = format!("Processing: {}", filename);
                }
                ProcessorResult::Success(index) => {
                    let filename = self.queue.items()[index].filename();
                    self.logger.log_success(&filename);
                    self.queue.set_state(index, ItemState::Completed);
                    self.queue.remove_completed();
                    self.process_next_item();
                }
                ProcessorResult::Error(index, error) => {
                    let filename = self.queue.items()[index].filename();
                    self.logger.log_error(&filename, &error);
                    self.queue.set_state(index, ItemState::Error(error));
                    self.process_next_item();
                }
                ProcessorResult::Cancelled => {
                    self.is_processing = false;
                    self.status_message = "Cancelled".to_string();
                }
            }
        }
    }
}

impl eframe::App for TaskFlowApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Poll for processor results
        self.poll_processor();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("TaskFlow");
            ui.separator();

            // Drop zone
            if let Some(paths) = render_drop_zone(ui) {
                self.handle_file_drops(paths);
            }

            ui.add_space(10.0);

            // Controls
            let controls_state = render_controls(
                ui,
                &self.available_scripts,
                &mut self.config.selected_script,
                &mut self.config.output_directory,
                &mut self.config.logging_enabled,
                self.is_processing,
            );
            self.handle_controls(controls_state);

            ui.separator();

            // Queue
            ui.label(format!("Queue ({} items)", self.queue.len()));
            let queue_interaction = render_queue_list(ui, self.queue.items_mut());
            self.handle_queue_interaction(queue_interaction);

            ui.separator();

            // Status bar
            ui.horizontal(|ui| {
                ui.label("Status:");
                ui.label(&self.status_message);
            });
        });

        // Request repaint to keep UI responsive
        ctx.request_repaint();
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        let _ = self.config.save();
    }
}
