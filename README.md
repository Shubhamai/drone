# Drone [WIP]


https://github.com/Shubhamai/drone/assets/51819922/63be072e-84b0-4408-a207-5eb6f450ec99




## Introduction

This project is a drone flight system consisting of custom flight hardware, firmware, a Raspberry Pi-based communication bridge, and a ground control station. The ultimate goal is to build a fully autonomous drone capable of takeoff/landing on a target platform.

## System Architecture

The system is composed of three main components:

1. Flight Computer with [Firmware](./firmware/) (Custom PCB with teensy 4.1 microcontroller)
2. [Communication Bridge](./rpi/) (Raspberry Pi 4B)
3. [Ground Control Station](./ground/) (Desktop application using rust egui)

### 1. Flight Computer

The flight computer is built on a perfboard with a Teensy 4.1 running custom firmware written in C++. It handles real-time flight control, sensor fusion, and motor control.

#### Key Components:

- Teensy 4.1 microcontroller
- LSM6DSOX + LIS3DML IMU
- BMP390L Barometer
- ublox M9N GPS

#### Firmware Structure:

The firmware is modular and consists of several key components:

1. `main.cpp`: The entry point of the firmware, handling initialization and the main control loop.
2. `filter.h`: Implements sensor fusion using a Madgwick or Mahony filter to estimate the drone's orientation.
3. `pid.h`: Implements a simplified PID controller for only roll control for testing.
4. `state.h`: Manages the drone's state and LED indicators.
5. `transmitter.h`: Handles communication with the Raspberry Pi.
6. `consts.h`: Defines constants used throughout the firmware.
7. `barometer.h`: Interfaces with the barometric pressure sensor.
8. `imu.h`: Interfaces with the IMU sensor for accelerometer and gyroscope data.
9. `motor.h`: Controls the drone's motors based on the PID output. Has functions for arming, disarming, and setting motor speeds. I also contains hardware interrupt handlers if the signal is not received for a certain time.

### 2. Communication Bridge (Raspberry Pi)

The Raspberry Pi acts as a bridge between the flight computer and the ground control station. It runs a Rust application that:

1. Communicates with the flight computer via UART
2. Hosts a WebSocket server for real-time communication with the ground control station
3. Optionally processes computer vision tasks (e.g., ArUco marker detection)

#### Key Components:

- Raspberry Pi 4 Model B
- Camera module (for computer vision tasks)
- Wi-Fi module for communication with the ground station

#### Rust Application Structure:

```rust
use tokio::net::TcpListener;
use tokio_serial::SerialPortBuilderExt;
use tokio_tungstenite::accept_async;

async fn handle_connection(stream: TcpStream) {
    let ws_stream = accept_async(stream).await.unwrap();
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    let serial = tokio_serial::new("/dev/ttyS0", 1_000_000).open_native_async().unwrap();
    let (serial_reader, mut serial_writer) = tokio::io::split(serial);

    // ... (WebSocket and serial communication logic)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:8765").await?;

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream));
    }

    Ok(())
}
```

### 3. Ground Control Station

The ground control station is a desktop application written in Rust using the [egui](https://github.com/emilk/egui). It provides a user interface for monitoring telemetry, controlling the drone, and adjusting flight parameters.

#### Key Features:

- Real-time telemetry display
- 3D attitude visualization
- PID tuning interface
- RC control emulation
- Command interface for sending instructions to the drone

#### Rust Application Structure:

```rust
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

use app::MyApp;
use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_maximized(true),
        ..Default::default()
    };
    eframe::run_native(
        "Drone Control",
        options,
        Box::new(move |cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::new(MyApp::new())
        }),
    )
}
```

The `MyApp` struct manages the overall application state and UI layout:

```rust
pub struct MyApp {
    drone_view: DroneView,
    attitude_view: AttitudeView,
    accelerometer_view: AccelerometerView,
    rc_view: RCView,
    rc_control: RCControl,
    pid_control: PIDControlView,
    chat_view: ChatView,
    commands_view: CommandsView,
    received_data: Arc<Mutex<ReceivedData>>,
    // ... (other fields)
}
```

## Getting Started

To use this system with your own drone project:

1. Flight Computer Setup:

   - Flash the custom firmware onto your microcontroller
   - Connect sensors and motors according to the pin definitions in `consts.h`
   - Adjust PID parameters in `pid.h` for your drone's characteristics

2. Raspberry Pi Setup:

   - Install Rust locally and run `sudo apt install -y gcc-aarch64-linux-gnu`
   - Built the code using `cargo build --release` and copy the binary to the Raspberry Pi
   - Copy the binary to the Raspberry Pi and run the executable

3. Ground Control Station Setup:

   - Build and run the application using `cargo run --release`

## Future Improvements

- Implement full attitude control (pitch and yaw in addition to roll)
- Integrate computer vision for target tracking and landing using ArUco markers.
- Add GPS support for position hold and waypoint navigation

## License

This project is licensed under the MIT License - see the LICENSE file for details.
