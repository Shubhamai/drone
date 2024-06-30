#ifndef SIMPLIFIED_PID_CONTROLLER_H
#define SIMPLIFIED_PID_CONTROLLER_H

#include <Arduino.h>
#include "filter.h"

class SimplifiedPIDController
{
private:
    // PID constants for roll and pitch
    float Kp_roll, Ki_roll, Kd_roll;
    float Kp_pitch, Ki_pitch, Kd_pitch;

    // Error variables for roll and pitch
    float error_roll, prev_error_roll, integral_roll;
    float error_pitch, prev_error_pitch, integral_pitch;

    // Desired angles (set by RC input)
    float desired_roll, desired_pitch;

    // Output limits
    const float MAX_OUTPUT = 350; // Adjust as needed

    // Anti-windup parameters
    const float MAX_INTEGRAL = 100;           // Adjust based on your system
    const float INTEGRAL_RESET_THRESHOLD = 5; // Degrees, adjust as needed

    // Helper function to apply PID with enhanced anti-windup
    float applyPID(float error, float &integral, float &prev_error,
                   float Kp, float Ki, float Kd, float dt)
    {
        // Proportional term
        float P = Kp * error;

        // Integral term with anti-windup
        integral += error * dt;
        integral = constrain(integral, -MAX_INTEGRAL, MAX_INTEGRAL);

        // Reset integral when error changes sign or is small
        if (error * prev_error < 0 || abs(error) < INTEGRAL_RESET_THRESHOLD)
        {
            integral = 0;
        }

        float I = Ki * integral;

        // Derivative term
        float derivative = (error - prev_error) / dt;
        float D = Kd * derivative;

        prev_error = error;

        // Calculate total output
        float output = P + I + D;

        // Apply output limits
        output = constrain(output, -MAX_OUTPUT, MAX_OUTPUT);

        return output;
    }

public:
    SimplifiedPIDController(float Kp_r, float Ki_r, float Kd_r, float Kp_p, float Ki_p, float Kd_p)
        : Kp_roll(Kp_r), Ki_roll(Ki_r), Kd_roll(Kd_r),
          Kp_pitch(Kp_p), Ki_pitch(Ki_p), Kd_pitch(Kd_p),
          error_roll(0), prev_error_roll(0), integral_roll(0),
          error_pitch(0), prev_error_pitch(0), integral_pitch(0),
          desired_roll(0), desired_pitch(0) {}

    void updateDesiredAngle(int roll, int pitch)
    {
        // Map RC input to desired angle with deadband
        int deadband = 1; // Adjust as needed
        desired_roll = (abs(roll - 1500) > deadband) ? map(roll, 1000, 2000, -20, 20) : 0;
        desired_pitch = (abs(pitch - 1500) > deadband) ? map(pitch, 1000, 2000, -20, 20) : 0;
    }

    void computePID(const FilterData &filterData, float dt, int &roll_output, int &pitch_output)
    {
        // Compute errors
        error_roll = desired_roll - filterData.roll;
        error_pitch = desired_pitch - filterData.pitch;

        // Apply PID for roll and pitch
        roll_output = applyPID(error_roll, integral_roll, prev_error_roll, Kp_roll, Ki_roll, Kd_roll, dt);
        pitch_output = applyPID(error_pitch, integral_pitch, prev_error_pitch, Kp_pitch, Ki_pitch, Kd_pitch, dt);

        // Limit outputs
        roll_output = constrain(roll_output, -MAX_OUTPUT, MAX_OUTPUT);
        pitch_output = constrain(pitch_output, -MAX_OUTPUT, MAX_OUTPUT);
    }

    void getMotorMixing(int throttle, int roll_output, int pitch_output,
                        int &front_right, int &back_right, int &back_left, int &front_left)
    {
        // X configuration mixing for roll and pitch
        front_right = throttle - roll_output + pitch_output;
        back_right = throttle - roll_output - pitch_output;
        back_left = throttle + roll_output - pitch_output;
        front_left = throttle + roll_output + pitch_output;

        // Ensure motor values are within valid range
        front_right = constrain(front_right, 1000, 2000);
        back_right = constrain(back_right, 1000, 2000);
        back_left = constrain(back_left, 1000, 2000);
        front_left = constrain(front_left, 1000, 2000);
    }

    // Method to adjust PID constants at runtime
    void adjustPIDConstants(float Kp_r, float Ki_r, float Kd_r, float Kp_p, float Ki_p, float Kd_p)
    {
        Kp_roll = Kp_r;
        Ki_roll = Ki_r;
        Kd_roll = Kd_r;
        Kp_pitch = Kp_p;
        Ki_pitch = Ki_p;
        Kd_pitch = Kd_p;
    }

    void getPIDConstants(float &Kp_r, float &Ki_r, float &Kd_r, float &Kp_p, float &Ki_p, float &Kd_p)
    {
        Kp_r = Kp_roll;
        Ki_r = Ki_roll;
        Kd_r = Kd_roll;
        Kp_p = Kp_pitch;
        Ki_p = Ki_pitch;
        Kd_p = Kd_pitch;
    }
};

#endif // SIMPLIFIED_PID_CONTROLLER_H