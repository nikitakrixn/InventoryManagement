use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AssignedItem {
    pub id: u32,
    pub item_id: u32,
    pub employee_id: u32,
    pub count: u32,
    pub assigned_date: String,
    pub return_date: Option<String>
}