use crossbeam_channel::Sender;
use eframe::egui;
use std::time::{Duration, Instant};

const KEYBOARD_CONTROL_SPEED: f32 = 0.025;

#[derive(Clone)]
pub struct RCControl {
    pub ui_to_drone_tx: Sender<String>,
    enabled_transmit: bool,
    throttle: f32,
    yaw: f32,
    pitch: f32,
    roll: f32,
    left_active: bool,
    right_active: bool,
    last_sent_time: Instant,
    last_sent_values: (f32, f32, f32, f32),
}

impl RCControl {
    pub fn new(ui_to_drone_tx: Sender<String>) -> Self {
        Self {
            ui_to_drone_tx,
            enabled_transmit: false,
            throttle: -1.0,
            yaw: 0.0,
            pitch: 0.0,
            roll: 0.0,
            left_active: false,
            right_active: false,
            last_sent_time: Instant::now(),
            last_sent_values: (-1.0, 0.0, 0.0, 0.0),
        }
    }

    pub fn window(&mut self, ctx: &egui::Context) {
        let current_time = Instant::now();
        let time_since_last_send = current_time.duration_since(self.last_sent_time);
        let current_values = (self.throttle, self.yaw, self.pitch, self.roll);

        if time_since_last_send >= Duration::from_millis(30)
            && self.enabled_transmit
            && current_values != self.last_sent_values
        {
            self.last_sent_time = current_time;
            self.last_sent_values = current_values;

            self.ui_to_drone_tx
                .send(format!(
                    "rc->{},{},{},{}\n",
                    (self.throttle * 500.0 + 1500.0).round(),
                    (self.yaw * 500.0 + 1500.0).round(),
                    (self.pitch * 500.0 + 1500.0).round(),
                    (self.roll * 500.0 + 1500.0).round()
                ))
                .expect("Failed to send RC control values");
        }

        egui::Window::new("RC Control")
            .resizable(true)
            .max_size([500.0, 300.0])
            .show(ctx, |ui| {
                ui.checkbox(&mut self.enabled_transmit, "Enable Transmit");

                let available_size = ui.available_size();
                let height = available_size.y - 20.0;
                let width = height * 2.0;
                ui.add_sized([width, height], |ui: &mut egui::Ui| {
                    self.draw_rc_control(ui);
                    ui.allocate_rect(ui.max_rect(), egui::Sense::hover())
                });

                ui.horizontal(|ui| {
                    ui.label(format!("Throttle: {:.2}", self.throttle));
                    ui.label(format!("Yaw: {:.2}", self.yaw));
                    ui.label(format!("Pitch: {:.2}", self.pitch));
                    ui.label(format!("Roll: {:.2}", self.roll));
                });

                self.handle_keyboard_input(ctx);
            });
    }

    fn draw_rc_control(&mut self, ui: &mut egui::Ui) -> egui::Response {
        let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::drag());
        let rect = response.rect;

        let center = rect.center();
        let height = rect.height();
        let stick_area_size = height / 2.0;
        let stick_size = stick_area_size / 6.0;

        // Left stick (throttle and yaw)
        let left_center = center - egui::vec2(stick_area_size * 1.2, 0.0);
        painter.circle_stroke(
            left_center,
            stick_area_size / 2.0,
            (1.0, egui::Color32::GRAY),
        );
        let left_stick_pos = left_center
            + egui::vec2(
                self.yaw * stick_area_size / 2.0,
                -self.throttle * stick_area_size / 2.0,
            );
        painter.circle_filled(left_stick_pos, stick_size / 2.0, egui::Color32::DARK_GRAY);

        // Right stick (pitch and roll)
        let right_center = center + egui::vec2(stick_area_size * 1.2, 0.0);
        painter.circle_stroke(
            right_center,
            stick_area_size / 2.0,
            (1.0, egui::Color32::GRAY),
        );
        let right_stick_pos = right_center
            + egui::vec2(
                self.roll * stick_area_size / 2.0,
                -self.pitch * stick_area_size / 2.0,
            );
        painter.circle_filled(right_stick_pos, stick_size / 2.0, egui::Color32::DARK_GRAY);

