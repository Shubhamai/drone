use crate::data::ReceivedData;
use eframe::egui;
use egui_plot::{Legend, Line, Plot, PlotPoint, PlotPoints, Text};
use nalgebra as na;
use ringbuffer::{AllocRingBuffer, RingBuffer};
use std::sync::{Arc, Mutex};

const CHART_HISTORY: usize = 10000;
const ERROR_MARGIN: f64 = 10.0; // 10 degree error margin

#[derive(Clone)]
pub struct AttitudeView {
    open: bool,
    view_mode: ViewMode,
    yaw_data: AllocRingBuffer<(f64, f64)>,
    pitch_data: AllocRingBuffer<(f64, f64)>,
    roll_data: AllocRingBuffer<(f64, f64)>,
    yaw_visible: bool,
    pitch_visible: bool,
    roll_visible: bool,
}

#[derive(Clone, PartialEq)]
enum ViewMode {
    Drone,
    Graph,
}

impl Default for AttitudeView {
    fn default() -> Self {
        Self {
            open: false,
            view_mode: ViewMode::Drone,
            yaw_data: AllocRingBuffer::new(CHART_HISTORY),
            pitch_data: AllocRingBuffer::new(CHART_HISTORY),
            roll_data: AllocRingBuffer::new(CHART_HISTORY),
            yaw_visible: true,
            pitch_visible: true,
            roll_visible: true,
        }
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
            .resizable(true)
            .show(ctx, |ui| {
                let data = received_data.lock().unwrap();

                let yaw = data.serial_data.yaw as f64;
                let pitch = data.serial_data.pitch as f64;
                let roll = data.serial_data.roll as f64;
                let elapsed = data.serial_data.elapsed_time as f64 / 1000.0;

                self.yaw_data.push((elapsed, yaw));
                self.pitch_data.push((elapsed, pitch));
                self.roll_data.push((elapsed, roll));

                ui.horizontal(|ui| {
                    ui.selectable_value(
                        &mut self.view_mode,
                        ViewMode::Drone,
                        "Drone Visualization",
                    );
                    ui.selectable_value(&mut self.view_mode, ViewMode::Graph, "Graph View");
                });

                ui.add_space(10.0);

                match self.view_mode {
                    ViewMode::Drone => {
                        let available_size = ui.available_size();
                        let size = available_size.x.min(available_size.y);
                        ui.horizontal_centered(|ui| {
                            ui.add_sized([size, size], |ui: &mut egui::Ui| {
                                draw_sci_fi_drone(
                                    ui,
                                    (yaw as f32).to_radians(),
                                    (pitch as f32).to_radians(),
                                    (roll as f32).to_radians(),
                                );
                                ui.allocate_rect(ui.max_rect(), egui::Sense::hover())
                            });
                        });
                    }
                    ViewMode::Graph => {
                        Plot::new("attitude_plot")
                            .view_aspect(2.0)
                            .x_axis_label("Time (seconds)")
                            .y_axis_label("Angle (degrees)")
                            .legend(Legend::default())
                            .show(ui, |plot_ui| {
                                let time_range = 10.0; // Show last 10 seconds of data

                                // Calculate y_min and y_max based on visible data series
                                let (y_min, y_max) = self.calculate_y_range();

                                plot_ui.set_plot_bounds(egui_plot::PlotBounds::from_min_max(
                                    [elapsed - time_range, y_min],
                                    [elapsed, y_max],
                                ));

                                if self.yaw_visible {
                                    let yaw_line = Line::new(PlotPoints::from_iter(
                                        self.yaw_data.iter().map(|&(t, v)| [t, v]),
                                    ))
                                    .name("Yaw")
                                    .color(egui::Color32::RED);
                                    plot_ui.line(yaw_line);
                                }

                                if self.pitch_visible {
                                    let pitch_line = Line::new(PlotPoints::from_iter(
                                        self.pitch_data.iter().map(|&(t, v)| [t, v]),
                                    ))
                                    .name("Pitch")
                                    .color(egui::Color32::GREEN);
                                    plot_ui.line(pitch_line);
                                }

                                if self.roll_visible {
                                    let roll_line = Line::new(PlotPoints::from_iter(
                                        self.roll_data.iter().map(|&(t, v)| [t, v]),
                                    ))
                                    .color(egui::Color32::BLUE);
                                    plot_ui.line(roll_line);
                                }

                                // Add current values text
                                let text = format!(
                                    "Current: Yaw: {:.2}°, Pitch: {:.2}°, Roll: {:.2}°",
                                    yaw, pitch, roll
                                );

                                plot_ui.text(Text::new(
                                    PlotPoint::from([elapsed - time_range + 1.0, y_max - 5.0]),
                                    egui::RichText::new(text)
                                        .size(14.0)
                                        .color(egui::Color32::WHITE),
                                ));
                            });

                        // Add toggle buttons for each data series
                        ui.horizontal(|ui| {
                            ui.checkbox(&mut self.yaw_visible, "Yaw");
                            ui.checkbox(&mut self.pitch_visible, "Pitch");
                            ui.checkbox(&mut self.roll_visible, "Roll");
                        });
                    }
                }
            });
    }

    fn calculate_y_range(&self) -> (f64, f64) {
        let mut min_value = f64::MAX;
        let mut max_value = f64::MIN;

        if self.yaw_visible {
            let (yaw_min, yaw_max) = self
                .yaw_data
                .iter()
                .fold((f64::MAX, f64::MIN), |(min, max), &(_, v)| {
                    (min.min(v), max.max(v))
                });
            min_value = min_value.min(yaw_min);
            max_value = max_value.max(yaw_max);
        }

        if self.pitch_visible {
            let (pitch_min, pitch_max) = self
                .pitch_data
                .iter()
                .fold((f64::MAX, f64::MIN), |(min, max), &(_, v)| {
                    (min.min(v), max.max(v))
                });
            min_value = min_value.min(pitch_min);
            max_value = max_value.max(pitch_max);
        }

        if self.roll_visible {
            let (roll_min, roll_max) = self
                .roll_data
                .iter()
                .fold((f64::MAX, f64::MIN), |(min, max), &(_, v)| {
                    (min.min(v), max.max(v))
                });
            min_value = min_value.min(roll_min);
            max_value = max_value.max(roll_max);
        }

        if min_value == f64::MAX || max_value == f64::MIN {
            // If no data series are visible, default to a reasonable range
            (-180.0, 180.0)
        } else {
            // Add error margin and clamp to valid range
            (
                (min_value - ERROR_MARGIN).max(-180.0),
                (max_value + ERROR_MARGIN).min(180.0),
            )
        }
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
        egui::Align2::CENTER_TOP,
        // "Roll",
        format!("Roll {:.2}°", roll.to_degrees()),
        egui::FontId::proportional(14.0),
        egui::Color32::WHITE,
    );
    painter.text(
        // egui::Pos2::new(center.x, center.y + label_offset),
        projected_points[2],
        egui::Align2::CENTER_TOP,
        // "Pitch",
        format!("Pitch {:.2}°", pitch.to_degrees()),
        egui::FontId::proportional(14.0),
        egui::Color32::WHITE,
    );
    painter.text(
        projected_points[4],
        egui::Align2::CENTER_BOTTOM,
        // "Yaw",
        format!("Yaw {:.2}°", yaw.to_degrees()),
        egui::FontId::proportional(14.0),
        egui::Color32::WHITE,
    );

    response
}
