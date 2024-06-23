#ifndef RECEIVER_CONTROLLER_H
#define RECEIVER_CONTROLLER_H

#include <Arduino.h>

class ReceiverController
{
private:
    static const int MIN_PULSE = 1000;
    static const int MAX_PULSE = 2000;
    static const int CHANNEL_COUNT = 4;
    static const int THROTTLE_HISTORY_SIZE = 100;  // Store 10 samples over 1000ms
    static const int THROTTLE_UPDATE_INTERVAL = 5; // Update every 5ms

    int pins[CHANNEL_COUNT];
    volatile int values[CHANNEL_COUNT];
    bool isInitialized;
    volatile unsigned long pulseStartTime[CHANNEL_COUNT];

    int throttleHistory[THROTTLE_HISTORY_SIZE];
    unsigned long lastThrottleUpdateTime;
    int throttleHistoryIndex;

    static ReceiverController *instance;

    int constrainPulse(int pulse);
    static void throttleInterruptHandler();
    static void rollInterruptHandler();
    static void pitchInterruptHandler();
    static void yawInterruptHandler();
    void handleInterrupt(int channel);
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