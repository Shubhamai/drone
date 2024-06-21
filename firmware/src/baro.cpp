#include "baro.h"
#include "consts.h"

Barometer::Barometer() : sensor(&Wire, DFRobot_BMP3XX::eSDOVDD) {}

void Barometer::begin() {
    int rslt;
    while(ERR_OK != (rslt = sensor.begin())) {
        if(ERR_DATA_BUS == rslt) {
            DEBUG_SERIAL.println("Data bus error!!!");
        } else if(ERR_IC_VERSION == rslt) {
            DEBUG_SERIAL.println("Chip versions do not match!!!");
        }
        delay(3000);
    }

    while(!sensor.setSamplingMode(sensor.eNormalPrecision2)) {
        DEBUG_SERIAL.println("Set sampling mode fail, retrying....");
        delay(3000);
    }

    // #ifdef CALIBRATE_ABSOLUTE_DIFFERENCE
    if(sensor.calibratedAbsoluteDifference(BAROMETER_CALIBRATION_ALTITUDE)) {
        DEBUG_SERIAL.println("Absolute difference base value set successfully!");
    }
    // #endif

    float sampingPeriodus = sensor.getSamplingPeriodUS();

    float sampingFrequencyHz = 1000000 / sampingPeriodus;
}

BaroData Barometer::readBaroData() {
    BaroData baroData;

    baroData.temperature = sensor.readTempC();
    baroData.pressure = sensor.readPressPa();
    baroData.altitude = sensor.readAltitudeM();

    return baroData;
}
