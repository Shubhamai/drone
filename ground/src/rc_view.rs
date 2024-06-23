use crate::data::ReceivedData;
use eframe::egui;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct RCView {
    open: bool,
}

impl Default for RCView {
    fn default() -> Self {
        Self { open: false }
    }
}

impl RCView {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        if ui
            .button(if self.open {
                "Close RC View"
            } else {
                "Open RC View"
            })
            .clicked()
        {
            self.open = !self.open;
        }
    }

    pub fn window(&mut self, ctx: &egui::Context, received_data: &Arc<Mutex<ReceivedData>>) {
        egui::Window::new("RC Remote View")
            // .open(&mut self.open)
            .resizable(true)
            .default_size([500.0, 300.0]) // More rectangular aspect ratio
            .show(ctx, |ui| {
                let data = received_data.lock().unwrap();

                let thrust = data.serial_data.rc_throttle as f32;
                let yaw = data.serial_data.rc_yaw as f32;
                let pitch = data.serial_data.rc_pitch as f32;
                let roll = data.serial_data.rc_roll as f32;

                // convert range 1000-2000 to degree
                let thrust = (thrust - 1000.0) / 10.0;
                let yaw = (yaw - 1500.0) / 10.0;
                let pitch = (pitch - 1500.0) / 10.0;
                let roll = (roll - 1500.0) / 10.0;

                // ui.heading("RC Remote Values");
                // ui.add_space(10.0);

                // ui.horizontal(|ui| {
                //     ui.label("Thrust:");
                //     ui.label(format!("{:.2}", thrust));
                // });
                // ui.horizontal(|ui| {
                //     ui.label("Yaw:");
                //     ui.label(format!("{:.2}°", yaw));
                // });
                // ui.horizontal(|ui| {
                //     ui.label("Pitch:");
                //     ui.label(format!("{:.2}°", pitch));
                // });
                // ui.horizontal(|ui| {
                //     ui.label("Roll:");
                //     ui.label(format!("{:.2}°", roll));
                // });

                // ui.add_space(20.0);

                let available_size = ui.available_size();
                let height = available_size.y - 20.0;
                let width = height * 2.0;
                ui.add_sized([width, height], |ui: &mut egui::Ui| {
                    draw_rc_remote(ui, thrust, yaw, pitch, roll);
                    ui.allocate_rect(ui.max_rect(), egui::Sense::hover())
                });
            });
    }
}

fn draw_rc_remote(
    ui: &mut egui::Ui,
    thrust: f32,
    yaw: f32,
    pitch: f32,
    roll: f32,
) -> egui::Response {
    let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::hover());
    let rect = response.rect;

    let center = rect.center();
    let height = rect.height();
    let stick_area_size = height / 2.0;
    let stick_size = stick_area_size / 6.0; // Smaller inner circle

    // Draw left stick (thrust and yaw)
    let left_center = center - egui::vec2(stick_area_size * 1.2, 0.0);
    painter.circle_stroke(
        left_center,
        stick_area_size / 2.0,
        (1.0, egui::Color32::GRAY),
    );
    let left_stick_pos = left_center
        + egui::vec2(
            (yaw / 100.0) * stick_area_size / 2.0,
            (-thrust / 100.0) * stick_area_size / 2.0,
        );
    painter.circle_filled(left_stick_pos, stick_size / 2.0, egui::Color32::DARK_GRAY);

    // Draw right stick (pitch and roll)
    let right_center = center + egui::vec2(stick_area_size * 1.2, 0.0);
    painter.circle_stroke(
        right_center,
        stick_area_size / 2.0,
        (1.0, egui::Color32::GRAY),
    );
    let right_stick_pos = right_center
        + egui::vec2(
            (roll / 100.0) * stick_area_size / 2.0,
            (-pitch / 100.0) * stick_area_size / 2.0,
        );
    painter.circle_filled(right_stick_pos, stick_size / 2.0, egui::Color32::DARK_GRAY);

    // Labels
    let font_id = egui::FontId::proportional(14.0);
    let small_font_id = egui::FontId::proportional(12.0);

    // Left stick labels
    // painter.text(
    //     left_center - egui::vec2(0.0, stick_area_size / 2.0 + 10.0),
    //     egui::Align2::CENTER_BOTTOM,
    //     "Thrust/Yaw",
    //     font_id.clone(),
    //     egui::Color32::WHITE,
    // );
    painter.text(
        left_center - egui::vec2(stick_area_size / 2.0 + 5.0, 0.0),
        egui::Align2::RIGHT_CENTER,
        "Yaw L",
        small_font_id.clone(),
        egui::Color32::WHITE,
    );
    painter.text(
        left_center + egui::vec2(stick_area_size / 2.0 + 5.0, 0.0),
        egui::Align2::LEFT_CENTER,
        "Yaw R",
        small_font_id.clone(),
        egui::Color32::WHITE,
    );
    painter.text(
        left_center - egui::vec2(0.0, stick_area_size / 2.0 + 5.0),
        egui::Align2::CENTER_BOTTOM,
        "Thrust Up",
        small_font_id.clone(),
        egui::Color32::WHITE,
    );
    painter.text(
        left_center + egui::vec2(0.0, stick_area_size / 2.0 + 5.0),
        egui::Align2::CENTER_TOP,
        "Thrust Down",
        small_font_id.clone(),
        egui::Color32::WHITE,
    );

    // Right stick labels
    // painter.text(
    //     right_center - egui::vec2(0.0, stick_area_size / 2.0 + 10.0),
    //     egui::Align2::CENTER_BOTTOM,
    //     "Pitch/Roll",
    //     font_id,
    //     egui::Color32::WHITE,
    // );
    painter.text(
        right_center - egui::vec2(stick_area_size / 2.0 + 5.0, 0.0),
        egui::Align2::RIGHT_CENTER,
        "Roll L",
        small_font_id.clone(),
        egui::Color32::WHITE,
    );
    painter.text(
        right_center + egui::vec2(stick_area_size / 2.0 + 5.0, 0.0),
        egui::Align2::LEFT_CENTER,
        "Roll R",
        small_font_id.clone(),
        egui::Color32::WHITE,
    );
    painter.text(
        right_center - egui::vec2(0.0, stick_area_size / 2.0 + 5.0),
        egui::Align2::CENTER_BOTTOM,
        "Pitch Up",
        small_font_id.clone(),
        egui::Color32::WHITE,
    );
    painter.text(
        right_center + egui::vec2(0.0, stick_area_size / 2.0 + 5.0),
        egui::Align2::CENTER_TOP,
        "Pitch Down",
        small_font_id,
        egui::Color32::WHITE,
    );

    response
}
