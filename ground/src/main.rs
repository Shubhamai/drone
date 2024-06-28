mod accelerometer_view;
mod app;
mod attitude_view;
mod chat_view;
mod commands_view;
mod data;
mod drone_view;
mod pid_view;
mod rc_control;
mod rc_view;
mod notes;

use app::MyApp;
use crossbeam_channel::{unbounded, Receiver, Sender};
use data::{ReceivedData, SerialData};
use eframe::egui;
use serde_json::Value;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tungstenite::{connect, Message};

const WEBSOCKET_URL: &str = "ws://192.168.1.107:8765";
const RECONNECT_INTERVAL: Duration = Duration::from_secs(5);

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let received_data = Arc::new(Mutex::new(ReceivedData::default()));
    let received_data_clone = Arc::clone(&received_data);

    let (ui_to_drone_tx, ui_to_drone_rx) = unbounded();
    let (drone_to_ui_tx, drone_to_ui_rx) = unbounded();

    let app = Arc::new(Mutex::new(MyApp::new(
        Arc::clone(&received_data),
        ui_to_drone_tx,
        ui_to_drone_rx,
        drone_to_ui_tx,
        drone_to_ui_rx,
    )));
    let app_clone = Arc::clone(&app);
    let drone_to_ui_sender = app.lock().unwrap().chat_view.drone_to_ui_tx.clone();
    let ui_to_drone_receiver = app.lock().unwrap().chat_view.ui_to_drone_rx.clone();

    thread::spawn(move || {
        loop {
            // let url = Url::parse(WEBSOCKET_URL).expect("Failed to parse WebSocket URL");
            match connect(WEBSOCKET_URL) {
                Ok((mut socket, _)) => {
                    println!("Connected to WebSocket server");
                    app_clone.lock().unwrap().update_connection_status(true);

                    loop {
                        match socket.read() {
                            Ok(msg) => {
                                let msg_str = msg.to_string();
                                let json_value: Value =
                                    serde_json::from_str(&msg_str).expect("Error parsing JSON");

                                // Update the last received time
                                app_clone.lock().unwrap().update_last_received_time();

                                if let Some(aruco_ids) = json_value["aruco_ids"].as_array() {
                                    let deserialized_ids: Vec<u32> = aruco_ids
                                        .iter()
                                        .map(|id| id.as_u64().unwrap() as u32)
                                        .collect();

                                    let mut data = received_data_clone.lock().unwrap();
                                    data.aruco_ids = deserialized_ids;
                                }

                                if let Some(serial_data_str) = json_value["serial_data"].as_str() {
                                    if let Ok(serial_data) =
                                        serde_json::from_str::<SerialData>(serial_data_str)
                                    {
                                        let mut data = received_data_clone.lock().unwrap();
                                        data.serial_data = serial_data;
                                    } else {
                                        // If it's not SerialData, treat it as a chat message, if it does not have `":`

                                        if !serial_data_str.contains(":") {
                                            drone_to_ui_sender
                                                .send(serial_data_str.to_string())
                                                .expect("Failed to send chat message");
                                        }
                                    }
                                }

                                if let Ok(ui_message) = ui_to_drone_receiver.try_recv() {
                                    if socket.can_write() {
                                        match socket.send(Message::Text(ui_message)) {
                                            Ok(_) => {}
                                            Err(e) => println!("Failed to send message: {:?}", e),
                                        }
                                    } else {
                                        println!("Socket is not ready for writing");
                                    }
                                }
                            }
                            Err(e) => {
                                println!("Error reading from WebSocket: {:?}", e);
                                break;
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("Failed to connect to WebSocket server: {:?}", e);
                }
            }

            app_clone.lock().unwrap().update_connection_status(false);
            app_clone.lock().unwrap().increment_connection_attempts();
            println!(
                "Attempting to reconnect in {} seconds...",
                RECONNECT_INTERVAL.as_secs()
            );
            thread::sleep(RECONNECT_INTERVAL);
        }
    });

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder {
            maximized: Some(true),
            title: Some("Drone Control".to_string()),
            ..Default::default()
        },
        // hardware_acceleration: eframe::HardwareAcceleration::Off,
        // renderer: eframe::Renderer::Wgpu,
        ..Default::default()
    };
    eframe::run_native(
        "Drone Control",
        options,
        Box::new(move |cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::new(app.lock().unwrap().clone())
        }),
    )
}
