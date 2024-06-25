#include <Arduino.h>
#include <Wire.h>

#include "consts.h"
#include "motor.h"
#include "state.h"
#include "receiver.h"
#include "fake_receiver.h"
#include "transmitter.h"
#include "baro.h"
#include "imu.h"
#include "filter.h"
#include "pid.h"

MotorController motors(FRONT_RIGHT_MOTOR_PIN, BACK_RIGHT_MOTOR_PIN,
                       BACK_LEFT_MOTOR_PIN, FRONT_LEFT_MOTOR_PIN);
StateController state;

SimplifiedPIDController pidController(3.0, 0.1, 0.0); // Roll (P, I, D)

TransmitterController transmitter;

// ReceiverController receiver(RC_CHANNEL_1_THROTTLE_PIN, RC_CHANNEL_4_ROLL_PIN,
//                             RC_CHANNEL_3_PITCH_PIN, RC_CHANNEL_2_YAW_PIN);
FakeReceiverController receiver; // Use FakeReceiverController instead of ReceiverController

Barometer barometer;
IMUManager imuManager;
FilterManager filterManager;

void setup(void)
{
    DEBUG_SERIAL.begin(SERIAL_BAUD_RATE);
    TRANSMITTER_SERIAL.setTimeout(2);
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
    // while (!receiver.isThrottleZero())
    // {
    //     DEBUG_SERIAL.println("Waiting for throttle to be zero...");
    //     delay(100);
    // }
    // DEBUG_SERIAL.println("Throttle is zero. Ready to fly!");

    receiver.setThrottle(1000);
    receiver.setRoll(1500);
    receiver.setPitch(1500);
    receiver.setYaw(1500);
    receiver.setEnabled(true);
}

elapsedMillis elapsedTime;
void loop()
{
    uint32_t start_loop = millis();

    state.update();
    bool isReceivedEnabled = receiver.update();

    String receivedData = transmitter.receiveData();
    if (receivedData.length() > 0)
    {
        receiver.parseRCValues(receivedData);

        if (receivedData == "command->abort")
        {
            DEBUG_SERIAL.println("Aborting...");
            TRANSMITTER_SERIAL.println("Aborting...");
            while (true)
                ;
        }

        // format, e.g. pid->3.0,0.1,0.0
        if (receivedData.startsWith("pid->"))
        {
            String pidValues = receivedData.substring(5);
            int commaIndex = pidValues.indexOf(',');
            float Kp_r = pidValues.substring(0, commaIndex).toFloat();
            pidValues = pidValues.substring(commaIndex + 1);
            commaIndex = pidValues.indexOf(',');
            float Ki_r = pidValues.substring(0, commaIndex).toFloat();
            float Kd_r = pidValues.substring(commaIndex + 1).toFloat();

            pidController.adjustPIDConstants(Kp_r, Ki_r, Kd_r);
        }
    }

    // if (!isReceivedEnabled)
    // {
    //     DEBUG_SERIAL.println("Receiver not enabled");
    //     motors.disableMotors();
    //     return;
    // }
    // else
    // {
    //     motors.enableMotors(receiver);
    // }

    int RCthrottle = receiver.getThrottle(); // in range [1000, 2000]
    int RCpitch = receiver.getPitch();       // in range [1000, 2000]
    int RCyaw = receiver.getYaw();           // in range [1000, 2000]
    int RCroll = receiver.getRoll();         // in range [1000, 2000]

    IMUData imu_data = imuManager.readIMU();
    BaroData baro_data = barometer.readBaroData();
    FilterData filterData = filterManager.processData(imu_data);

    // motors.setAllThrust(RCthrottle, RCthrottle, RCthrottle, RCthrottle);
    // Update PID controller for roll only
    pidController.updateDesiredAngle(RCroll);

    int roll_output;
    float dt = 0.01; // Assume 100Hz loop frequency, adjust if different
    pidController.computePID(filterData, dt, roll_output);

    float Kp_r, Ki_r, Kd_r;
    pidController.getPIDConstants(Kp_r, Ki_r, Kd_r);

    int fr, br, bl, fl;
    pidController.getMotorMixing(RCthrottle, roll_output, fr, br, bl, fl);

    motors.setAllThrust(fr, br, bl, fl);

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
    data.kp_r = Kp_r;
    data.ki_r = Ki_r;
    data.kd_r = Kd_r;
    // data.kp_p = Kp_p;
    // data.ki_p = Ki_p;
    // data.kd_p = Kd_p;

    transmitter.transmitData(data);

    //////////////////////////////////////////////////

    const uint32_t end_loop = millis();
    DEBUG_SERIAL.print("Loop time: ");
    DEBUG_SERIAL.println(end_loop - start_loop);
    // if (end_loop - start_loop > 10)
    // {
    //     DEBUG_SERIAL.print("Loop time: ");
    //     DEBUG_SERIAL.println(end_loop - start_loop);
    // }
}