#ifndef MOTOR_CONTROLLER_H
#define MOTOR_CONTROLLER_H

#include <Arduino.h>
#include <TimerOne.h>
#include "consts.h"
#include "receiver.h"
#include "fake_receiver.h"

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
    unsigned long lastThrustUpdateTime;
    static const unsigned long THRUST_TIMEOUT = 200000; // 200ms timeout in microseconds
    bool isInitialized;

    void checkAndDisableMotors();

    static MotorController *instance;
    static void checkMotorsWrapper();

public:
    MotorController(int frPin, int brPin, int blPin, int flPin);

    void initialize();
    bool setThrust(int motor, int thrust);
    int getThrust(int motor) const;

    bool setAllThrust(int frThrust, int brThrust, int blThrust, int flThrust);
    void getAllThrust(int &frThrust, int &brThrust, int &blThrust, int &flThrust) const;

    void disableMotors();
    void enableMotors(ReceiverController &receiver);
    void enableMotors(FakeReceiverController &receiver);

    void setupTimer();
};

#endif