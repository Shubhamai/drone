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
ReceiverController receiver(RC_CHANNEL_1_THROTTLE_PIN, RC_CHANNEL_4_ROLL_PIN,
                            RC_CHANNEL_3_PITCH_PIN, RC_CHANNEL_2_YAW_PIN);

TransmitterController transmitter;

Barometer barometer;
IMUManager imuManager;
FilterManager filterManager;

void setup(void)
{
    DEBUG_SERIAL.begin(SERIAL_BAUD_RATE);
    TRANSMITTER_SERIAL.begin(1000000);

    Wire.begin();
    Wire.setClock(I2C_CLOCK_SPEED);

    motors.initialize();
    motors.disableMotors();
    motors.setupTimer();

    // Wait for serial monitor to open
    while (!DEBUG_SERIAL)
    {
        digitalWrite(LED_BUILTIN, HIGH);
        delay(100);
        digitalWrite(LED_BUILTIN, LOW);
        delay(100);
    }
    DEBUG_SERIAL.println("Starting Drone...");

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

elapsedMillis elapsedTime;
void loop()
{
    uint32_t start_loop = millis();

    state.update();
    bool isReceivedEnabled = receiver.update();
    if (!isReceivedEnabled)
    {
        DEBUG_SERIAL.println("Receiver not enabled");
        motors.disableMotors();
        return;
    }
    else
    {
        motors.enableMotors(receiver);
    }

    int RCthrottle = receiver.getThrottle();
    int RCpitch = receiver.getPitch();
    int RCyaw = receiver.getYaw();
    int RCroll = receiver.getRoll();

    IMUData imu_data = imuManager.readIMU();
    BaroData baro_data = barometer.readBaroData();
    FilterData filterData = filterManager.processData(imu_data);

    motors.setAllThrust(RCthrottle, RCthrottle, RCthrottle, RCthrottle);
    int fr, br, bl, fl;
    motors.getAllThrust(fr, br, bl, fl);

    TransmitterData data;
    data.elapsedTime = elapsedTime;
    data.accX = imu_data.accel.acceleration.x;
    data.accY = imu_data.accel.acceleration.y;
    data.accZ = imu_data.accel.acceleration.z;
    data.gyroX = imu_data.gyro.gyro.x * SENSORS_RADS_TO_DPS;
    data.gyroY = imu_data.gyro.gyro.y * SENSORS_RADS_TO_DPS;
    data.gyroZ = imu_data.gyro.gyro.z * SENSORS_RADS_TO_DPS;
    data.magX = imu_data.mag.magnetic.x;
    data.magY = imu_data.mag.magnetic.y;
    data.magZ = imu_data.mag.magnetic.z;
    data.altitude = baro_data.altitude;
    data.temp = baro_data.temperature;
    data.yaw = filterData.yaw;
    data.pitch = filterData.pitch;
    data.roll = filterData.roll;
    data.rcThrottle = RCthrottle;
    data.rcYaw = RCyaw;
    data.rcPitch = RCpitch;
    data.rcRoll = RCroll;
    data.frontRight = fr;
    data.backRight = br;
    data.backLeft = bl;
    data.frontLeft = fl;
    transmitter.update(data);

    //////////////////////////////////////////////////

    const uint32_t end_loop = millis();
    DEBUG_SERIAL.print("Loop time: ");
    DEBUG_SERIAL.println(end_loop - start_loop);
}