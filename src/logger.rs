use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

pub struct Logger {
    log_file: Option<PathBuf>,
    enabled: bool,
}

impl Logger {
    pub fn new(output_dir: Option<PathBuf>, enabled: bool) -> Self {
        let log_file = if enabled {
            output_dir.map(|dir| {
                let timestamp = Local::now().format("%Y%m%d_%H%M%S");
                dir.join(format!("taskflow_{}.log", timestamp))
            })
        } else {
            None
        };

        Self { log_file, enabled }
    }

    pub fn log(&self, message: &str) {
        if !self.enabled || self.log_file.is_none() {
            return;
        }

        if let Some(ref path) = self.log_file {
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
            let log_line = format!("[{}] {}\n", timestamp, message);

            if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) {
                let _ = file.write_all(log_line.as_bytes());
            }
        }
    }

    pub fn log_start(&self, filename: &str, script: &str) {
        self.log(&format!("Starting: {} with script: {}", filename, script));
    }

    pub fn log_success(&self, filename: &str) {
        self.log(&format!("Success: {}", filename));
    }

    pub fn log_error(&self, filename: &str, error: &str) {
        self.log(&format!("Error: {} - {}", filename, error));
    }

    pub fn update(&mut self, output_dir: Option<PathBuf>, enabled: bool) {
        self.enabled = enabled;
        self.log_file = if enabled {
            output_dir.map(|dir| {
                let timestamp = Local::now().format("%Y%m%d_%H%M%S");
                dir.join(format!("taskflow_{}.log", timestamp))
            })
        } else {
            None
        };
    }
}
