use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct InventoryItem {
    pub id: u32,
    pub serial_number: Option<SerialNumber>,
    pub responsible_person_id: Option<u32>,
    pub material_account: Option<MaterialAccount>,
    pub name: String,
    pub inventory_count: f64,
    pub cost: f64,
    pub item_type: String,
    pub assigned_to: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum SerialNumber {
    Number(u64),
    String(String),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum MaterialAccount {
    Number(u32),
    String(String),
}

impl InventoryItem {
    pub fn assign_to_employee(&mut self, employee_id: u32) {
        self.assigned_to = Some(employee_id);
    }

    pub fn return_from_employee(&mut self) {
        self.assigned_to = None;
    }
}