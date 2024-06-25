use crate::data::ReceivedData;
use eframe::egui;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct DroneView {
    open: RefCell<bool>,
}

impl Default for DroneView {
    fn default() -> Self {
        Self {
            open: RefCell::new(false),
        }
    }
}

impl DroneView {
    pub fn ui(&self, ui: &mut egui::Ui) {
        let mut open = self.open.borrow_mut();
        if ui
            .button(if *open {
                "Close Drone View"
            } else {
                "Open Drone View"
            })
            .clicked()
        {
            *open = !*open;
        }
    }

    pub fn window(&self, ctx: &egui::Context, received_data: &Arc<Mutex<ReceivedData>>) {
        let mut open = self.open.borrow_mut();
        egui::Window::new("Drone View")
            // .open(&mut *open)
            .default_size([600.0, 400.0])
            .show(ctx, |ui| {
                let data = received_data.lock().unwrap();

                let motor_thrusts = vec![
                    data.serial_data.front_right as f32,
                    data.serial_data.back_right as f32,
                    data.serial_data.back_left as f32,
                    data.serial_data.front_left as f32,
                ];

                // ui.horizontal(|ui| {
                self.draw_drone(ui, &motor_thrusts);
                //     ui.vertical(|ui| {
                //         self.draw_motor_bars(ui, &motor_thrusts);
                //         ui.add_space(10.0);
                //         // self.draw_aruco_ids(ui, &data.aruco_ids);
                //     });
                // });
            });
    }
    fn draw_drone(&self, ui: &mut egui::Ui, motor_thrusts: &[f32]) {
        let available_size = ui.available_size();
        let drone_size = available_size.x.min(available_size.y) - 40.0;
        let (response, painter) = ui.allocate_painter(
            egui::Vec2::new(drone_size, drone_size),
            egui::Sense::hover(),
        );
        let rect = response.rect;
        let center = rect.center();

        // Draw cross-shaped body
        let arm_length = drone_size / 3.0;
        let arm_width = drone_size / 20.0;
        let arm_color = egui::Color32::from_gray(80); // Darker grey for arms

        for i in 0..4 {
            let angle = std::f32::consts::PI / 4.0 + std::f32::consts::PI / 2.0 * i as f32;
            let end = center + egui::Vec2::new(angle.cos() * arm_length, angle.sin() * arm_length);
            painter.line_segment([center, end], (arm_width, arm_color));
        }

        // Draw central hub
        let hub_radius = drone_size / 8.0;
        painter.circle_filled(center, hub_radius, egui::Color32::from_gray(40)); // Even darker grey for hub

        // Draw motors and display thrust values
        let motor_radius = drone_size / 10.0;
        let motor_positions = [
            center + egui::Vec2::new(arm_length, -arm_length),
            center + egui::Vec2::new(arm_length, arm_length),
            center + egui::Vec2::new(-arm_length, arm_length),
            center + egui::Vec2::new(-arm_length, -arm_length),
        ];

        for (i, pos) in motor_positions.iter().enumerate() {
            let thrust = motor_thrusts.get(i).cloned().unwrap_or(0.0);
            let motor_color = self.get_color_for_thrust(thrust);
            painter.circle_filled(*pos, motor_radius, motor_color);
            painter.text(
                *pos,
                egui::Align2::CENTER_CENTER,
                format!("{:.0}", thrust),
                egui::FontId::proportional(15.0),
                // egui::Color32::WHITE, // White text for better contrast
                egui::Color32::BLACK
            );
        }

        // Draw forward direction indicator
        let forward_length = drone_size / 4.0;
        let forward_start = center + egui::Vec2::new(0.0, -hub_radius);
        let forward_end = center + egui::Vec2::new(0.0, -forward_length);
        painter.arrow(
            forward_start,
            forward_end - forward_start,
            (arm_width, egui::Color32::from_gray(200)), // Light grey for forward indicator
        );
    }

    fn get_color_for_thrust(&self, thrust: f32) -> egui::Color32 {
        // let normalized = (thrust / 100.0).clamp(0.0, 1.0);
        // let grey = (50.0 + normalized * 155.0) as u8; // Range from dark grey to light grey
        // egui::Color32::from_gray(grey)

        egui::Color32::LIGHT_YELLOW
    }

    fn draw_motor_bars(&self, ui: &mut egui::Ui, motor_thrusts: &[f32]) {
        ui.heading("Motor Thrusts");
        for (i, &thrust) in motor_thrusts.iter().enumerate() {
            let motor_name = match i {
                0 => "Front Right",
                1 => "Back Right",
                2 => "Back Left",
                3 => "Front Left",
                _ => "Unknown",
            };
            ui.horizontal(|ui| {
                ui.label(format!("{}: ", motor_name));
                let bar_height = 20.0;
                let bar_width = 150.0;
                let fill = thrust / 100.0; // Assuming max thrust is 100
                let (_, rect) = ui.allocate_space(egui::Vec2::new(bar_width, bar_height));
                let color = self.get_color_for_thrust(thrust);
                ui.painter()
                    .rect_filled(rect, 0.0, egui::Color32::DARK_GRAY);
                ui.painter().rect_filled(
                    egui::Rect::from_min_size(rect.min, egui::vec2(bar_width * fill, bar_height)),
                    0.0,
                    color,
                );
                ui.painter().text(
                    rect.center(),
                    egui::Align2::CENTER_CENTER,
                    format!("{:.1}", thrust),
                    egui::FontId::proportional(14.0),
                    egui::Color32::WHITE,
                );
            });
        }
    }

    fn draw_aruco_ids(&self, ui: &mut egui::Ui, aruco_ids: &[i32]) {
        ui.heading("Aruco IDs");
        ui.label(format!("{:?}", aruco_ids));
    }
}
