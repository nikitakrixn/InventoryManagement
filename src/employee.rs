use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Employee {
    pub id: u32,
    pub fullname: String,
    pub position: String,
    pub department: String,
}