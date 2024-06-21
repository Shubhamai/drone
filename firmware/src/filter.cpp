#include "filter.h"
#include <Adafruit_Sensor.h>  // Ensure sensor constants are available

FilterManager::FilterManager() : timestamp(0) {}

void FilterManager::initFilter() {
    filter.begin(updateRateHz);
}

FilterData FilterManager::processData(IMUData data) {
    FilterData filteredData;
    float roll, pitch, heading;
    float gx, gy, gz;

    while ((millis() - timestamp) < (1000 / updateRateHz)) {
        // Wait until the update rate interval has passed
    }
    timestamp = millis();

    // Convert gyroscope from radians/s to degrees/s
    gx = data.gyro.gyro.x * SENSORS_RADS_TO_DPS;
    gy = data.gyro.gyro.y * SENSORS_RADS_TO_DPS;
    gz = data.gyro.gyro.z * SENSORS_RADS_TO_DPS;

    // Update the sensor fusion filter
    filter.update(gx, gy, gz,
                  data.accel.acceleration.x, data.accel.acceleration.y, data.accel.acceleration.z,
                  data.mag.magnetic.x, data.mag.magnetic.y, data.mag.magnetic.z);

    roll = filter.getRoll();
    pitch = filter.getPitch();
    heading = filter.getYaw();

    // Get quaternion values
    float qw, qx, qy, qz;
    filter.getQuaternion(&qw, &qx, &qy, &qz);

    // Note: The IMU is mounted sideways, so roll and pitch are swapped
    filteredData.roll = pitch;
    filteredData.pitch = roll;
    filteredData.yaw = heading;

    filteredData.quaternion[0] = qw;
    filteredData.quaternion[1] = qx;
    filteredData.quaternion[2] = qy;
    filteredData.quaternion[3] = qz;

    return filteredData;
}