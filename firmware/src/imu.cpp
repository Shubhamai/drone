#include "imu.h"
#include "consts.h"

IMUManager::IMUManager() {}  // Adjust as necessary for your I2C address

void IMUManager::initIMU() {
    if (!lsm6ds.begin_I2C()) {
        DEBUG_SERIAL.println("Failed to find LSM6DS chip");
    }
    if (!lis3mdl.begin_I2C()) {
        DEBUG_SERIAL.println("Failed to find LIS3MDL chip");
    }
    if (myICM.begin() != ICM_20948_Stat_Ok) {
        DEBUG_SERIAL.println(F("ICM_20948 initialization failed!"));
    }

    // Configure additional settings as needed here
    lsm6ds.setAccelRange(LSM6DS_ACCEL_RANGE_2_G);
    lsm6ds.setAccelDataRate(LSM6DS_RATE_416_HZ);
    lsm6ds.setGyroRange(LSM6DS_GYRO_RANGE_250_DPS);
    lsm6ds.setGyroDataRate(LSM6DS_RATE_416_HZ);

    lis3mdl.setDataRate(LIS3MDL_DATARATE_300_HZ);
    lis3mdl.setRange(LIS3MDL_RANGE_4_GAUSS);
    lis3mdl.setPerformanceMode(LIS3MDL_ULTRAHIGHMODE);
    lis3mdl.setOperationMode(LIS3MDL_CONTINUOUSMODE);
    lis3mdl.setIntThreshold(500);
    lis3mdl.configInterrupt(false, false, true, // enable z axis
                            true,               // polarity
                            false,              // don't latch
                            true);              // enabled!


}

IMUData IMUManager::readIMU() {
    IMUData data;
    lsm6ds.getEvent(&data.accel, &data.gyro, &data.temp);
    lis3mdl.getEvent(&data.mag);

    // Optionally, implement quaternion reading from ICM_20948 and convert to Euler angles

    return data;
}
