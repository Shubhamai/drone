use crate::data::ReceivedData;
use crossbeam_channel::{unbounded, Receiver, Sender};
use eframe::egui::{self, RichText};
use epaint::Color32;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct CommandsView {
    open: bool,
    motor_enabled: bool,
    last_sent_time: std::time::Instant,
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
            motor_enabled: false,
            last_sent_time: std::time::Instant::now(),
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
        if self.last_sent_time.elapsed() > std::time::Duration::from_millis(50) {
            self.last_sent_time = std::time::Instant::now();

            if self.motor_enabled {
                self.ui_to_drone_tx
                    .send("command->enable_motors".to_string())
                    .expect("Failed to send enable motors message");
            }
        }

        egui::Window::new("Commands")
            // .open(&mut self.open)
            .resizable(true)
            .default_size([400.0, 600.0])
            .show(ctx, |ui| {
                if ui
                    .button(RichText::new("Master Arm").size(32.).color(Color32::GREEN))
                    .clicked()
                {
                    self.ui_to_drone_tx
                        .send("command->arm".to_string())
                        .expect("Failed to send arm message");
                }

                if ui
                    .button(
                        RichText::new(format!("Motors Enabled: {}", self.motor_enabled))
                            .size(32.)
                            .color(Color32::GREEN),
                    )
                    .clicked()
                {
                    self.motor_enabled = !self.motor_enabled;
                }

                if ui
                    .button(RichText::new("Master Abort").size(32.).color(Color32::RED))
                    .clicked()
                {
                    self.ui_to_drone_tx
                        .send("command->abort".to_string())
                        .expect("Failed to send abort message");
                }

                // button to enable/disable motors
                if ui
                    .button(RichText::new("Reboot").size(32.).color(Color32::RED))
                    .clicked()
                {
                    self.ui_to_drone_tx
                        .send("command->reboot".to_string())
                        .expect("Failed to send reboot message");
                }
            });
    }
}
