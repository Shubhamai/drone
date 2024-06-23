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

        if (TRANSMITTER_SERIAL.available())
        {
            String input = TRANSMITTER_SERIAL.readStringUntil('\n');
            if (input == "exit")
            {
                DEBUG_SERIAL.println("Exiting...");
                TRANSMITTER_SERIAL.println("Exiting...");
                while (true)
                    ;
            }
        }
    }
}

void TransmitterController::printValues(unsigned long elapsed, int yaw, int pitch, int roll, int throttle, int fr, int br, int bl, int fl)
{
    TRANSMITTER_SERIAL.print(elapsed);
    TRANSMITTER_SERIAL.print(",");
    TRANSMITTER_SERIAL.print(yaw);
    TRANSMITTER_SERIAL.print(",");
    TRANSMITTER_SERIAL.print(pitch);
    TRANSMITTER_SERIAL.print(",");
    TRANSMITTER_SERIAL.print(roll);
    TRANSMITTER_SERIAL.print(",");
    TRANSMITTER_SERIAL.print(throttle);
    TRANSMITTER_SERIAL.print(",");
    TRANSMITTER_SERIAL.print(fr);
    TRANSMITTER_SERIAL.print(",");
    TRANSMITTER_SERIAL.print(br);
    TRANSMITTER_SERIAL.print(",");
    TRANSMITTER_SERIAL.print(bl);
    TRANSMITTER_SERIAL.print(",");
    TRANSMITTER_SERIAL.println(fl);
}