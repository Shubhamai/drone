#ifndef IMU_MANAGER_H
#define IMU_MANAGER_H

#include <Adafruit_LSM6DSOX.h>
#include <Adafruit_LIS3MDL.h>
#include "ICM_20948.h"

struct IMUData {
    sensors_event_t accel, gyro, mag, temp;
    // float orientation[3]; // [yaw, pitch, roll]
};

class IMUManager {
private:
    Adafruit_LSM6DSOX lsm6ds;
    Adafruit_LIS3MDL lis3mdl;
    ICM_20948_I2C myICM;

public:
    IMUManager();
    void initIMU();
    IMUData readIMU();
};

#endif
