#ifndef SIMPLIFIED_PID_CONTROLLER_H
#define SIMPLIFIED_PID_CONTROLLER_H

#include <Arduino.h>
#include "filter.h"

class SimplifiedPIDController
{
private:
    // PID constants for roll only
    float Kp_roll, Ki_roll, Kd_roll;

    // Error variables for roll only
    float error_roll, prev_error_roll, integral_roll;

    // Desired angle (set by RC input)
    float desired_roll;

    // Output limits
    const float MAX_OUTPUT = 250; // Adjust as needed

    // Helper function to apply PID
    float applyPID(float error, float &integral, float &prev_error,
                   float Kp, float Ki, float Kd, float dt)
    {
        integral += error * dt;
        float derivative = (error - prev_error) / dt;
        prev_error = error;
        // Anti-windup: Limit integral term
        integral = constrain(integral, -MAX_OUTPUT / Ki, MAX_OUTPUT / Ki);
        return Kp * error + Ki * integral + Kd * derivative;
    }

public:
    SimplifiedPIDController(float Kp_r, float Ki_r, float Kd_r)
        : Kp_roll(Kp_r), Ki_roll(Ki_r), Kd_roll(Kd_r),
          error_roll(0), prev_error_roll(0), integral_roll(0),
          desired_roll(0) {}

    void updateDesiredAngle(int roll)
    {
        // Map RC input to desired angle with deadband
        int deadband = 50; // Adjust as needed
        desired_roll = (abs(roll - 1500) > deadband) ? map(roll, 1000, 2000, -20, 20) : 0;
    }

    void computePID(const FilterData &filterData, float dt, int &roll_output)
    {
        // Compute error
        error_roll = desired_roll - filterData.roll;

        // Apply PID for roll
        roll_output = applyPID(error_roll, integral_roll, prev_error_roll, Kp_roll, Ki_roll, Kd_roll, dt);

        // Limit output
        roll_output = constrain(roll_output, -MAX_OUTPUT, MAX_OUTPUT);
    }

    void getMotorMixing(int throttle, int roll_output,
                        int &front_right, int &back_right, int &back_left, int &front_left)
    {
        // X configuration mixing for roll only
        front_right = throttle + roll_output;
        back_right = throttle + roll_output;
        back_left = throttle - roll_output;
        front_left = throttle - roll_output;

        // Ensure motor values are within valid range
        front_right = constrain(front_right, 1000, 2000);
        back_right = constrain(back_right, 1000, 2000);
        back_left = constrain(back_left, 1000, 2000);
        front_left = constrain(front_left, 1000, 2000);
    }

    // Method to adjust PID constants at runtime
    void adjustPIDConstants(float Kp_r, float Ki_r, float Kd_r)
    {
        Kp_roll = Kp_r;
        Ki_roll = Ki_r;
        Kd_roll = Kd_r;
    }

    void getPIDConstants(float &Kp_r, float &Ki_r, float &Kd_r)
    {
        Kp_r = Kp_roll;
        Ki_r = Ki_roll;
        Kd_r = Kd_roll;
    }
};

#endif // SIMPLIFIED_PID_CONTROLLER_H