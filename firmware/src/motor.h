#ifndef MOTOR_CONTROLLER_H
#define MOTOR_CONTROLLER_H

#include <Arduino.h>
#include "consts.h"
#include "receiver.h"

class MotorController
{
private:
    const int frontRightPin;
    const int backRightPin;
    const int backLeftPin;
    const int frontLeftPin;

    int frontRightThrust;
    int backRightThrust;
    int backLeftThrust;
    int frontLeftThrust;

    void writeThrust(int pin, int thrust);

    bool DISABLE_MOTORS = false;

public:
    MotorController(int frPin, int brPin, int blPin, int flPin);

    void initialize();
    bool setThrust(int motor, int thrust);
    int getThrust(int motor) const;

    // New functions
    bool setAllThrust(int frThrust, int brThrust, int blThrust, int flThrust);
    void getAllThrust(int &frThrust, int &brThrust, int &blThrust, int &flThrust) const;

    void disableMotors();
    void enableMotors(ReceiverController &receiver);
};

#endif