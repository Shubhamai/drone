#ifndef CONSTS_H
#define CONSTS_H

// PINS
const int FRONT_RIGHT_LED_PIN = 11; // 15
const int BACK_RIGHT_LED_PIN = 22;
const int BACK_LEFT_LED_PIN = 28;
const int FRONT_LEFT_LED_PIN = 11; //8 

const int FRONT_RIGHT_MOTOR_PIN = 14;
const int BACK_RIGHT_MOTOR_PIN = 23;
const int BACK_LEFT_MOTOR_PIN = 9;
const int FRONT_LEFT_MOTOR_PIN = 15; // 7 - TODO: temp change, pin not working

const int BUZZER_PIN = 29;
const int RED_LED_PIN = 33;
const int GREEN_LED_PIN = 36;
const int BLUE_LED_PIN = 37;

const int RC_CHANNEL_1_THROTTLE_PIN = 12;
const int RC_CHANNEL_2_YAW_PIN = 10;
const int RC_CHANNEL_3_PITCH_PIN = 11;
const int RC_CHANNEL_4_ROLL_PIN = 24;

const int MIN_THROTTLE = 1000;
const int MAX_THROTTLE = 1700;

/// Sensors //////////////////////////

const float BAROMETER_CALIBRATION_ALTITUDE = 540.0; // meters

const int FILTER_UPDATE_HERTZ = 142; // Hz

///////////////////////////////////////

const int NEO_PIXEL_LED_BRIGHTNESS = 64; // 25%

///////////////////////////////////////

const int SERIAL_BAUD_RATE = 230400;
const int I2C_CLOCK_SPEED = 1000000;
#define DEBUG_SERIAL Serial
#define TRANSMITTER_SERIAL Serial5

#endif