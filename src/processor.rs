use std::path::PathBuf;
use std::process::Command;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

#[derive(Debug)]
pub enum ProcessorMessage {
    Process {
        index: usize,
        file: PathBuf,
        script: String,
        output_dir: PathBuf,
    },
    Cancel,
}

#[derive(Debug)]
pub enum ProcessorResult {
    Started(usize),
    Success(usize),
    Error(usize, String),
    Cancelled,
}

pub struct Processor {
    tx: Sender<ProcessorMessage>,
    rx: Receiver<ProcessorResult>,
}

impl Processor {
    pub fn new() -> Self {
        let (msg_tx, msg_rx) = std::sync::mpsc::channel();
        let (result_tx, result_rx) = std::sync::mpsc::channel();

        // Spawn worker thread
        thread::spawn(move || {
            Self::worker_thread(msg_rx, result_tx);
        });

        Self {
            tx: msg_tx,
            rx: result_rx,
        }
    }

    pub fn process(
        &self,
        index: usize,
        file: PathBuf,
        script: String,
        output_dir: PathBuf,
    ) -> Result<(), String> {
        self.tx
            .send(ProcessorMessage::Process {
                index,
                file,
                script,
                output_dir,
            })
            .map_err(|e| format!("Failed to send process message: {}", e))
    }

    pub fn cancel(&self) -> Result<(), String> {
        self.tx
            .send(ProcessorMessage::Cancel)
            .map_err(|e| format!("Failed to send cancel message: {}", e))
    }

    pub fn try_recv_result(&self) -> Option<ProcessorResult> {
        self.rx.try_recv().ok()
    }

    fn worker_thread(rx: Receiver<ProcessorMessage>, tx: Sender<ProcessorResult>) {
        let mut cancelled = false;

        while let Ok(msg) = rx.recv() {
            match msg {
                ProcessorMessage::Process {
                    index,
                    file,
                    script,
                    output_dir,
                } => {
                    if cancelled {
                        cancelled = false;
                        continue;
                    }

                    let _ = tx.send(ProcessorResult::Started(index));

                    let result = Self::execute_script(&file, &script, &output_dir);

                    let result_msg = match result {
                        Ok(_) => ProcessorResult::Success(index),
                        Err(e) => ProcessorResult::Error(index, e),
                    };

                    let _ = tx.send(result_msg);
                }
                ProcessorMessage::Cancel => {
                    cancelled = true;
                    let _ = tx.send(ProcessorResult::Cancelled);
                }
            }
        }
    }

    fn execute_script(file: &PathBuf, script: &str, output_dir: &PathBuf) -> Result<(), String> {
        let output = Command::new(script)
            .arg(file.to_str().unwrap_or(""))
            .arg(output_dir.to_str().unwrap_or(""))
            .output()
            .map_err(|e| format!("Failed to execute script: {}", e))?;

        if output.status.success() {
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(format!("Script failed: {}", stderr))
        }
    }
}

impl Default for Processor {
    fn default() -> Self {
        Self::new()
    }
}
