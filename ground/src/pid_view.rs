use eframe::egui;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::data::ReceivedData;

#[derive(Clone)]
pub struct PIDControlView {
    last_sent_time: Instant,
    enabled_transmit: bool,
    ui_to_drone_tx: crossbeam_channel::Sender<String>,
    roll_pid: PIDValues,
    pitch_pid: PIDValues,
    last_sent_pids: Option<(PIDValues, PIDValues)>,}

#[derive(Clone, PartialEq)]
struct PIDValues {
    p: f32,
    i: f32,
    d: f32,
}

impl PIDControlView {
    pub fn new(ui_to_drone_tx: crossbeam_channel::Sender<String>) -> Self {
        Self {
            last_sent_time: Instant::now(),
            enabled_transmit: false,
            ui_to_drone_tx,
            roll_pid: PIDValues {
                p: 1.2,
                i: 1.0,
                d: 4.0,
            },
            pitch_pid: PIDValues {
                p: 1.2,
                i: 1.0,
                d: 4.0,
            },
            last_sent_pids: None,
        }
    }

    pub fn window(&mut self, ctx: &egui::Context, received_data: &Arc<Mutex<ReceivedData>>) {
        egui::Window::new("PID Control")
            .resizable(true)
            .show(ctx, |ui| {
                let data = received_data.lock().unwrap();

                ui.heading("PID Control Values");
                ui.add_space(10.0);

                ui.checkbox(&mut self.enabled_transmit, "Enable Transmit");

                ui.group(|ui| {
                    self.roll_pid.p = data.serial_data.kp_r;
                    self.roll_pid.i = data.serial_data.ki_r;
                    self.roll_pid.d = data.serial_data.kd_r;

                    ui.label("Roll PID");

                    // UI for PID values (unchanged)
                    ui.horizontal(|ui| {
                        ui.label("P:");
                        ui.spacing_mut().slider_width = 300.0;
                        ui.add(
                            egui::Slider::new(&mut self.roll_pid.p, 0.0..=40.0)
                                .step_by(0.001)
                                .smart_aim(false)
                                .fixed_decimals(3),
                        );
                    });

                    ui.horizontal(|ui| {
                        ui.label("I:");
                        ui.spacing_mut().slider_width = 300.0;
                        ui.add(
                            egui::Slider::new(&mut self.roll_pid.i, 0.0..=40.0)
                                .step_by(0.001)
                                .smart_aim(false)
                                .fixed_decimals(3),
                        );
                    });

                    ui.horizontal(|ui| {
                        ui.label("D:");
                        ui.spacing_mut().slider_width = 300.0;
                        ui.add(
                            egui::Slider::new(&mut self.roll_pid.d, 0.0..=40.0)
                                .step_by(0.001)
                                .smart_aim(false)
                                .fixed_decimals(3),
                        );
                    });

                    ui.add_space(10.0);

                    self.pitch_pid.p = data.serial_data.kp_p;
                    self.pitch_pid.i = data.serial_data.ki_p;
                    self.pitch_pid.d = data.serial_data.kd_p;

                    ui.horizontal(|ui| {
                        ui.label("Pitch PID");
                    });

                    // UI for PID values (unchanged)
                    ui.horizontal(|ui| {
                        ui.label("P:");
                        ui.spacing_mut().slider_width = 300.0;
                        ui.add(
                            egui::Slider::new(&mut self.pitch_pid.p, 0.0..=40.0)
                                .step_by(0.001)
                                .smart_aim(false)
                                .fixed_decimals(3),
                        );
                    });

                    ui.horizontal(|ui| {
                        ui.label("I:");
                        ui.spacing_mut().slider_width = 300.0;
                        ui.add(
                            egui::Slider::new(&mut self.pitch_pid.i, 0.0..=40.0)
                                .step_by(0.001)
                                .smart_aim(false)
                                .fixed_decimals(3),
                        );
                    });

                    ui.horizontal(|ui| {
                        ui.label("D:");
                        ui.spacing_mut().slider_width = 300.0;
                        ui.add(
                            egui::Slider::new(&mut self.pitch_pid.d, 0.0..=40.0)
                                .step_by(0.001)
                                .smart_aim(false)
                                .fixed_decimals(3),
                        );
                    });

                    ui.add_space(10.0);
                });

                // Check if we should send an update
                if self.enabled_transmit {
                    let current_time = Instant::now();
                    let time_since_last_send = current_time.duration_since(self.last_sent_time);

                    if time_since_last_send >= Duration::from_millis(200)
                        // && self.last_sent_pid.as_ref() != Some(&self.roll_pid)
                        && self.last_sent_pids.as_ref() != Some(&(self.roll_pid.clone(), self.pitch_pid.clone()))
                    {
                        self.ui_to_drone_tx
                            .send(format!(
                                "pid->{},{},{},{},{},{}\n",
                                self.roll_pid.p,
                                self.roll_pid.i,
                                self.roll_pid.d,
                                self.pitch_pid.p,
                                self.pitch_pid.i,
                                self.pitch_pid.d
                            ))
                            .expect("Failed to send PID control values");

                        self.last_sent_time = current_time;
                        // self.last_sent_pid = Some(self.roll_pid.clone());
                        self.last_sent_pids = Some((self.roll_pid.clone(), self.pitch_pid.clone()));

                    }
                }
            });
    }
}
