use crate::data::ReceivedData;
use crossbeam_channel::{Receiver, Sender};
use eframe::egui::{self, Key, RichText};
use epaint::Color32;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct CommandsView {
    open: bool,
    motor_enabled: bool,
    last_sent_time: Instant,
    last_motor_state: bool,
    ui_to_drone_tx: Sender<String>,
    ui_to_drone_rx: Receiver<String>,
    drone_to_ui_tx: Sender<String>,
    drone_to_ui_rx: Receiver<String>,
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
            last_sent_time: Instant::now(),
            last_motor_state: false,
            ui_to_drone_tx,
            ui_to_drone_rx,
            drone_to_ui_tx,
            drone_to_ui_rx,
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        if ui
            .button(if self.open { "Close" } else { "Open" })
            .clicked()
        {
            self.open = !self.open;
        }
    }

    pub fn window(&mut self, ctx: &egui::Context, received_data: &Arc<Mutex<ReceivedData>>) {
        let current_time = Instant::now();
        let time_since_last_send = current_time.duration_since(self.last_sent_time);

        if time_since_last_send >= Duration::from_millis(50)
            // && self.motor_enabled != self.last_motor_state
        {
            self.last_sent_time = current_time;
            self.last_motor_state = self.motor_enabled;

            if self.motor_enabled {
                self.ui_to_drone_tx
                    .send("command->enable_motors".to_string())
                    .expect("Failed to send enable motors message");
            }
        }

        egui::Window::new("Commands")
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
                            .color(if self.motor_enabled {
                                Color32::GREEN
                            } else {
                                Color32::RED
                            }),
                    )
                    .clicked()
                {
                    self.motor_enabled = !self.motor_enabled;
                }

                ctx.input(|i| {
                    if i.key_down(Key::Enter) {
                        self.motor_enabled = false;
                        // self.ui_to_drone_tx
                        //     .send("command->abort".to_string())
                        //     .expect("Failed to send abort message");
                    }
                });

                // esc to abort
                ctx.input(|i| {
                    if i.key_down(Key::Escape) {
                        self.ui_to_drone_tx
                            .send("command->abort".to_string())
                            .expect("Failed to send abort message");
                    }
                });

                // shift + r to reboot
                ctx.input(|i| {
                    if i.modifiers.shift && i.key_down(Key::R) {
                        self.motor_enabled = false;
                        self.ui_to_drone_tx
                            .send("command->reboot".to_string())
                            .expect("Failed to send reboot message");
                    }
                });
                

                if ui
                    .button(RichText::new("Master Abort").size(32.).color(Color32::RED))
                    .clicked()
                {
                    self.ui_to_drone_tx
                        .send("command->abort".to_string())
                        .expect("Failed to send abort message");
                }

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
