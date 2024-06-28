use crate::accelerometer_view::AccelerometerView;
use crate::attitude_view::AttitudeView;
use crate::chat_view::ChatView;
use crate::commands_view::CommandsView;
use crate::data::ReceivedData;
use crate::drone_view::DroneView;
use crate::notes::NoteEditorView;
use crate::pid_view::PIDControlView;
use crate::rc_control::RCControl;
use crate::rc_view::RCView;
use chrono::Local;
use crossbeam_channel::{unbounded, Receiver, Sender};
use eframe::egui::{self, RichText};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Clone)]

pub struct MyApp {
    drone_view: DroneView,
    attitude_view: AttitudeView,
    accelerometer_view: AccelerometerView,
    rc_view: RCView,
    rc_control: RCControl,
    pid_control: PIDControlView,
    notes: NoteEditorView,
    pub chat_view: ChatView,
    commands_view: CommandsView,
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
    RCView,
    Chat,
    Commands,
    RCControl,
    PIDControl,
    Notes,
}

impl MyApp {
    pub fn new(
        received_data: Arc<Mutex<ReceivedData>>,
        ui_to_drone_tx: Sender<String>,
        ui_to_drone_rx: Receiver<String>,
        drone_to_ui_tx: Sender<String>,
        drone_to_ui_rx: Receiver<String>,
    ) -> Self {
        Self {
            drone_view: DroneView::default(),
            attitude_view: AttitudeView::default(),
            accelerometer_view: AccelerometerView::new(),
            rc_view: RCView::default(),
            rc_control: RCControl::new(ui_to_drone_tx.clone()),
            chat_view: ChatView::new(
                ui_to_drone_tx.clone(),
                ui_to_drone_rx.clone(),
                drone_to_ui_tx.clone(),
                drone_to_ui_rx.clone(),
            ),
            commands_view: CommandsView::new(
                ui_to_drone_tx.clone(),
                ui_to_drone_rx,
                drone_to_ui_tx,
                drone_to_ui_rx,
            ),
            pid_control: PIDControlView::new(ui_to_drone_tx),
            notes: NoteEditorView::new(PathBuf::from("/home/elden/Documents/projects/jet/ground/notes.txt")),
            received_data,
            start_time: Instant::now(),
            last_received_time: Arc::new(Mutex::new(Instant::now())),
            tabs: vec![Tab {
                name: "Tab 1".to_string(),
                windows: vec![WindowType::Commands],
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
                if ui.button("Open All").clicked() {
                    self.tabs[self.active_tab].windows = vec![
                        WindowType::Drone,
                        WindowType::Attitude,
                        WindowType::Accelerometer,
                        WindowType::RCView,
                        WindowType::Chat,
                        WindowType::Commands,
                        WindowType::RCControl,
                        WindowType::PIDControl,
                        WindowType::Notes,
                    ];
                }

                if ui.button("Close All").clicked() {
                    self.tabs[self.active_tab].windows = Vec::new();
                }

                if ui
                    .button(
                        // "Drone View"
                        RichText::new("Drone View"), // .background_color(if self.tabs[self.active_tab]
                                                     //     .windows
                                                     //     .iter()
                                                     //     .any(|w| *w == WindowType::Drone)
                                                     // {
                                                     //     egui::Color32::BLACK
                                                     // } else {
                                                     //     egui::Color32::GRAY
                                                     // }),
                    )
                    .clicked()
                {
                    if self.tabs[self.active_tab]
                        .windows
                        .iter()
                        .any(|w| *w == WindowType::Drone)
                    {
                        self.tabs[self.active_tab]
                            .windows
                            .retain(|w| *w != WindowType::Drone);
                    } else {
                        self.tabs[self.active_tab].windows.push(WindowType::Drone);
                    }
                }
                if ui.button("Attitude View").clicked() {
                    if self.tabs[self.active_tab]
                        .windows
                        .iter()
                        .any(|w| *w == WindowType::Attitude)
                    {
                        self.tabs[self.active_tab]
                            .windows
                            .retain(|w| *w != WindowType::Attitude);
                    } else {
                        self.tabs[self.active_tab]
                            .windows
                            .push(WindowType::Attitude);
                    }
                }
                if ui.button("Accelerometer View").clicked() {
                    if self.tabs[self.active_tab]
                        .windows
                        .iter()
                        .any(|w| *w == WindowType::Accelerometer)
                    {
                        self.tabs[self.active_tab]
                            .windows
                            .retain(|w| *w != WindowType::Accelerometer);
                    } else {
                        self.tabs[self.active_tab]
                            .windows
                            .push(WindowType::Accelerometer);
                    }
                }
                if ui.button("RC View").clicked() {
                    if self.tabs[self.active_tab]
                        .windows
                        .iter()
                        .any(|w| *w == WindowType::RCView)
                    {
                        self.tabs[self.active_tab]
                            .windows
                            .retain(|w| *w != WindowType::RCView);
                    } else {
                        self.tabs[self.active_tab].windows.push(WindowType::RCView);
                    }
                }
                if ui.button("RC Control").clicked() {
                    if self.tabs[self.active_tab]
                        .windows
                        .iter()
                        .any(|w| *w == WindowType::RCControl)
                    {
                        self.tabs[self.active_tab]
                            .windows
                            .retain(|w| *w != WindowType::RCControl);
                    } else {
                        self.tabs[self.active_tab]
                            .windows
                            .push(WindowType::RCControl);
                    }
                }
                if ui.button("Chat").clicked() {
                    if self.tabs[self.active_tab]
                        .windows
                        .iter()
                        .any(|w| *w == WindowType::Chat)
                    {
                        self.tabs[self.active_tab]
                            .windows
                            .retain(|w| *w != WindowType::Chat);
                    } else {
                        self.tabs[self.active_tab].windows.push(WindowType::Chat);
                    }
                }
                if ui.button("Commands").clicked() {
                    if self.tabs[self.active_tab]
                        .windows
                        .iter()
                        .any(|w| *w == WindowType::Commands)
                    {
                        self.tabs[self.active_tab]
                            .windows
                            .retain(|w| *w != WindowType::Commands);
                    } else {
                        self.tabs[self.active_tab]
                            .windows
                            .push(WindowType::Commands);
                    }
                }
                if ui.button("PID Control").clicked() {
                    if self.tabs[self.active_tab]
                        .windows
                        .iter()
                        .any(|w| *w == WindowType::PIDControl)
                    {
                        self.tabs[self.active_tab]
                            .windows
                            .retain(|w| *w != WindowType::PIDControl);
                    } else {
                        self.tabs[self.active_tab]
                            .windows
                            .push(WindowType::PIDControl);
                    }
                }
                if ui.button("Notes").clicked() {
                    if self.tabs[self.active_tab]
                        .windows
                        .iter()
                        .any(|w| *w == WindowType::Notes)
                    {
                        self.tabs[self.active_tab]
                            .windows
                            .retain(|w| *w != WindowType::Notes);
                    } else {
                        self.tabs[self.active_tab].windows.push(WindowType::Notes);
                    }
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
                    WindowType::RCView => self.rc_view.window(ctx, &self.received_data),
                    WindowType::Chat => self.chat_view.window(ctx, &self.received_data),
                    WindowType::Commands => self.commands_view.window(ctx, &self.received_data),
                    WindowType::RCControl => self.rc_control.window(ctx),
                    WindowType::PIDControl => self.pid_control.window(ctx, &self.received_data),
                    WindowType::Notes => self.notes.window(ctx),
                }
            }
        });
        ctx.request_repaint();
    }
}
