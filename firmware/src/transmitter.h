#ifndef TRANSMITTER_CONTROLLER_H
#define TRANSMITTER_CONTROLLER_H

#include <Arduino.h>

struct TransmitterData
{
    float elapsedTime;

    float accX;
    float accY;
    float accZ;

    float gyroX;
    float gyroY;
    float gyroZ;

    float magX;
    float magY;
    float magZ;

    float altitude;

    float temp;

    int32_t yaw;
    int32_t pitch;
    int32_t roll;

    int32_t rcThrottle;
    int32_t rcYaw;
    int32_t rcPitch;
    int32_t rcRoll;

    int32_t frontRight;
    int32_t backRight;
    int32_t backLeft;
    int32_t frontLeft;
};

class TransmitterController
{
private:
    unsigned long lastPrintTime;
    const unsigned long printInterval = 20; // Print every 100ms

public:
    TransmitterController();

    void update(TransmitterData data);
    void sendValues(TransmitterData data);
};

#endif