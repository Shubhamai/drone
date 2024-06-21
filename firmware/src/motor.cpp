#include "motor.h"
#include "receiver.h"

MotorController::MotorController(int frPin, int brPin, int blPin, int flPin)
    : frontRightPin(frPin), backRightPin(brPin), backLeftPin(blPin), frontLeftPin(flPin),
      frontRightThrust(MIN_THROTTLE), backRightThrust(MIN_THROTTLE),
      backLeftThrust(MIN_THROTTLE), frontLeftThrust(MIN_THROTTLE) {}

void MotorController::initialize()
{
    pinMode(frontRightPin, OUTPUT);
    pinMode(backRightPin, OUTPUT);
    pinMode(backLeftPin, OUTPUT);
    pinMode(frontLeftPin, OUTPUT);

    writeThrust(frontRightPin, MIN_THROTTLE);
    writeThrust(backRightPin, MIN_THROTTLE);
    writeThrust(backLeftPin, MIN_THROTTLE);
    writeThrust(frontLeftPin, MIN_THROTTLE);
}

void MotorController::writeThrust(int pin, int thrust)
{
    if (DISABLE_MOTORS || thrust < MIN_THROTTLE || thrust > MAX_THROTTLE)
    {
        thrust = MIN_THROTTLE;
    }

    // Map thrust from 1000-2000 to 0-180 for analogWrite
    int mappedThrust = map(thrust, 1000, 2000, 0, 180);

    analogWrite(pin, mappedThrust);
}

bool MotorController::setThrust(int motor, int thrust)
{
    if (thrust < MIN_THROTTLE || thrust > MAX_THROTTLE)
    {
        return false; // Invalid thrust value
    }

    switch (motor)
    {
    case 1:
        frontRightThrust = thrust;
        writeThrust(frontRightPin, thrust);
        break;
    case 2:
        backRightThrust = thrust;
        writeThrust(backRightPin, thrust);
        break;
    case 3:
        backLeftThrust = thrust;
        writeThrust(backLeftPin, thrust);
        break;
    case 4:
        frontLeftThrust = thrust;
        writeThrust(frontLeftPin, thrust);
        break;
    default:
        return false; // Invalid motor number
    }

    return true;
}

int MotorController::getThrust(int motor) const
{
    switch (motor)
    {
    case 1:
        return frontRightThrust;
    case 2:
        return backRightThrust;
    case 3:
        return backLeftThrust;
    case 4:
        return frontLeftThrust;
    default:
        return -1; // Invalid motor number
    }
}

// New function implementations

bool MotorController::setAllThrust(int frThrust, int brThrust, int blThrust, int flThrust)
{
    if (frThrust < MIN_THROTTLE || frThrust > MAX_THROTTLE ||
        brThrust < MIN_THROTTLE || brThrust > MAX_THROTTLE ||
        blThrust < MIN_THROTTLE || blThrust > MAX_THROTTLE ||
        flThrust < MIN_THROTTLE || flThrust > MAX_THROTTLE)
    {
        return false; // Invalid thrust value(s)
    }

    frontRightThrust = frThrust;
    backRightThrust = brThrust;
    backLeftThrust = blThrust;
    frontLeftThrust = flThrust;

    writeThrust(frontRightPin, frThrust);
    writeThrust(backRightPin, brThrust);
    writeThrust(backLeftPin, blThrust);
    writeThrust(frontLeftPin, flThrust);

    return true;
}

void MotorController::getAllThrust(int &frThrust, int &brThrust, int &blThrust, int &flThrust) const
{
    frThrust = frontRightThrust;
    brThrust = backRightThrust;
    blThrust = backLeftThrust;
    flThrust = frontLeftThrust;
}

void MotorController::disableMotors()
{
    DISABLE_MOTORS = true;
    writeThrust(frontRightPin, MIN_THROTTLE);
    writeThrust(backRightPin, MIN_THROTTLE);
    writeThrust(backLeftPin, MIN_THROTTLE);
    writeThrust(frontLeftPin, MIN_THROTTLE);
}

void MotorController::enableMotors(ReceiverController &receiver)
{
    // make sure the throttle is zero before enabling motors
    if (receiver.isThrottleZero())
    {
        DISABLE_MOTORS = false;
    }
}