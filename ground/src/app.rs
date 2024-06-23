use crate::data::ReceivedData;
use crate::drone_view::DroneView;
use crate::attitude_view::AttitudeView;
use crate::accelerometer_view::AccelerometerView;
use eframe::egui;
use std::sync::{Arc, Mutex};

pub struct MyApp {
    drone_view: DroneView,
    attitude_view: AttitudeView,
    accelerometer_view: AccelerometerView,
    received_data: Arc<Mutex<ReceivedData>>,
}

impl MyApp {
    pub fn new(received_data: Arc<Mutex<ReceivedData>>) -> Self {
        Self {
            drone_view: DroneView::default(),
            attitude_view: AttitudeView::default(),
            accelerometer_view: AccelerometerView::new(),
            received_data,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Drone Control");
            self.drone_view.ui(ui);
            self.attitude_view.ui(ui);
            self.accelerometer_view.ui(ui);
        });

        self.drone_view.window(ctx, &self.received_data);
        self.attitude_view.window(ctx, &self.received_data);
        self.accelerometer_view.window(ctx, &self.received_data);

        ctx.request_repaint();
    }
}