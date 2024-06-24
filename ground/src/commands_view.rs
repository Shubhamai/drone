use crate::data::ReceivedData;
use crossbeam_channel::{unbounded, Receiver, Sender};
use eframe::egui::{self, RichText};
use epaint::Color32;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct CommandsView {
    open: bool,
    pub ui_to_drone_tx: Sender<String>,
    pub ui_to_drone_rx: Receiver<String>,
    pub drone_to_ui_tx: Sender<String>,
    pub drone_to_ui_rx: Receiver<String>,
}

impl CommandsView {
    pub fn new(
        ui_to_drone_tx: Sender<String>,
        ui_to_drone_rx: Receiver<String>,
        drone_to_ui_tx: Sender<String>,
        drone_to_ui_rx: Receiver<String>,
    ) -> Self {
        Self {
            open: false,
            ui_to_drone_tx,
            ui_to_drone_rx,
            drone_to_ui_tx,
            drone_to_ui_rx,
        }
    }
}

impl CommandsView {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        if ui
            .button(if self.open { "Close" } else { "Open" })
            .clicked()
        {
            self.open = !self.open;
        }
    }

    pub fn window(&mut self, ctx: &egui::Context, received_data: &Arc<Mutex<ReceivedData>>) {
        egui::Window::new("Commands")
            // .open(&mut self.open)
            .resizable(true)
            .default_size([400.0, 600.0])
            .show(ctx, |ui| {
                if ui
                    .button(RichText::new("Master Abort").size(32.).color(Color32::RED))
                    .clicked()
                {
                    self.ui_to_drone_tx
                        .send("abort".to_string())
                        .expect("Failed to send abort message");
                }
            });
    }
}
