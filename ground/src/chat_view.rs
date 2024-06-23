use crate::data::ReceivedData;
use crossbeam_channel::{unbounded, Receiver, Sender};
use eframe::egui;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct ChatView {
    open: bool,
    input: String,
    messages: Vec<ChatMessage>,
    tx: Sender<String>,
    rx: Receiver<String>,
}

#[derive(Clone)]
struct ChatMessage {
    text: String,
    is_user: bool,
}

impl Default for ChatView {
    fn default() -> Self {
        let (tx, rx) = unbounded();
        Self {
            open: false,
            input: String::new(),
            messages: Vec::new(),
            tx,
            rx,
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
                while let Ok(message) = self.rx.try_recv() {
                    self.messages.push(ChatMessage {
                        text: message,
                        is_user: false,
                    });
                }

                // Display messages
                egui::ScrollArea::vertical()
                    .stick_to_bottom(true)
                    .show(ui, |ui| {
                        for message in &self.messages {
                            let (text, color) = if message.is_user {
                                ("You: ", egui::Color32::LIGHT_BLUE)
                            } else {
                                ("Drone: ", egui::Color32::LIGHT_GREEN)
                            };
                            ui.colored_label(color, format!("{}{}", text, message.text));
                        }
                    });

                // Input field and send button
                ui.horizontal(|ui| {
                    let input = ui.text_edit_singleline(&mut self.input);
                    let enter_pressed = ui.input(|i| i.key_pressed(egui::Key::Enter));
                    if ui.button("Send").clicked() || (input.lost_focus() && enter_pressed) {
                        if !self.input.is_empty() {
                            // Send the message to the WebSocket thread
                            if let Err(e) = self.tx.send(self.input.clone()) {
                                eprintln!("Failed to send message: {}", e);
                            }
                            // Add the message to the chat
                            self.messages.push(ChatMessage {
                                text: self.input.clone(),
                                is_user: true,
                            });
                            self.input.clear();
                        }
                    }
                });
            });
    }

    pub fn get_receiver(&self) -> Receiver<String> {
        self.rx.clone()
    }

    pub fn get_sender(&self) -> Sender<String> {
        self.tx.clone()
    }
}
