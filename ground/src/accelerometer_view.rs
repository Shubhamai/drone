use crate::data::ReceivedData;
use eframe::egui;
use egui_plot::{Line, Plot, PlotPoint, PlotPoints, Text};
use ringbuffer::{AllocRingBuffer, RingBuffer};
use std::sync::{Arc, Mutex};

const CHART_HISTORY: usize = 10000;

#[derive(Clone)]
pub struct AccelerometerView {
    open: bool,
    accel_x: AllocRingBuffer<(f64, f64)>,
    accel_y: AllocRingBuffer<(f64, f64)>,
    accel_z: AllocRingBuffer<(f64, f64)>,
}

impl AccelerometerView {
    pub fn new() -> Self {
        Self {
            open: false,
            accel_x: AllocRingBuffer::new(CHART_HISTORY),
            accel_y: AllocRingBuffer::new(CHART_HISTORY),
            accel_z: AllocRingBuffer::new(CHART_HISTORY),
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        if ui
            .button(if self.open {
                "Close Accelerometer View"
            } else {
                "Open Accelerometer View"
            })
            .clicked()
        {
            self.open = !self.open;
        }
    }

    pub fn window(&mut self, ctx: &egui::Context, received_data: &Arc<Mutex<ReceivedData>>) {
        egui::Window::new("Accelerometer Data")
            // .open(&mut self.open)
            .default_size([600.0, 400.0])
            .resizable(true)
            .show(ctx, |ui| {
                let data = received_data.lock().unwrap();

                let elapsed = data.serial_data.elapsed_time as f64 / 1000.0;

                self.accel_x.push((elapsed, data.serial_data.acc_x as f64));
                self.accel_y.push((elapsed, data.serial_data.acc_y as f64));
                self.accel_z.push((elapsed, data.serial_data.acc_z as f64));

                Plot::new("accelerometer_plot")
                    .view_aspect(2.0)
                    .x_axis_label("Time (seconds)")
                    .y_axis_label("Acceleration (m/s²)")
                    .legend(egui_plot::Legend::default())
                    .show(ui, |plot_ui| {
                        const Y_AXIS_RANGE: f64 = 20.0;

                        let time_range = 10.0; // Show last 10 seconds of data
                        plot_ui.set_plot_bounds(egui_plot::PlotBounds::from_min_max(
                            [elapsed - time_range, -Y_AXIS_RANGE],
                            [elapsed, Y_AXIS_RANGE],
                        ));

                        let x_line = Line::new(PlotPoints::from_iter(
                            self.accel_x.iter().map(|&(t, v)| [t, v]),
                        ))
                        .name("X-axis")
                        .color(egui::Color32::RED);
                        let y_line = Line::new(PlotPoints::from_iter(
                            self.accel_y.iter().map(|&(t, v)| [t, v]),
                        ))
                        .name("Y-axis")
                        .color(egui::Color32::GREEN);
                        let z_line = Line::new(PlotPoints::from_iter(
                            self.accel_z.iter().map(|&(t, v)| [t, v]),
                        ))
                        .name("Z-axis")
                        .color(egui::Color32::BLUE);

                        plot_ui.line(x_line);
                        plot_ui.line(y_line);
                        plot_ui.line(z_line);

                        // Add current values text
                        let text = format!(
                            "Current: X: {:.2}, Y: {:.2}, Z: {:.2}",
                            self.accel_x.back().map(|&(_, v)| v).unwrap_or(0.0),
                            self.accel_y.back().map(|&(_, v)| v).unwrap_or(0.0),
                            self.accel_z.back().map(|&(_, v)| v).unwrap_or(0.0)
                        );

                        plot_ui.text(Text::new(
                            PlotPoint::from([elapsed - time_range + 1.0, Y_AXIS_RANGE - 50.]),
                            egui::RichText::new(text)
                                .size(14.0)
                                .color(egui::Color32::WHITE),
                        ));
                    });
            });
    }
}
