use crate::accelerometer_view::AccelerometerView;
use crate::attitude_view::AttitudeView;
use crate::chat_view::ChatView;
use crate::data::ReceivedData;
use crate::drone_view::DroneView;
use crate::rc_view::RCView;
use chrono::Local;
use eframe::egui;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Clone)]

pub struct MyApp {
    drone_view: DroneView,
    attitude_view: AttitudeView,
    accelerometer_view: AccelerometerView,
    rc_view: RCView,
    pub chat_view: ChatView,
    received_data: Arc<Mutex<ReceivedData>>,
    start_time: Instant,
    last_received_time: Arc<Mutex<Instant>>,
    tabs: Vec<Tab>,
    active_tab: usize,
    is_connected: bool,
    connection_attempts: u32,
}

#[derive(Clone)]
struct Tab {
    name: String,
    windows: Vec<WindowType>,
}

#[derive(Clone, PartialEq)]
enum WindowType {
    Drone,
    Attitude,
    Accelerometer,
    RC,
    Chat,
}

impl MyApp {
    pub fn new(received_data: Arc<Mutex<ReceivedData>>) -> Self {
        Self {
            drone_view: DroneView::default(),
            attitude_view: AttitudeView::default(),
            accelerometer_view: AccelerometerView::new(),
            rc_view: RCView::default(),
            chat_view: ChatView::default(),
            received_data,
            start_time: Instant::now(),
            last_received_time: Arc::new(Mutex::new(Instant::now())),
            tabs: vec![Tab {
                name: "Tab 1".to_string(),
                windows: Vec::new(),
            }],
            active_tab: 0,
            is_connected: false,
            connection_attempts: 0,
        }
    }

    pub fn update_last_received_time(&self) {
        if let Ok(mut last_received) = self.last_received_time.lock() {
            *last_received = Instant::now();
        }
    }

    pub fn update_connection_status(&mut self, status: bool) {
        self.is_connected = status;
        if status {
            self.connection_attempts = 0;
        }
    }

    pub fn increment_connection_attempts(&mut self) {
        self.connection_attempts += 1;
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let now = Local::now();
        let app_elapsed = self.start_time.elapsed();

        let last_packet_elapsed = self
            .last_received_time
            .lock()
            .map(|time| time.elapsed())
            .unwrap_or_else(|_| Duration::from_secs(0));

        egui::TopBottomPanel::top("title_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("Day: {}", now.format("%A")));
                ui.label(format!("Date: {}", now.format("%Y-%m-%d")));
                ui.label(format!("Time: {}", now.format("%H:%M:%S")));
                ui.label(format!(
                    "App Elapsed: {:02}:{:02}:{:02}",
                    app_elapsed.as_secs() / 3600,
                    (app_elapsed.as_secs() % 3600) / 60,
                    app_elapsed.as_secs() % 60
                ));

                if let Ok(data) = self.received_data.lock() {
                    let drone_elapsed = data.serial_data.elapsed_time;
                    ui.label(format!("Drone Elapsed: {:.2}s", drone_elapsed));
                }

                ui.label(format!(
                    "Last Packet: {:.2}s ago",
                    last_packet_elapsed.as_secs_f32()
                ));

                let connected = last_packet_elapsed < Duration::from_millis(500);
                ui.colored_label(
                    if connected {
                        egui::Color32::GREEN
                    } else {
                        egui::Color32::RED
                    },
                    if connected {
                        "Connected"
                    } else {
                        "Disconnected"
                    },
                );

                if !connected {
                    ui.label(format!("Connection Attempts: {}", self.connection_attempts));
                }
            });
        });

        egui::TopBottomPanel::top("tabs").show(ctx, |ui| {
            ui.horizontal(|ui| {
                for (index, tab) in self.tabs.iter().enumerate() {
                    if ui
                        .selectable_label(self.active_tab == index, &tab.name)
                        .clicked()
                    {
                        self.active_tab = index;
                    }
                }
                if ui.button("+").clicked() {
                    self.tabs.push(Tab {
                        name: format!("Tab {}", self.tabs.len() + 1),
                        windows: Vec::new(),
                    });
                    self.active_tab = self.tabs.len() - 1;
                }
            });
        });

        egui::SidePanel::right("controls").show(ctx, |ui| {
            ui.vertical(|ui| {
                if ui.button("Add Drone View").clicked() {
                    self.tabs[self.active_tab].windows.push(WindowType::Drone);
                }
                if ui.button("Add Attitude View").clicked() {
                    self.tabs[self.active_tab]
                        .windows
                        .push(WindowType::Attitude);
                }
                if ui.button("Add Accelerometer View").clicked() {
                    self.tabs[self.active_tab]
                        .windows
                        .push(WindowType::Accelerometer);
                }
                if ui.button("Add RC View").clicked() {
                    self.tabs[self.active_tab].windows.push(WindowType::RC);
                }
                if ui.button("Add Chat").clicked() {
                    self.tabs[self.active_tab].windows.push(WindowType::Chat);
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |_ui| {
            for window in &self.tabs[self.active_tab].windows {
                match window {
                    WindowType::Drone => self.drone_view.window(ctx, &self.received_data),
                    WindowType::Attitude => self.attitude_view.window(ctx, &self.received_data),
                    WindowType::Accelerometer => {
                        self.accelerometer_view.window(ctx, &self.received_data)
                    }
                    WindowType::RC => self.rc_view.window(ctx, &self.received_data),
                    WindowType::Chat => self.chat_view.window(ctx, &self.received_data),
                }
            }
        });
        ctx.request_repaint();
    }
}
