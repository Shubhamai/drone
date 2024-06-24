#ifndef SIMPLIFIED_PID_CONTROLLER_H
#define SIMPLIFIED_PID_CONTROLLER_H

#include <Arduino.h>
#include "receiver.h"
#include "filter.h"

class SimplifiedPIDController {
private:
    // PID constants
    float Kp_roll, Ki_roll, Kd_roll;
    float Kp_pitch, Ki_pitch, Kd_pitch;

    // Error variables
    float error_roll, prev_error_roll, integral_roll;
    float error_pitch, prev_error_pitch, integral_pitch;

    // Desired angles (set by RC input)
    float desired_roll, desired_pitch;

    // Output limits
    const float MAX_OUTPUT = 250; // Adjust as needed

    // Helper function to apply PID
    float applyPID(float error, float &integral, float &prev_error, 
                   float Kp, float Ki, float Kd, float dt) {
        integral += error * dt;
        float derivative = (error - prev_error) / dt;
        prev_error = error;

        // Anti-windup: Limit integral term
        integral = constrain(integral, -MAX_OUTPUT / Ki, MAX_OUTPUT / Ki);

        return Kp * error + Ki * integral + Kd * derivative;
    }

public:
    SimplifiedPIDController(float Kp_r, float Ki_r, float Kd_r,
                            float Kp_p, float Ki_p, float Kd_p)
        : Kp_roll(Kp_r), Ki_roll(Ki_r), Kd_roll(Kd_r),
          Kp_pitch(Kp_p), Ki_pitch(Ki_p), Kd_pitch(Kd_p),
          error_roll(0), prev_error_roll(0), integral_roll(0),
          error_pitch(0), prev_error_pitch(0), integral_pitch(0),
          desired_roll(0), desired_pitch(0) {}

    void updateDesiredAngles(const ReceiverController &receiver) {
        // Map RC inputs to desired angles with deadband
        int deadband = 50; // Adjust as needed
        int roll = receiver.getRoll();
        int pitch = receiver.getPitch();

        desired_roll = (abs(roll - 1500) > deadband) ? map(roll, 1000, 2000, -20, 20) : 0;
        desired_pitch = (abs(pitch - 1500) > deadband) ? map(pitch, 1000, 2000, -20, 20) : 0;
    }

    void computePID(const FilterData &filterData, float dt, int &roll_output, int &pitch_output) {
        // Compute errors
        error_roll = desired_roll - filterData.roll;
        error_pitch = desired_pitch - filterData.pitch;

        // Apply PID for each axis
        roll_output = applyPID(error_roll, integral_roll, prev_error_roll, Kp_roll, Ki_roll, Kd_roll, dt);
        pitch_output = applyPID(error_pitch, integral_pitch, prev_error_pitch, Kp_pitch, Ki_pitch, Kd_pitch, dt);

        // Limit outputs
        roll_output = constrain(roll_output, -MAX_OUTPUT, MAX_OUTPUT);
        pitch_output = constrain(pitch_output, -MAX_OUTPUT, MAX_OUTPUT);
    }

    void getMotorMixing(int throttle, int roll_output, int pitch_output, 
                        int &front_right, int &back_right, int &back_left, int &front_left) {
        front_right = throttle + roll_output - pitch_output;
        back_right = throttle + roll_output + pitch_output;
        back_left = throttle - roll_output + pitch_output;
        front_left = throttle - roll_output - pitch_output;

        // Ensure motor values are within valid range
        front_right = constrain(front_right, 1000, 2000);
        back_right = constrain(back_right, 1000, 2000);
        back_left = constrain(back_left, 1000, 2000);
        front_left = constrain(front_left, 1000, 2000);
    }

    // Method to adjust PID constants at runtime
    void adjustPIDConstants(float Kp_r, float Ki_r, float Kd_r,
                            float Kp_p, float Ki_p, float Kd_p) {
        Kp_roll = Kp_r; Ki_roll = Ki_r; Kd_roll = Kd_r;
        Kp_pitch = Kp_p; Ki_pitch = Ki_p; Kd_pitch = Kd_p;
    }
};

#endif // SIMPLIFIED_PID_CONTROLLER_H