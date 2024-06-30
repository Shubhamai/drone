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

    float kp_r;
    float ki_r;
    float kd_r;

    float kp_p;
    float ki_p;
    float kd_p;
};

class TransmitterController
{
private:
    unsigned long lastTransmitTime;
    const unsigned long transmitInterval = 5; // Print every 100ms

    void sendData(TransmitterData data);

public:
    TransmitterController();

    void transmitData(TransmitterData data);
    String receiveData();
};

#endif