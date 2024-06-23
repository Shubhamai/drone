mod app;
mod data;
mod drone_view;
mod attitude_view;
mod accelerometer_view;

use app::MyApp;
use data::ReceivedData;
use eframe::egui;
use std::sync::{Arc, Mutex};
use std::thread;
use tungstenite::connect;
use url::Url;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let received_data = Arc::new(Mutex::new(ReceivedData::default()));
    let received_data_clone = Arc::clone(&received_data);

    thread::spawn(move || {
        let url = Url::parse("ws://192.168.1.107:8765").expect("Failed to parse WebSocket URL");
        let (mut socket, _) = connect(url.to_string()).expect("Can't connect to WebSocket server");

        loop {
            let msg = socket.read().expect("Error reading message");
            let msg_str = msg.to_string();
            let deserialized_msg: ReceivedData = serde_json::from_str(&msg_str)
                .expect("Error deserializing JSON");

            let mut data = received_data_clone.lock().unwrap();
            *data = deserialized_msg;
        }
    });

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Drone Control",
        options,
        Box::new(move |cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::new(MyApp::new(Arc::clone(&received_data)))
        }),
    )
}