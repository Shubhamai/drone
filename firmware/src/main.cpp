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

void doReboot()
{
    SCB_AIRCR = 0x05FA0004;
}

MotorController motors(FRONT_RIGHT_MOTOR_PIN, BACK_RIGHT_MOTOR_PIN,
                       BACK_LEFT_MOTOR_PIN, FRONT_LEFT_MOTOR_PIN);
StateController state;

SimplifiedPIDController pidController(1.2, 1.0, 4.0, 1.2, 1.0, 4.0);

TransmitterController transmitter;

// ReceiverController receiver(RC_CHANNEL_1_THROTTLE_PIN, RC_CHANNEL_4_ROLL_PIN,
//                             RC_CHANNEL_3_PITCH_PIN, RC_CHANNEL_2_YAW_PIN);
FakeReceiverController receiver; // Use FakeReceiverController instead of ReceiverController

Barometer barometer;
IMUManager imuManager;
FilterManager filterManager;

void parsePIDString(String input, float *values, int numValues)
{
    int startIndex = input.indexOf('>') + 1;
    int endIndex = 0;

    for (int i = 0; i < numValues; i++)
    {
        endIndex = input.indexOf(',', startIndex);
        if (endIndex == -1)
        {
            endIndex = input.length();
        }

        String valueStr = input.substring(startIndex, endIndex);
        values[i] = valueStr.toFloat();

        startIndex = endIndex + 1;
        if (startIndex >= input.length())
        {
            break;
        }
    }
}

void setup(void)
{
    motors.initialize();
    motors.disableMotors();
    motors.setupTimer();

    DEBUG_SERIAL.begin(SERIAL_BAUD_RATE);
    TRANSMITTER_SERIAL.setTimeout(2);
    TRANSMITTER_SERIAL.begin(2000000);

    Wire.begin();
    Wire.setClock(I2C_CLOCK_SPEED);

    // Wait for serial monitor to open
    // while (!DEBUG_SERIAL)
    // {
    //     digitalWrite(LED_BUILTIN, HIGH);
    //     delay(100);
    //     digitalWrite(LED_BUILTIN, LOW);
    //     delay(100);
    // }
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
    delay(5000);
    TRANSMITTER_SERIAL.println("Waiting for command to arm...");
    DEBUG_SERIAL.println("Waiting for command to arm...");
    while (true)
    {
        TransmitterData data;
        transmitter.transmitData(data);
        String receivedData = transmitter.receiveData();
        if (receivedData.length() > 0)
        {
            if (receivedData == "command->arm")
            {
                DEBUG_SERIAL.println("Armed...");
                TRANSMITTER_SERIAL.println("Armed...");
                break;
            }
        }
        delay(400);
    }

    receiver.setThrottle(1000);
    receiver.setRoll(1500);
    receiver.setPitch(1500);
    receiver.setYaw(1500);
    receiver.setEnabled(true);
}

elapsedMillis elapsedTime;
unsigned long lastEnableMotorCheck = 0;
void loop()
{
    uint32_t start_loop = millis();

    state.update();
    // bool isReceivedEnabled = receiver.update();

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

        // format, e.g. pid->3.0,0.1,0.0,3.0,0.1,0.0
        if (receivedData.startsWith("pid->"))
        {
            float values[6];
            parsePIDString(receivedData, values, 6);

            pidController.adjustPIDConstants(values[0], values[1], values[2], values[3], values[4], values[5]);
        }

        // make sure ping is received every 300ms

        if (receivedData == "command->enable_motors")
        {
            // DEBUG_SERIAL.println("Enabling motors...");
            // TRANSMITTER_SERIAL.println("Enabling motors...");
            // motors.enableMotors();
            lastEnableMotorCheck = millis();
        }
        if (receivedData == "command->reboot")
        {
            doReboot();
        }
    }

    // if (!isReceivedEnabled)
    if (millis() - lastEnableMotorCheck > 200)
    {
        DEBUG_SERIAL.println("Last enable motor check: disabling motors...");
        motors.disableMotors();
    }
    else
    {
        motors.enableMotors(receiver);
    }

    int RCthrottle = receiver.getThrottle(); // in range [1000, 2000]
    int RCpitch = receiver.getPitch();       // in range [1000, 2000]
    int RCyaw = receiver.getYaw();           // in range [1000, 2000]
    int RCroll = receiver.getRoll();         // in range [1000, 2000]

    IMUData imu_data = imuManager.readIMU();
    BaroData baro_data = barometer.readBaroData();
    FilterData filterData = filterManager.processData(imu_data);

    // motors.setAllThrust(RCthrottle, RCthrottle, RCthrottle, RCthrottle);
    // Update PID controller for roll only
    pidController.updateDesiredAngle(RCroll, RCpitch);

    int roll_output;
    int pitch_output;
    float dt = 0.0142; // Assume 100Hz loop frequency, adjust if different
    pidController.computePID(filterData, dt, roll_output, pitch_output);

    float Kp_r, Ki_r, Kd_r, Kp_p, Ki_p, Kd_p;
    pidController.getPIDConstants(Kp_r, Ki_r, Kd_r, Kp_p, Ki_p, Kd_p);

    int fr, br, bl, fl;
    pidController.getMotorMixing(RCthrottle, roll_output, pitch_output, fr, br, bl, fl);

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
    data.kp_p = Kp_p;
    data.ki_p = Ki_p;
    data.kd_p = Kd_p;

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
