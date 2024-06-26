use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio_serial::SerialPortBuilderExt;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;

const IP: &str = "0.0.0.0";
const PORT: u16 = 8765;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting WebSocket server at ws://{}:{}", IP, PORT);

    let listener = TcpListener::bind(format!("{}:{}", IP, PORT)).await?;

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream));
    }

    Ok(())
}

async fn handle_connection(stream: TcpStream) {
    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            eprintln!("Failed to accept WebSocket connection: {}", e);
            return;
        }
    };

    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    let tty_path = "/dev/ttyS0";
    let serial = match tokio_serial::new(tty_path, 1_000_000).open_native_async() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to open serial port: {}", e);
            return;
        }
    };

    let (serial_reader, mut serial_writer) = tokio::io::split(serial);
    let mut serial_reader = BufReader::new(serial_reader);

    // Channel for sending data from serial reader to WebSocket sender
    let (tx, mut rx) = mpsc::channel::<String>(100);

    // Shared state for signaling tasks to stop
    let stop_signal = Arc::new(AtomicBool::new(false));

    // Spawn a task to read from serial and send to channel
    let serial_task = {
        let stop_signal = Arc::clone(&stop_signal);
        tokio::spawn(async move {
            let mut line = String::new();
            while !stop_signal.load(Ordering::Relaxed) {
                tokio::select! {
                    result = serial_reader.read_line(&mut line) => {
                        match result {
                            Ok(0) => break, // EOF
                            Ok(_) => {
                                if tx.send(line.clone()).await.is_err() {
                                    break;
                                }
                                line.clear();
                            }
                            Err(e) => {
                                eprintln!("Error reading from serial: {}", e);
                                break;
                            }
                        }
                    }
                    _ = tokio::time::sleep(tokio::time::Duration::from_millis(100)) => {
                        // This branch ensures we periodically check the stop_signal
                    }
                }
            }
            println!("Stopping serial read task");
        })
    };

    // Spawn a task to receive from channel and send to WebSocket
    let ws_task = tokio::spawn(async move {
        while let Some(line) = rx.recv().await {
            let aruco_ids = vec![1, 2, 3, 4, 5]; // TODO: Replace with actual Aruco ID detection
            let combined_data = json!({
                "aruco_ids": aruco_ids,
                "serial_data": line.trim(),
            });
            if let Err(e) = ws_sender
                .send(Message::Text(combined_data.to_string()))
                .await
            {
                eprintln!("WebSocket send error: {}", e);
                break;
            }
        }
    });

    // Handle WebSocket messages and write to serial
    while let Some(msg) = ws_receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                println!("Received: {}", text);
                if let Err(e) = serial_writer.write_all(text.as_bytes()).await {
                    eprintln!("Serial write error: {}", e);
                    break;
                }
            }
            Ok(Message::Close(_)) => {
                println!("WebSocket connection closed");
                break;
            }
            Err(e) => {
                eprintln!("WebSocket receive error: {}", e);
                break;
            }
            _ => {}
        }
    }

    // Signal tasks to stop
    stop_signal.store(true, Ordering::Relaxed);

    // Wait for tasks to finish
    let _ = serial_task.await;
    let _ = ws_task.await;

    println!("Connection handler finished");
}
// sudo apt install -y gcc-aarch64-linux-gnu # local machine
// cargo build --target aarch64-unknown-linux-gnu
// scp /home/elden/Documents/projects/jet/rpi/target/aarch64-unknown-linux-gnu/release/rpi elden@192.168.1.107:~/Documents/jet/rpi
