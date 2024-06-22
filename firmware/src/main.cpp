#include <Arduino.h>
#include <Wire.h>

#include "consts.h"
#include "motor.h"
#include "state.h"
#include "receiver.h"
#include "transmitter.h"
#include "baro.h"
#include "imu.h"
#include "filter.h"

MotorController motors(FRONT_RIGHT_MOTOR_PIN, BACK_RIGHT_MOTOR_PIN,
                       BACK_LEFT_MOTOR_PIN, FRONT_LEFT_MOTOR_PIN);
StateController state;
ReceiverController receiver(RC_CHANNEL_1_THROTTLE_PIN, RC_CHANNEL_1_THROTTLE_PIN, RC_CHANNEL_1_THROTTLE_PIN, RC_CHANNEL_1_THROTTLE_PIN);
TransmitterController transmitter;

Barometer barometer;
IMUManager imuManager;
FilterManager filterManager;

void setup(void)
{
    DEBUG_SERIAL.begin(SERIAL_BAUD_RATE);
    TRANSMITTER_SERIAL.begin(SERIAL_BAUD_RATE);
    
    Wire.begin();
    Wire.setClock(I2C_CLOCK_SPEED);

    // Wait for serial monitor to open
    while (!DEBUG_SERIAL)
    {
        digitalWrite(LED_BUILTIN, HIGH);
        delay(100);
        digitalWrite(LED_BUILTIN, LOW);
        delay(100);
    }
    DEBUG_SERIAL.println("Starting Drone...");

    motors.initialize();
    state.initialize();
    receiver.initialize(); // Initialize interrupts for receiving RC signals

    barometer.begin(); // Initialize the barometer
    imuManager.initIMU();
    filterManager.initFilter();

    // Wait for throttle to be zero
    while (!receiver.isThrottleZero())
    {
        DEBUG_SERIAL.println("Waiting for throttle to be zero...");
        delay(100);
    }
    DEBUG_SERIAL.println("Throttle is zero. Ready to fly!");
}

void loop()
{
    uint32_t start_loop = millis();

    state.update();
    bool isReceivedEnabled = receiver.update();
    if (!isReceivedEnabled)
    {
        // DEBUG_SERIAL.println("Receiver not enabled");
        motors.disableMotors();
        return;
    }
    else
    {
        motors.enableMotors(receiver);
    }

    int throttle = receiver.getThrottle();

    BaroData baro_data = barometer.readBaroData();
    IMUData imu_data = imuManager.readIMU();
    FilterData filterData = filterManager.processData(imu_data);

    motors.setAllThrust(throttle, throttle, throttle, throttle);
    int fr, br, bl, fl;
    motors.getAllThrust(fr, br, bl, fl);

    transmitter.update(0, 0, 0, throttle, fr, br, bl, fl);

    //////////////////////////////////////////////////

    const uint32_t end_loop = millis();
    DEBUG_SERIAL.print("Loop time: ");
    DEBUG_SERIAL.println(end_loop - start_loop);
}