        // Labels
        let small_font_id = egui::FontId::proportional(12.0);

        // Left stick labels
        painter.text(
            left_center - egui::vec2(stick_area_size / 2.0 + 5.0, 0.0),
            egui::Align2::RIGHT_CENTER,
            "Yaw L (A)",
            small_font_id.clone(),
            egui::Color32::WHITE,
        );
        painter.text(
            left_center + egui::vec2(stick_area_size / 2.0 + 5.0, 0.0),
            egui::Align2::LEFT_CENTER,
            "Yaw R (D)",
            small_font_id.clone(),
            egui::Color32::WHITE,
        );
        painter.text(
            left_center - egui::vec2(0.0, stick_area_size / 2.0 + 5.0),
            egui::Align2::CENTER_BOTTOM,
            "Throttle Up (W)",
            small_font_id.clone(),
            egui::Color32::WHITE,
        );
        painter.text(
            left_center + egui::vec2(0.0, stick_area_size / 2.0 + 5.0),
            egui::Align2::CENTER_TOP,
            "Throttle Down (S)",
            small_font_id.clone(),
            egui::Color32::WHITE,
        );

        // Right stick labels
        painter.text(
            right_center - egui::vec2(stick_area_size / 2.0 + 5.0, 0.0),
            egui::Align2::RIGHT_CENTER,
            "Roll L (Left)",
            small_font_id.clone(),
            egui::Color32::WHITE,
        );
        painter.text(
            right_center + egui::vec2(stick_area_size / 2.0 + 5.0, 0.0),
            egui::Align2::LEFT_CENTER,
            "Roll R (Right)",
            small_font_id.clone(),
            egui::Color32::WHITE,
        );
        painter.text(
            right_center - egui::vec2(0.0, stick_area_size / 2.0 + 5.0),
            egui::Align2::CENTER_BOTTOM,
            "Pitch Up (Up)",
            small_font_id.clone(),
            egui::Color32::WHITE,
        );
        painter.text(
            right_center + egui::vec2(0.0, stick_area_size / 2.0 + 5.0),
            egui::Align2::CENTER_TOP,
            "Pitch Down (Down)",
            small_font_id,
            egui::Color32::WHITE,
        );

        // Handle mouse input
        if response.drag_started() {
            let pointer_pos = response.interact_pointer_pos().unwrap();
            if (left_center - pointer_pos).length() < stick_area_size / 2.0 {
                self.left_active = true;
            } else if (right_center - pointer_pos).length() < stick_area_size / 2.0 {
                self.right_active = true;
            }
        }

        if response.dragged() {
            let drag_delta = response.drag_delta();
            if self.left_active {
                self.yaw = ((left_stick_pos.x + drag_delta.x - left_center.x)
                    / (stick_area_size / 2.0))
                    .clamp(-1.0, 1.0);
                self.throttle = ((left_center.y - (left_stick_pos.y + drag_delta.y))
                    / (stick_area_size / 2.0))
                    .clamp(-1.0, 1.0);
            } else if self.right_active {
                self.roll = ((right_stick_pos.x + drag_delta.x - right_center.x)
                    / (stick_area_size / 2.0))
                    .clamp(-1.0, 1.0);
                self.pitch = ((right_center.y - (right_stick_pos.y + drag_delta.y))
                    / (stick_area_size / 2.0))
                    .clamp(-1.0, 1.0);
            }
        }

        if response.drag_stopped() {
            if self.left_active {
                self.yaw = 0.0;
                self.left_active = false;
            }
            if self.right_active {
                self.pitch = 0.0;
                self.roll = 0.0;
                self.right_active = false;
            }
        }

