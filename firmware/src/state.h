#ifndef STATE_CONTROLLER_H
#define STATE_CONTROLLER_H

#include <Arduino.h>
#include <Adafruit_NeoPixel.h>

#include "consts.h"

class StateController
{
public:
    StateController();
    void initialize();
    void update();

private:
    Adafruit_NeoPixel frontRightLed;
    Adafruit_NeoPixel backRightLed;
    Adafruit_NeoPixel backLeftLed;
    Adafruit_NeoPixel frontLeftLed;

    void initializeLEDs();
    void initializeBuzzer();
    void updateLEDs();
    void updateBuzzer();
    void setNeoPixedRGB(uint8_t r, uint8_t g, uint8_t b);

    bool ledState;
    unsigned long lastLEDToggle;
    const unsigned long LED_TOGGLE_INTERVAL = 1000; // 1 second

    int stateNeoPixelRGB = 0;
    unsigned long previousMillisNeoPixelLed = 10000;
    const unsigned long interval1RGB = 600; // 1 second
    const unsigned long interval2RGB = 200;  // 0.5 second

    unsigned long lastBuzzerToggle;
    const unsigned long BUZZER_TOGGLE_INTERVAL = 1000; // 1 second
};

#endif