#ifndef FAKE_RECEIVER_CONTROLLER_H
#define FAKE_RECEIVER_CONTROLLER_H

#include <Arduino.h>

class FakeReceiverController
{
private:
    int throttle;
    int roll;
    int pitch;
    int yaw;
    bool enabled;
    bool initialized;

public:
    FakeReceiverController() : throttle(1000), roll(1500), pitch(1500), yaw(1500), enabled(false), initialized(false) {}

    void initialize() { initialized = true; }

    bool update()
    {
        // Simulate RC data update
        // In a real scenario, you would update these values based on your simulation needs
        return enabled;
    }

    bool isThrottleZero() { return throttle >= 1100 && throttle <= 1110; }

    int getThrottle() { return throttle; }
    int getRoll() { return roll; }
    int getPitch() { return pitch; }
    int getYaw() { return yaw; }

    void setThrottle(int value) { throttle = constrain(value, 1000, 2000); }
    void setRoll(int value) { roll = constrain(value, 1000, 2000); }
    void setPitch(int value) { pitch = constrain(value, 1000, 2000); }
    void setYaw(int value) { yaw = constrain(value, 1000, 2000); }

    void setEnabled(bool value) { enabled = value; }

    void parseRCValues(const String& input)
    {
        if (input.startsWith("rc->"))
        {
            String rcValues = input.substring(4);
            int commaIndex = rcValues.indexOf(',');
            throttle = rcValues.substring(0, commaIndex).toInt();
            rcValues = rcValues.substring(commaIndex + 1);
            commaIndex = rcValues.indexOf(',');
            yaw = rcValues.substring(0, commaIndex).toInt();
            rcValues = rcValues.substring(commaIndex + 1);
            commaIndex = rcValues.indexOf(',');
            pitch = rcValues.substring(0, commaIndex).toInt();
            roll = rcValues.substring(commaIndex + 1).toInt();
        }
    }
};

#endif