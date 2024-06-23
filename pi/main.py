import cv2
import numpy as np
from picamera2 import Picamera2
import serial
import asyncio
import websockets
import threading
import base64
import json
import time

IP = "0.0.0.0"
PORT = 8765

print("Starting WebSocket server at ws://{}:{}".format(IP, PORT))


# ArUco detection function
def aruco_detection():
    global frame, ids
    cam = Picamera2()
    height = 480
    width = 640
    cam.configure(
        cam.create_video_configuration(
            main={"format": "RGB888", "size": (width, height)}
        )
    )
    cam.start()

    dictionary = cv2.aruco.Dictionary_get(cv2.aruco.DICT_5X5_100)
    detectorParams = cv2.aruco.DetectorParameters_create()

    while True:
        frame = cam.capture_array()
        (corners, ids, rejected) = cv2.aruco.detectMarkers(
            frame, dictionary, parameters=detectorParams
        )

        if ids is not None:
            ids = ids.flatten().tolist()
        else:
            ids = []

    cam.stop()


# WebSocket server
async def websocket_server(websocket, path):
    try:
        with serial.Serial("/dev/ttyS0", 115200) as ser:
            while True:
                start_time = time.time()
                # Read from serial
                serial_data = ser.readline().decode("utf-8").strip()

                # Encode the frame as JPEG
                # _, buffer = cv2.imencode('.jpg', frame)
                # jpg_as_text = base64.b64encode(buffer).decode('utf-8')

                # Combine ArUco IDs, serial data, and encoded frame
                combined_data = {
                    "aruco_ids": ids,
                    "serial_data": serial_data,
                    # "frame": jpg_as_text
                }

                # Send combined data via WebSocket
                await websocket.send(json.dumps(combined_data))

                # Receive data from WebSocket
                # try:
                #     received_data = await asyncio.wait_for(websocket.recv(), timeout=0.1)
                #     # Send received data to serial port
                #     ser.write(received_data.encode("utf-8"))
                # except asyncio.TimeoutError:
                #     pass  # No data received within timeout

                end_time = time.time()
                print("Time taken (ms):", (end_time - start_time) * 1000)

    except websockets.exceptions.ConnectionClosed:
        pass


# Start ArUco detection in a separate thread
frame = None
ids = []
aruco_thread = threading.Thread(target=aruco_detection)
aruco_thread.daemon = True
aruco_thread.start()

# Start WebSocket server
start_server = websockets.serve(websocket_server, IP, PORT)

# Run the event loop
asyncio.get_event_loop().run_until_complete(start_server)
asyncio.get_event_loop().run_forever()
