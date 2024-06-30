// filter.h
#ifndef FILTER_MANAGER_H
#define FILTER_MANAGER_H

#include <Adafruit_AHRS.h>
#include "imu.h"  // Assuming IMUData is defined here
#include "consts.h"

struct FilterData {
    float roll, pitch, yaw;  // Euler angles
    float quaternion[4];     // Quaternion representation
};

class FilterManager {
private:
    Adafruit_NXPSensorFusion filter;
    uint32_t timestamp;
    const int updateRateHz = FILTER_UPDATE_HERTZ;
    const float alpha = 0.98; // Complementary filter coefficient
    float filteredRoll, filteredPitch, filteredYaw;

public:
    FilterManager();
    void initFilter();
    FilterData processData(IMUData data);
};

#endif