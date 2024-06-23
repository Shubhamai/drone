use crate::data::ReceivedData;
use eframe::egui;
use nalgebra as na;
use std::sync::{Arc, Mutex};

pub struct AttitudeView {
    open: bool,
}

impl Default for AttitudeView {
    fn default() -> Self {
        Self { open: false }
    }
}

impl AttitudeView {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        if ui
            .button(if self.open {
                "Close Attitude View"
            } else {
                "Open Attitude View"
            })
            .clicked()
        {
            self.open = !self.open;
        }
    }

    pub fn window(&mut self, ctx: &egui::Context, received_data: &Arc<Mutex<ReceivedData>>) {
        egui::Window::new("Drone Attitude")
            .open(&mut self.open)
            .resizable(true)
            .show(ctx, |ui| {
                let data = received_data.lock().unwrap();
                let parts: Vec<&str> = data.serial_data.split(',').collect();

                // Assuming the last three values in serial_data are yaw, pitch, roll in degrees
                let yaw = parts
                    .get(1)
                    .and_then(|s| s.parse::<f32>().ok())
                    .unwrap_or(0.0);
                let pitch = parts
                    .get(2)
                    .and_then(|s| s.parse::<f32>().ok())
                    .unwrap_or(0.0);
                let roll = parts
                    .get(3)
                    .and_then(|s| s.parse::<f32>().ok())
                    .unwrap_or(0.0);

                ui.heading("Drone Attitude");
                // ui.add_space(10.0);

                ui.horizontal(|ui| {
                    ui.label("Yaw:");
                    ui.label(format!("{:.2}°", yaw));
                });
                ui.horizontal(|ui| {
                    ui.label("Pitch:");
                    ui.label(format!("{:.2}°", pitch));
                });
                ui.horizontal(|ui| {
                    ui.label("Roll:");
                    ui.label(format!("{:.2}°", roll));
                });
                ui.add_space(10.0);

                // Use available width for the drone visualization
                let available_size = ui.available_size();
                let size = available_size.x.min(available_size.y); //.min(300.0); // Cap at 300 for very large windows
                ui.horizontal_centered(|ui| {
                    ui.add_sized([size, size], |ui: &mut egui::Ui| {
                        draw_sci_fi_drone(
                            ui,
                            yaw.to_radians(),
                            pitch.to_radians(),
                            roll.to_radians(),
                        );
                        ui.allocate_rect(ui.max_rect(), egui::Sense::hover()) // Ensure the drawing takes up the full allocated space
                    });
                });
            });
    }
}

fn draw_sci_fi_drone(ui: &mut egui::Ui, yaw: f32, pitch: f32, roll: f32) -> egui::Response {
    let size = ui.available_size();
    let (response, painter) = ui.allocate_painter(size, egui::Sense::hover());
    let rect = response.rect;

    let center = rect.center();
    let draw_size = size.x.min(size.y) * 0.8;

    // Create rotation matrix
    let rotation = na::Rotation3::from_euler_angles(roll, pitch, yaw);

    // Define drone vertices
    let vertices = vec![
        na::Point3::new(1.0, 0.0, 0.0),
        na::Point3::new(-1.0, 0.0, 0.0),
        na::Point3::new(0.0, 1.0, 0.0),
        na::Point3::new(0.0, -1.0, 0.0),
        na::Point3::new(0.0, 0.0, 0.5),
        na::Point3::new(0.7, 0.7, 0.0),
        na::Point3::new(-0.7, 0.7, 0.0),
        na::Point3::new(-0.7, -0.7, 0.0),
        na::Point3::new(0.7, -0.7, 0.0),
    ];

    // Rotate vertices
    let rotated_vertices: Vec<na::Point3<f32>> = vertices.iter().map(|v| rotation * v).collect();

    // Project 3D points to 2D
    let projected_points: Vec<egui::Pos2> = rotated_vertices
        .iter()
        .map(|v| {
            let x = v.x * draw_size / 2.0 + center.x;
            let y = -v.z * draw_size / 2.0 + center.y;
            egui::Pos2::new(x, y)
        })
        .collect();

    // Draw drone frame
    let frame_color = egui::Color32::from_rgb(0, 255, 255); // Cyan color for sci-fi look
    painter.line_segment(
        [projected_points[0], projected_points[1]],
        (2.0, frame_color),
    );
    painter.line_segment(
        [projected_points[2], projected_points[3]],
        (2.0, frame_color),
    );

    // Draw diagonal arms
    painter.line_segment(
        [projected_points[5], projected_points[7]],
        (2.0, frame_color),
    );
    painter.line_segment(
        [projected_points[6], projected_points[8]],
        (2.0, frame_color),
    );

    // Draw direction indicator
    painter.line_segment([center, projected_points[4]], (3.0, egui::Color32::YELLOW));

    // Draw motors
    for i in 5..9 {
        painter.circle_filled(projected_points[i], 5.0, egui::Color32::RED);
    }

    // Draw axis labels
    let label_offset = draw_size / 2.0 + 20.0;
    painter.text(
        // egui::Pos2::new(center.x + label_offset, center.y),
        projected_points[0],
        egui::Align2::LEFT_CENTER,
        "Roll",
        egui::FontId::proportional(14.0),
        egui::Color32::WHITE,
    );
    painter.text(
        // egui::Pos2::new(center.x, center.y + label_offset),
        projected_points[2],
        egui::Align2::CENTER_TOP,
        "Pitch",
        egui::FontId::proportional(14.0),
        egui::Color32::WHITE,
    );
    painter.text(
        projected_points[4],
        egui::Align2::CENTER_BOTTOM,
        "Yaw",
        egui::FontId::proportional(14.0),
        egui::Color32::WHITE,
    );

    response
}
