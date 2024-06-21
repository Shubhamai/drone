#ifndef BAROMETER_H
#define BAROMETER_H

#include <DFRobot_BMP3XX.h>
#include <Wire.h>

struct BaroData; // Forward declaration of BaroData struct

class Barometer {
private:
    DFRobot_BMP390L_I2C sensor;

public:
    Barometer();
    void begin();
    BaroData readBaroData();
};

struct BaroData { // Define BaroData struct after Barometer class definition
    float temperature;
    float pressure;
    float altitude;
};

#endif