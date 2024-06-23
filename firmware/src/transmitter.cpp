#include "transmitter.h"
#include "consts.h"
#include <ArduinoJson.h>

TransmitterController::TransmitterController() : lastPrintTime(0) {}

void TransmitterController::update(TransmitterData data)
{
    unsigned long elapsed = millis();

    if (elapsed - lastPrintTime >= printInterval)
    {
        sendValues(data);
        lastPrintTime = elapsed;

        if (TRANSMITTER_SERIAL.available())
        {
            String input = TRANSMITTER_SERIAL.readStringUntil('\n');
            if (input == "exit")
            {
                DEBUG_SERIAL.println("Exiting...");
                TRANSMITTER_SERIAL.println("Exiting...");
                while (true)
                    ;
            }
        }
    }
}

void TransmitterController::sendValues(TransmitterData data)
{
    StaticJsonDocument<512> doc;

    doc["elapsed_time"] = data.elapsedTime;
    doc["acc_x"] = data.accX;
    doc["acc_y"] = data.accY;
    doc["acc_z"] = data.accZ;
    doc["gyro_x"] = data.gyroX;
    doc["gyro_y"] = data.gyroY;
    doc["gyro_z"] = data.gyroZ;
    doc["mag_x"] = data.magX;
    doc["mag_y"] = data.magY;
    doc["mag_z"] = data.magZ;
    doc["altitude"] = data.altitude;
    doc["temp"] = data.temp;
    doc["yaw"] = data.yaw;
    doc["pitch"] = data.pitch;
    doc["roll"] = data.roll;
    doc["rc_throttle"] = data.rcThrottle;
    doc["rc_yaw"] = data.rcYaw;
    doc["rc_pitch"] = data.rcPitch;
    doc["rc_roll"] = data.rcRoll;
    doc["front_right"] = data.frontRight;
    doc["back_right"] = data.backRight;
    doc["back_left"] = data.backLeft;
    doc["front_left"] = data.frontLeft;

    String jsonString;
    serializeJson(doc, jsonString);
    TRANSMITTER_SERIAL.println(jsonString);
}