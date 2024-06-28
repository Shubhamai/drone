use eframe::egui;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct NoteEditorView {
    open: bool,
    content: String,
    file_path: PathBuf,
    status_message: String,
    last_save: Instant,
    last_reload: Instant,
    autosave_interval: Duration,
    autoreload_interval: Duration,
    last_modified: Option<std::time::SystemTime>,
}

impl NoteEditorView {
    pub fn new(file_path: PathBuf) -> Self {
        let content = fs::read_to_string(&file_path).unwrap_or_else(|_| String::new());
        let last_modified = fs::metadata(&file_path).ok().and_then(|m| m.modified().ok());
        Self {
            open: false,
            content,
            file_path,
            status_message: String::new(),
            last_save: Instant::now(),
            last_reload: Instant::now(),
            autosave_interval: Duration::from_secs(30), // Autosave every 30 seconds
            autoreload_interval: Duration::from_secs(5), // Check for changes every 5 seconds
            last_modified,
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        if ui
            .button(if self.open { "Close Note Editor" } else { "Open Note Editor" })
            .clicked()
        {
            self.open = !self.open;
        }
    }

    pub fn window(&mut self, ctx: &egui::Context) {
        egui::Window::new("Note Editor")
            // .open(&mut self.open)
            .resizable(true)
            .max_size([600.0, 400.0])
            .show(ctx, |ui| {
                let mut content_changed = false;

                // Text area for editing
                ui.add_sized(
                    ui.available_size(),
                    egui::TextEdit::multiline(&mut self.content).desired_width(f32::INFINITY),
                );

                // Check if content has changed
                if ui.input(|i| i.key_pressed(egui::Key::Enter) || i.key_pressed(egui::Key::Space)) {
                    content_changed = true;
                }

                ui.horizontal(|ui| {
                    if ui.button("Save").clicked() {
                        self.save_file();
                    }

                    if ui.button("Reload").clicked() {
                        self.reload_file();
                    }
                });

                // Autosave
                if content_changed || self.last_save.elapsed() >= self.autosave_interval {
                    self.save_file();
                    self.last_save = Instant::now();
                }

                // Autoreload
                if self.last_reload.elapsed() >= self.autoreload_interval {
                    self.check_and_reload();
                    self.last_reload = Instant::now();
                }

                // Display status message
                ui.label(&self.status_message);
            });
    }

    fn save_file(&mut self) {
        match fs::File::create(&self.file_path) {
            Ok(mut file) => {
                if let Err(e) = file.write_all(self.content.as_bytes()) {
                    self.status_message = format!("Failed to write to file: {}", e);
                } else {
                    self.status_message = "File saved successfully".to_string();
                    self.last_modified = fs::metadata(&self.file_path).ok().and_then(|m| m.modified().ok());
                }
            }
            Err(e) => {
                self.status_message = format!("Failed to create file: {}", e);
            }
        }
    }

    fn reload_file(&mut self) {
        match fs::read_to_string(&self.file_path) {
            Ok(content) => {
                self.content = content;
                self.status_message = "File reloaded successfully".to_string();
                self.last_modified = fs::metadata(&self.file_path).ok().and_then(|m| m.modified().ok());
            }
            Err(e) => {
                self.status_message = format!("Failed to read file: {}", e);
            }
        }
    }

    fn check_and_reload(&mut self) {
        if let Ok(metadata) = fs::metadata(&self.file_path) {
            if let Ok(modified) = metadata.modified() {
                if self.last_modified.map_or(true, |last| modified > last) {
                    self.reload_file();
                    self.status_message = "File automatically reloaded due to external changes".to_string();
                }
            }
        }
    }
}