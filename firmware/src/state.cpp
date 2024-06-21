#include "state.h"

StateController::StateController() : ledState(false), lastLEDToggle(0), lastBuzzerToggle(0)

                                     ,
                                     frontRightLed(2, FRONT_RIGHT_LED_PIN, NEO_GRB + NEO_KHZ800),
                                     backRightLed(2, BACK_RIGHT_LED_PIN, NEO_GRB + NEO_KHZ800),
                                     backLeftLed(2, BACK_LEFT_LED_PIN, NEO_GRB + NEO_KHZ800),
                                     frontLeftLed(2, FRONT_LEFT_LED_PIN, NEO_GRB + NEO_KHZ800)

{
}

void StateController::initialize()
{
    initializeLEDs();
    initializeBuzzer();
}

void StateController::initializeLEDs()
{
    frontRightLed.begin();
    frontRightLed.setBrightness(NEO_PIXEL_LED_BRIGHTNESS);
    frontRightLed.clear();
    frontRightLed.show();

    backRightLed.begin();
    backRightLed.setBrightness(NEO_PIXEL_LED_BRIGHTNESS);
    backRightLed.clear();
    backRightLed.show();

    backLeftLed.begin();
    backLeftLed.setBrightness(NEO_PIXEL_LED_BRIGHTNESS);
    backLeftLed.clear();
    backLeftLed.show();

    frontLeftLed.begin();
    frontLeftLed.setBrightness(NEO_PIXEL_LED_BRIGHTNESS);
    frontLeftLed.clear();
    frontLeftLed.show();

    frontRightLed.setPixelColor(0, 255, 0, 0);
    frontRightLed.setPixelColor(1, 255, 255, 255);
    frontRightLed.show();

    backRightLed.setPixelColor(0, 255, 0, 0);
    backRightLed.setPixelColor(1, 255, 255, 255);
    backRightLed.show();

    backLeftLed.setPixelColor(0, 255, 0, 0);
    backLeftLed.setPixelColor(1, 255, 255, 255);
    backLeftLed.show();

    frontLeftLed.setPixelColor(0, 255, 0, 0);
    frontLeftLed.setPixelColor(1, 255, 255, 255);
    frontLeftLed.show();

    pinMode(RED_LED_PIN, OUTPUT);
    pinMode(GREEN_LED_PIN, OUTPUT);
    pinMode(BLUE_LED_PIN, OUTPUT);

    digitalWrite(RED_LED_PIN, LOW);
    digitalWrite(GREEN_LED_PIN, LOW);
    digitalWrite(BLUE_LED_PIN, LOW);
}

void StateController::initializeBuzzer()
{
    pinMode(BUZZER_PIN, OUTPUT);
    digitalWrite(BUZZER_PIN, LOW);
}

void StateController::update()
{
    updateLEDs();
    updateBuzzer();
}

void StateController::updateLEDs()
{
    unsigned long currentMillis = millis();

    if (currentMillis - lastLEDToggle >= LED_TOGGLE_INTERVAL)
    {
        ledState = !ledState;
        digitalWrite(RED_LED_PIN, ledState);
        digitalWrite(GREEN_LED_PIN, !ledState);
        digitalWrite(BLUE_LED_PIN, ledState);

        lastLEDToggle = currentMillis;
    }

    switch (stateNeoPixelRGB)
    {
    case 0: // First state: Turn all LEDs off
        if (currentMillis - previousMillisNeoPixelLed >= interval1RGB)
        {
            previousMillisNeoPixelLed = currentMillis;
            setNeoPixedRGB(0, 0, 0); // Turn all LEDs off
            stateNeoPixelRGB = 1;
        }
        break;
    case 1: // Second state: Turn all LEDs to white
        if (currentMillis - previousMillisNeoPixelLed >= interval2RGB)
        {
            previousMillisNeoPixelLed = currentMillis;
            setNeoPixedRGB(255, 255, 255); // Turn all LEDs to white
            stateNeoPixelRGB = 2;
        }
        break;
    case 2: // Third state: Turn all LEDs off again
        if (currentMillis - previousMillisNeoPixelLed >= interval1RGB)
        {
            previousMillisNeoPixelLed = currentMillis;
            setNeoPixedRGB(0, 0, 0); // Turn all LEDs off again
            stateNeoPixelRGB = 3;
        }
        break;
    case 3: // Fourth state: Turn all LEDs to white again
        if (currentMillis - previousMillisNeoPixelLed >= interval2RGB)
        {
            previousMillisNeoPixelLed = currentMillis;
            setNeoPixedRGB(255, 255, 255); // Turn all LEDs to white again
            stateNeoPixelRGB = 0;
        }
        break;
    }
}

void StateController::setNeoPixedRGB(uint8_t r, uint8_t g, uint8_t b)
{
    frontRightLed.setPixelColor(1, r, g, b);
    backRightLed.setPixelColor(1, r, g, b);
    backLeftLed.setPixelColor(1, r, g, b);
    frontLeftLed.setPixelColor(1, r, g, b);

    frontRightLed.show();
    backRightLed.show();
    backLeftLed.show();
    frontLeftLed.show();
}

void StateController::updateBuzzer()
{
    unsigned long currentMillis = millis();

    if (currentMillis - lastBuzzerToggle >= BUZZER_TOGGLE_INTERVAL)
    {
        tone(BUZZER_PIN, 20.63, 50);

        lastBuzzerToggle = currentMillis;
    }
}