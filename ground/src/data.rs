use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ReceivedData {
    pub aruco_ids: Vec<u32>,
    pub serial_data: String,
}