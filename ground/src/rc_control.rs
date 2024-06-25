use crossbeam_channel::Sender;
use eframe::egui;
use std::sync::{Arc, Mutex};

const KEYBOARD_CONTROL_SPEED: f32 = 0.01; //0.05;

#[derive(Clone)]
pub struct RCControl {
    // open: bool,
    pub ui_to_drone_tx: Sender<String>,
    throttle: f32,
    yaw: f32,
    pitch: f32,
    roll: f32,
    left_active: bool,
    right_active: bool,
    last_sent_time: std::time::Instant,
}

impl RCControl {
    pub fn new(ui_to_drone_tx: Sender<String>) -> Self {
        Self {
            // open: false,
            ui_to_drone_tx,
            throttle: 0.0,
            yaw: 0.0,
            pitch: 0.0,
            roll: 0.0,
            left_active: false,
            right_active: false,
            last_sent_time: std::time::Instant::now(),
        }
    }
}

impl RCControl {
    // pub fn ui(&mut self, ui: &mut egui::Ui) {
    //     if ui
    //         .button(if self.open {
    //             "Close RC Control"
    //         } else {
    //             "Open RC Control"
    //         })
    //         .clicked()
    //     {
    //         self.open = !self.open;
    //     }
    // }

    pub fn window(&mut self, ctx: &egui::Context) {
        // transmit the RC control values to the drone every 50ms

        if self.last_sent_time.elapsed() > std::time::Duration::from_millis(50) {
            self.last_sent_time = std::time::Instant::now();

            // convert all ranges from 1000 to 2000

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
            // .open(&mut self.open)
            .resizable(true)
            .max_size([500.0, 300.0])
            .show(ctx, |ui| {
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
        // if !ctx.input().key_down(Key::A) && !ctx.input().key_down(Key::D) {
        //     self.yaw = 0.0;
        // }
        // if !ctx.input().key_down(Key::ArrowUp) && !ctx.input().key_down(Key::ArrowDown) {
        //     self.pitch = 0.0;
        // }
        // if !ctx.input().key_down(Key::ArrowLeft) && !ctx.input().key_down(Key::ArrowRight) {
        //     self.roll = 0.0;
        // }

        ctx.input(|i| {
            if !i.key_down(Key::A) && !i.key_down(Key::D) {
                self.yaw = 0.0;
            }
        });

        ctx.input(|i| {
            if !i.key_down(Key::ArrowUp) && !i.key_down(Key::ArrowDown) {
                self.pitch = 0.0;
            }
        });

        ctx.input(|i| {
            if !i.key_down(Key::ArrowLeft) && !i.key_down(Key::ArrowRight) {
                self.roll = 0.0;
            }
        });
    }
}
