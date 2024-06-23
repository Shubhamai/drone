#include "receiver.h"

ReceiverController *ReceiverController::instance = nullptr;

ReceiverController::ReceiverController(int throttlePin, int rollPin, int pitchPin, int yawPin)
{
    pins[0] = throttlePin;
    pins[1] = rollPin;
    pins[2] = pitchPin;
    pins[3] = yawPin;

    for (int i = 0; i < CHANNEL_COUNT; i++)
    {
        pinMode(pins[i], INPUT);
        values[i] = MIN_PULSE;
        pulseStartTime[i] = 0;
    }

    isInitialized = false;
    instance = this;

    // Initialize throttle history
    for (int i = 0; i < THROTTLE_HISTORY_SIZE; i++)
    {
        throttleHistory[i] = MIN_PULSE;
    }
    lastThrottleUpdateTime = 0;
    throttleHistoryIndex = 0;
}

void ReceiverController::initialize()
{
    attachInterrupt(digitalPinToInterrupt(pins[0]), throttleInterruptHandler, CHANGE);
    attachInterrupt(digitalPinToInterrupt(pins[1]), rollInterruptHandler, CHANGE);
    attachInterrupt(digitalPinToInterrupt(pins[2]), pitchInterruptHandler, CHANGE);
    attachInterrupt(digitalPinToInterrupt(pins[3]), yawInterruptHandler, CHANGE);
}

void ReceiverController::throttleInterruptHandler() { instance->handleInterrupt(0); }
void ReceiverController::rollInterruptHandler() { instance->handleInterrupt(1); }
void ReceiverController::pitchInterruptHandler() { instance->handleInterrupt(2); }
void ReceiverController::yawInterruptHandler() { instance->handleInterrupt(3); }

void ReceiverController::handleInterrupt(int channel)
{
    if (digitalRead(instance->pins[channel]) == HIGH)
    {
        instance->pulseStartTime[channel] = micros();
    }
    else
    {
        unsigned long duration = micros() - instance->pulseStartTime[channel];
        instance->values[channel] = instance->constrainPulse(duration);
    }
}

bool ReceiverController::update()
{
    if (!isInitialized && isThrottleZero())
    {
        isInitialized = true;
    }

    // Update throttle history
    unsigned long currentTime = millis();
    if (currentTime - lastThrottleUpdateTime >= THROTTLE_UPDATE_INTERVAL)
    {
        throttleHistory[throttleHistoryIndex] = values[0];
        throttleHistoryIndex = (throttleHistoryIndex + 1) % THROTTLE_HISTORY_SIZE;
        lastThrottleUpdateTime = currentTime;
    }

    // Check if all throttle values in the last 1000ms are the same
    bool allThrottleValuesSame = true;
    int firstThrottleValue = throttleHistory[0];
    for (int i = 1; i < THROTTLE_HISTORY_SIZE; i++)
    {
        if (throttleHistory[i] != firstThrottleValue)
        {
            allThrottleValuesSame = false;
            break;
        }
    }

    if (allThrottleValuesSame)
    {
        return false;
    }

    return isEnabled();
}

bool ReceiverController::isThrottleZero()
{
    return values[0] >= 1100 && values[0] <= 1110;
}

int ReceiverController::getThrottle()
{
    return map(isInitialized ? values[0] : MIN_PULSE, 1100, 1750, 1000, 2000);
}

int ReceiverController::getRoll()
{
    return map(isInitialized ? values[1] : MIN_PULSE, 1035, 1810, 1000, 2000);
}

int ReceiverController::getPitch()
{
    return map(isInitialized ? values[2] : MIN_PULSE, 1100, 1750, 1000, 2000);
}

int ReceiverController::getYaw()
{
    return map(isInitialized ? values[3] : MIN_PULSE, 1035, 1810, 1000, 2000);
}

int ReceiverController::constrainPulse(int pulse)
{
    return constrain(pulse, MIN_PULSE, MAX_PULSE);
}

bool ReceiverController::isEnabled()
{
    // If the throttle switch is in down position, the receiver is enabled
    return (values[0] < 1010 && values[0] > 990) ? false : true;
}