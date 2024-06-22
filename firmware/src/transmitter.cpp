#include "transmitter.h"
#include "consts.h"

TransmitterController::TransmitterController() : lastPrintTime(0) {}

void TransmitterController::update(int yaw, int pitch, int roll, int throttle, int fr, int br, int bl, int fl)
{
    unsigned long elapsed = millis();

    if (elapsed - lastPrintTime >= printInterval)
    {
        printValues(elapsed, yaw, pitch, roll, throttle, fr, br, bl, fl);
        lastPrintTime = elapsed;
    }
}

void TransmitterController::printValues(unsigned long elapsed, int yaw, int pitch, int roll, int throttle, int fr, int br, int bl, int fl)
{
    TRANSMITTER_SERIAL.print("Elapsed: ");
    TRANSMITTER_SERIAL.print(elapsed);
    TRANSMITTER_SERIAL.print(" ms, Yaw: ");
    TRANSMITTER_SERIAL.print(yaw);
    TRANSMITTER_SERIAL.print(", Pitch: ");
    TRANSMITTER_SERIAL.print(pitch);
    TRANSMITTER_SERIAL.print(", Roll: ");
    TRANSMITTER_SERIAL.print(roll);
    TRANSMITTER_SERIAL.print(", Throttle: ");
    TRANSMITTER_SERIAL.print(throttle);
    TRANSMITTER_SERIAL.print(", FR: ");
    TRANSMITTER_SERIAL.print(fr);
    TRANSMITTER_SERIAL.print(", BR: ");
    TRANSMITTER_SERIAL.print(br);
    TRANSMITTER_SERIAL.print(", BL: ");
    TRANSMITTER_SERIAL.print(bl);
    TRANSMITTER_SERIAL.print(", FL: ");
    TRANSMITTER_SERIAL.println(fl);
}