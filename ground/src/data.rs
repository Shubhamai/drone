use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ReceivedData {
    pub aruco_ids: Vec<u32>,
    pub serial_data: SerialData,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
#[repr(C, packed)]
pub struct SerialData {
    pub elapsed_time: f32,
    pub acc_x: f32,
    pub acc_y: f32,
    pub acc_z: f32,
    pub gyro_x: f32,
    pub gyro_y: f32,
    pub gyro_z: f32,
    pub mag_x: f32,
    pub mag_y: f32,
    pub mag_z: f32,
    pub altitude: f32,
    pub temp: f32,
    pub yaw: i32,
    pub pitch: i32,
    pub roll: i32,
    pub rc_throttle: i32,
    pub rc_yaw: i32,
    pub rc_pitch: i32,
    pub rc_roll: i32,
    pub front_right: i32,
    pub back_right: i32,
    pub back_left: i32,
    pub front_left: i32,

    pub kp_r: f32,
    pub ki_r: f32,
    pub kd_r: f32,

    // pub kp_p: f32,
    // pub ki_p: f32,
    // pub kd_p: f32,
}
