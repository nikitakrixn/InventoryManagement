use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct Employee {
    pub id: i32,
    pub name: String,
    pub position: String,
    pub department: String,
}

#[derive(Debug, FromRow)]
pub struct InventoryItem {
    pub id: i32,
    pub serial_number: String,
    pub responsible_person_id: i32,
    pub material_account: String,
    pub name: String,
    pub count: f64,
    pub price: f64,
    pub item_type: String,
}

#[derive(Debug, FromRow)]
pub struct IssueRecord {
    pub id: i32,
    pub employee_id: i32,
    pub inventory_item_id: i32,
    pub issue_date: chrono::NaiveDateTime,
    pub return_date: Option<chrono::NaiveDateTime>,
    pub status: String,
    pub comments: Option<String>,
}