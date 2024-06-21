#ifndef RECEIVER_CONTROLLER_H
#define RECEIVER_CONTROLLER_H

#include <Arduino.h>

class ReceiverController {
private:
    static const int MIN_PULSE = 1000;
    static const int MAX_PULSE = 2000;
    static const int CHANNEL_COUNT = 4;  // Assuming 4 channels: throttle, roll, pitch, yaw
    static const int THROTTLE_HISTORY_SIZE = 100; // Store 10 samples over 1000ms
    static const int THROTTLE_UPDATE_INTERVAL = 5; // Update every 5ms

    int pins[CHANNEL_COUNT];
    volatile int values[CHANNEL_COUNT];
    bool isInitialized;
    volatile unsigned long pulseStartTime[CHANNEL_COUNT];
    volatile bool newPulseAvailable[CHANNEL_COUNT];

    // New members for throttle history
    int throttleHistory[THROTTLE_HISTORY_SIZE];
    unsigned long lastThrottleUpdateTime;
    int throttleHistoryIndex;

    int constrainPulse(int pulse);
    static void pinInterruptHandler();
    bool isEnabled();

public:
    ReceiverController(int throttlePin, int rollPin, int pitchPin, int yawPin);
    bool update();
    void initialize();
    bool isThrottleZero();
    int getThrottle();
    int getRoll();
    int getPitch();
    int getYaw();
};

#endif