        response
    }

    fn handle_keyboard_input(&mut self, ctx: &egui::Context) {
        use egui::Key;

        // Left stick (WASD)
        // if ctx.input().key_down(Key::W) {
        //     self.throttle = (self.throttle + KEYBOARD_CONTROL_SPEED).min(1.0);
        // }
        ctx.input(|i| {
            if i.key_down(Key::W) {
                self.throttle = (self.throttle + KEYBOARD_CONTROL_SPEED).min(1.0);
            }
        });

        ctx.input(|i| {
            if i.key_down(Key::Space) {
                self.throttle = -1.0;

                self.yaw = 0.0;
                self.pitch = 0.0;
                self.roll = 0.0;
            }
        });

        // if ctx.input().key_down(Key::S) {
        //     self.throttle = (self.throttle - KEYBOARD_CONTROL_SPEED).max(-1.0);
        // }
        // if ctx.input().key_down(Key::A) {
        //     self.yaw = (self.yaw - KEYBOARD_CONTROL_SPEED).max(-1.0);
        // }
        // if ctx.input().key_down(Key::D) {
        //     self.yaw = (self.yaw + KEYBOARD_CONTROL_SPEED).min(1.0);
        // }

        ctx.input(|i| {
            if i.key_down(Key::S) {
                self.throttle = (self.throttle - KEYBOARD_CONTROL_SPEED).max(-1.0);
            }
        });

        ctx.input(|i| {
            if i.key_down(Key::A) {
                self.yaw = (self.yaw - KEYBOARD_CONTROL_SPEED).max(-1.0);
            }
        });

        ctx.input(|i| {
            if i.key_down(Key::D) {
                self.yaw = (self.yaw + KEYBOARD_CONTROL_SPEED).min(1.0);
            }
        });

        // Right stick (Arrow keys)
        // if ctx.input().key_down(Key::ArrowUp) {
        //     self.pitch = (self.pitch + KEYBOARD_CONTROL_SPEED).min(1.0);
        // }
        // if ctx.input().key_down(Key::ArrowDown) {
        //     self.pitch = (self.pitch - KEYBOARD_CONTROL_SPEED).max(-1.0);
        // }
        // if ctx.input().key_down(Key::ArrowLeft) {
        //     self.roll = (self.roll - KEYBOARD_CONTROL_SPEED).max(-1.0);
        // }
        // if ctx.input().key_down(Key::ArrowRight) {
        //     self.roll = (self.roll + KEYBOARD_CONTROL_SPEED).min(1.0);
        // }

        ctx.input(|i| {
            if i.key_down(Key::ArrowUp) {
                self.pitch = (self.pitch + KEYBOARD_CONTROL_SPEED).min(1.0);
            }
        });

        ctx.input(|i| {
            if i.key_down(Key::ArrowDown) {
                self.pitch = (self.pitch - KEYBOARD_CONTROL_SPEED).max(-1.0);
            }
        });

        ctx.input(|i| {
            if i.key_down(Key::ArrowLeft) {
                self.roll = (self.roll - KEYBOARD_CONTROL_SPEED).max(-1.0);
            }
        });

        ctx.input(|i| {
            if i.key_down(Key::ArrowRight) {
                self.roll = (self.roll + KEYBOARD_CONTROL_SPEED).min(1.0);
            }
        });

        // // Auto-center yaw, pitch, and roll when keys are released

        // ctx.input(|i| {
        //     if !i.key_down(Key::A) && !i.key_down(Key::D) {
        //         self.yaw = 0.0;
        //     }
        // });

        // ctx.input(|i| {
        //     if !i.key_down(Key::ArrowUp) && !i.key_down(Key::ArrowDown) {
        //         self.pitch = 0.0;
        //     }
        // });

        // ctx.input(|i| {
        //     if !i.key_down(Key::ArrowLeft) && !i.key_down(Key::ArrowRight) {
        //         self.roll = 0.0;
        //     }
        // });
    }
}
