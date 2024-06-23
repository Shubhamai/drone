#ifndef TRANSMITTER_CONTROLLER_H
#define TRANSMITTER_CONTROLLER_H

#include <Arduino.h>

class TransmitterController {
private:
    unsigned long lastPrintTime;
    const unsigned long printInterval = 1; // Print every 100ms

public:
    TransmitterController();
    
    void update(int yaw, int pitch, int roll, int throttle, int fr, int br, int bl, int fl);
    void printValues(unsigned long elapsed, int yaw, int pitch, int roll, int throttle, int fr, int br, int bl, int fl);
};

#endif