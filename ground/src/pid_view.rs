use eframe::egui;
use std::sync::{Arc, Mutex};

use crate::data::ReceivedData;

#[derive(Clone)]
pub struct PIDControlView {
    // open: bool,
    last_sent_time: std::time::Instant,
    enabled_transmit: bool,
    ui_to_drone_tx: crossbeam_channel::Sender<String>,
    yaw_pid: PIDValues,
    pitch_pid: PIDValues,
    roll_pid: PIDValues,
}

#[derive(Clone)]
struct PIDValues {
    p: f32,
    i: f32,
    d: f32,
}

impl PIDControlView {
    pub fn new(ui_to_drone_tx: crossbeam_channel::Sender<String>) -> Self {
        Self {
            // open: false,
            last_sent_time: std::time::Instant::now(),
            enabled_transmit: false,
            ui_to_drone_tx,
            yaw_pid: PIDValues {
                p: 0.0,
                i: 0.0,
                d: 0.0,
            },
            pitch_pid: PIDValues {
                p: 0.0,
                i: 0.0,
                d: 0.0,
            },
            roll_pid: PIDValues {
                p: 4.6,
                i: 0.1,
                d: 0.0,
            },
        }
    }
}

impl PIDControlView {
    // pub fn ui(&mut self, ui: &mut egui::Ui) {
    //     if ui
    //         .button(if self.open {
    //             "Close PID Control View"
    //         } else {
    //             "Open PID Control View"
    //         })
    //         .clicked()
    //     {
    //         self.open = !self.open;
    //     }
    // }

    pub fn window(&mut self, ctx: &egui::Context, received_data: &Arc<Mutex<ReceivedData>>) {
        egui::Window::new("PID Control")
            // .open(&mut self.open)
            .resizable(true)
            .show(ctx, |ui| {
                let data = received_data.lock().unwrap();

                if self.last_sent_time.elapsed() > std::time::Duration::from_millis(200)
                    && self.enabled_transmit
                {
                    self.last_sent_time = std::time::Instant::now();

                    // convert all ranges from 1000 to 2000

                    self.ui_to_drone_tx
                        .send(format!(
                            "pid->{},{},{}\n",
                            self.roll_pid.p, self.roll_pid.i, self.roll_pid.d
                        ))
                        .expect("Failed to send RC control values");
                }

                ui.heading("PID Control Values");
                ui.add_space(10.0);

                ui.checkbox(&mut self.enabled_transmit, "Enable Transmit");

                ui.group(|ui| {
                    // ui.label(format!(
                    //     "Roll - P: {}, I: {}, D: {}",
                    //     data.serial_data.kp_r as f32, data.serial_data.ki_r as f32, data.serial_data.kd_r as f32
                    // ));

                    self.roll_pid.p = data.serial_data.kp_r;
                    self.roll_pid.i = data.serial_data.ki_r;
                    self.roll_pid.d = data.serial_data.kd_r;

                    // use slider to change the values
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
                });
                // self.pid_ui(ui, "Yaw", &mut self.yaw_pid);
                // ui.add_space(10.0);
                // self.pid_ui(ui, "Pitch", &mut self.pitch_pid);
                // ui.add_space(10.0);
                // self.pid_ui(ui, "Roll", &mut self.roll_pid);
            });
    }

    fn pid_ui(&mut self, ui: &mut egui::Ui, label: &str, pid: &mut PIDValues) {
        ui.group(|ui| {
            ui.label(label);
            ui.horizontal(|ui| {
                ui.label("P:");
                ui.add(egui::DragValue::new(&mut pid.p).speed(0.1));
            });
            ui.horizontal(|ui| {
                ui.label("I:");
                ui.add(egui::DragValue::new(&mut pid.i).speed(0.1));
            });
            ui.horizontal(|ui| {
                ui.label("D:");
                ui.add(egui::DragValue::new(&mut pid.d).speed(0.1));
            });
        });
    }

    // You might want to add a method to get the current PID values
    pub fn get_pid_values(&self) -> (PIDValues, PIDValues, PIDValues) {
        (
            self.yaw_pid.clone(),
            self.pitch_pid.clone(),
            self.roll_pid.clone(),
        )
    }
}
