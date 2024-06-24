import cv2
import numpy as np
from picamera2 import Picamera2
import serial
import asyncio
import websockets
import multiprocessing as mp
import base64
import json
import time
import signal
from concurrent.futures import ThreadPoolExecutor

IP = "0.0.0.0"
PORT = 8765

print(f"Starting WebSocket server at ws://{IP}:{PORT}")

shutdown_event = mp.Event()

def aruco_detection(frame_queue, ids_queue):
    cam = Picamera2()
    height, width = 480, 640
    cam.configure(cam.create_video_configuration(main={"format": "RGB888", "size": (width, height)}))
    cam.start()

    dictionary = cv2.aruco.Dictionary_get(cv2.aruco.DICT_5X5_100)
    detectorParams = cv2.aruco.DetectorParameters_create()

    while not shutdown_event.is_set():
        frame = cam.capture_array()
        (corners, ids, rejected) = cv2.aruco.detectMarkers(frame, dictionary, parameters=detectorParams)

        if not frame_queue.full():
            frame_queue.put_nowait(frame)
        if not ids_queue.full():
            ids_queue.put_nowait(ids.flatten().tolist() if ids is not None else [])

    cam.stop()
    print("ArUco detection process stopped")

def process_frame(frame):
    _, buffer = cv2.imencode('.jpg', frame, [cv2.IMWRITE_JPEG_QUALITY, 80])
    return base64.b64encode(buffer).decode('utf-8')

async def websocket_handler(websocket, path, frame_queue, ids_queue):
    executor = ThreadPoolExecutor(max_workers=2)
    loop = asyncio.get_event_loop()
    last_frame_time = 0
    frame_interval = 0.1  # 100ms, adjust as needed

    try:
        with serial.Serial("/dev/ttyS0", 1000000, timeout=0) as ser:
            while not shutdown_event.is_set():
                current_time = time.time()

                # Read from serial (non-blocking)
                serial_data = ser.readline().decode("utf-8").strip()
                # serial_data = ser.read(ser.in_waiting).decode("utf-8").strip()

                # Get latest frame and IDs if available
                # frame = None
                ids = []
                # if not frame_queue.empty() and current_time - last_frame_time >= frame_interval:
                #     frame = frame_queue.get_nowait()
                #     last_frame_time = current_time
                # if not ids_queue.empty():
                #     ids = ids_queue.get_nowait()

                # Process frame if available
                # jpg_as_text = ""
                # if frame is not None:
                #     jpg_as_text = await loop.run_in_executor(executor, process_frame, frame)

                # Combine ArUco IDs, serial data, and encoded frame
                combined_data = {
                    "aruco_ids": ids,
                    "serial_data": serial_data,
                    # "frame": jpg_as_text
                }

                # Send combined data via WebSocket
                await websocket.send(json.dumps(combined_data))

                # Receive data from WebSocket (non-blocking)
                try:
                    received_data = await asyncio.wait_for(websocket.recv(), timeout=0.001)
                    print(f"Received data: {received_data}")
                    ser.write(received_data.encode("utf-8"))
                except asyncio.TimeoutError:
                    pass
                
                print("done")
                # # Ensure we're not processing faster than the Arduino's 20ms interval
                # elapsed_time = time.time() - current_time
                # if elapsed_time < 0.02:
                #     await asyncio.sleep(0.02 - elapsed_time)

    except websockets.exceptions.ConnectionClosed:
        pass
    finally:
        executor.shutdown(wait=False)

async def shutdown(signal, loop):
    print(f"Received exit signal {signal.name}...")
    shutdown_event.set()
    tasks = [t for t in asyncio.all_tasks() if t is not asyncio.current_task()]
    [task.cancel() for task in tasks]
    await asyncio.gather(*tasks, return_exceptions=True)
    loop.stop()

def main():
    frame_queue = mp.Queue(maxsize=2)
    ids_queue = mp.Queue(maxsize=2)

    aruco_process = mp.Process(target=aruco_detection, args=(frame_queue, ids_queue))
    aruco_process.start()

    loop = asyncio.get_event_loop()

    signals = (signal.SIGHUP, signal.SIGTERM, signal.SIGINT)
    for s in signals:
        loop.add_signal_handler(
            s, lambda s=s: asyncio.create_task(shutdown(s, loop))
        )

    server = websockets.serve(
        lambda ws, path: websocket_handler(ws, path, frame_queue, ids_queue),
        IP, PORT
    )

    try:
        loop.run_until_complete(server)
        loop.run_forever()
    finally:
        loop.close()
        aruco_process.join()
        print("Shutdown complete.")

if __name__ == "__main__":
    main()