use crate::data::ReceivedData;
use eframe::egui;
use std::sync::{Arc, Mutex};

#[derive( Clone)]
pub struct DroneView {
    open: bool,
}

impl Default for DroneView {
    fn default() -> Self {
        Self { open: false }
    }
}

impl DroneView {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        if ui
            .button(if self.open {
                "Close Drone View"
            } else {
                "Open Drone View"
            })
            .clicked()
        {
            self.open = !self.open;
        }
    }

    pub fn window(&mut self, ctx: &egui::Context, received_data: &Arc<Mutex<ReceivedData>>) {
        egui::Window::new("Drone View")
            // .open(&mut self.open)
            .default_size([400.0, 400.0])
            .resizable(true)
            .show(ctx, |ui| {
                let data = received_data.lock().unwrap();

                // let parts: Vec<&str> = data.serial_data.split(',').collect();

                // let motor_thrusts: Vec<f32> = parts
                //     .iter()
                //     .skip(5)
                //     .take(8)
                //     .filter_map(|&s| s.parse().ok())
                //     .collect();
                let motor_thrusts = vec![
                    data.serial_data.front_right as f32,
                    data.serial_data.back_right as f32,
                    data.serial_data.back_left as f32,
                    data.serial_data.front_left as f32,
                ];

                // Draw drone
                let available_size = ui.available_size();
                let drone_size = available_size.x.min(available_size.y) - 40.0; // Subtract 40 for some padding
                let (response, painter) = ui.allocate_painter(
                    egui::Vec2::new(drone_size, drone_size),
                    egui::Sense::hover(),
                );
                let rect = response.rect;

                // Draw drone body
                let center = rect.center();
                let body_radius = drone_size / 8.0;
                painter.circle_filled(center, body_radius, egui::Color32::LIGHT_GRAY);

                // Draw arms
                let arm_length = drone_size / 2.5;
                let arm_width = drone_size / 30.0;
                let arm_color = egui::Color32::DARK_GRAY;

                for i in 0..4 {
                    let angle = std::f32::consts::PI / 2.0 * i as f32;
                    let end = center
                        + egui::Vec2::new(angle.cos() * arm_length, angle.sin() * arm_length);
                    painter.line_segment([center, end], (arm_width, arm_color));
                }

                // Draw motors and display thrust values
                let motor_radius = drone_size / 10.0;
                let motor_positions = [
                    center + egui::Vec2::new(arm_length, 0.0),
                    center + egui::Vec2::new(0.0, arm_length),
                    center + egui::Vec2::new(-arm_length, 0.0),
                    center + egui::Vec2::new(0.0, -arm_length),
                ];

                for (i, pos) in motor_positions.iter().enumerate() {
                    painter.circle_filled(*pos, motor_radius, egui::Color32::DARK_BLUE);
                    let thrust = motor_thrusts.get(i).cloned().unwrap_or(0.0);
                    painter.text(
                        *pos,
                        egui::Align2::CENTER_CENTER,
                        format!("{:.1}", thrust),
                        egui::FontId::proportional(14.0),
                        egui::Color32::WHITE,
                    );
                }

                // Draw forward direction indicator
                let forward_length = drone_size / 5.0;
                let forward_end = center + egui::Vec2::new(0.0, -forward_length);
                painter.arrow(center, forward_end - center, (2.0, egui::Color32::RED));

                // Display Aruco IDs
                ui.label(format!("Aruco IDs: {:?}", data.aruco_ids));
            });
    }
}
