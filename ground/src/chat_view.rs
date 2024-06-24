use crate::data::ReceivedData;
use chrono::Local;
use crossbeam_channel::{unbounded, Receiver, Sender};
use eframe::egui;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct ChatView {
    open: bool,
    input: String,
    messages: Vec<ChatMessage>,
    pub ui_to_drone_tx: Sender<String>,
    pub ui_to_drone_rx: Receiver<String>,
    pub drone_to_ui_tx: Sender<String>,
    pub drone_to_ui_rx: Receiver<String>,
    scroll_to_bottom: bool,
}

#[derive(Clone)]
struct ChatMessage {
    text: String,
    is_user: bool,
    timestamp: String,
}

impl ChatView {
    pub fn new(
        ui_to_drone_tx: Sender<String>,
        ui_to_drone_rx: Receiver<String>,
        drone_to_ui_tx: Sender<String>,
        drone_to_ui_rx: Receiver<String>,
    ) -> Self {
        Self {
            open: false,
            input: String::new(),
            messages: Vec::new(),
            ui_to_drone_tx,
            ui_to_drone_rx,
            drone_to_ui_tx,
            drone_to_ui_rx,
            scroll_to_bottom: false,
        }
    }
}

impl ChatView {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        if ui
            .button(if self.open { "Close Chat" } else { "Open Chat" })
            .clicked()
        {
            self.open = !self.open;
        }
    }

    pub fn window(&mut self, ctx: &egui::Context, received_data: &Arc<Mutex<ReceivedData>>) {
        egui::Window::new("Drone Chat")
            // .open(&mut self.open)
            .resizable(true)
            .default_size([400.0, 600.0])
            .show(ctx, |ui| {
                // Check for new messages from the WebSocket
                while let Ok(message) = self.drone_to_ui_rx.try_recv() {
                    self.messages.push(ChatMessage {
                        text: message,
                        is_user: false,
                        timestamp: Local::now().format("%H:%M:%S").to_string(),
                    });
                    self.scroll_to_bottom = true;
                }

                // Display messages
                let scroll_area = egui::ScrollArea::vertical().stick_to_bottom(true);
                scroll_area.show(ui, |ui| {
                    for message in &self.messages {
                        let (text, color) = if message.is_user {
                            ("You: ", egui::Color32::LIGHT_BLUE)
                        } else {
                            ("Drone: ", egui::Color32::LIGHT_GREEN)
                        };
                        ui.horizontal(|ui| {
                            ui.colored_label(
                                color,
                                format!("[{}] {}{}", message.timestamp, text, message.text),
                            );
                        });
                    }
                });

                // Input field and send button
                ui.horizontal(|ui| {
                    let input = ui.text_edit_singleline(&mut self.input);
                    let enter_pressed = ui.input(|i| i.key_pressed(egui::Key::Enter));
                    if ui.button("Send").clicked() || (input.lost_focus() && enter_pressed) {
                        if !self.input.is_empty() {
                            // Send the message to the WebSocket thread
                            if let Err(e) = self.ui_to_drone_tx.send(self.input.clone()) {
                                eprintln!("Failed to send message: {}", e);
                            }
                            // Add the message to the chat
                            self.messages.push(ChatMessage {
                                text: self.input.clone(),
                                is_user: true,
                                timestamp: Local::now().format("%H:%M:%S").to_string(),
                            });
                            self.input.clear();
                            self.scroll_to_bottom = true;
                        }
                    }
                });

                // Clear messages button
                if ui.button("Clear Messages").clicked() {
                    self.messages.clear();
                }

                // Scroll to bottom if needed
                if self.scroll_to_bottom {
                    ui.scroll_to_cursor(Some(egui::Align::BOTTOM));
                    self.scroll_to_bottom = false;
                }
            });
    }
}
