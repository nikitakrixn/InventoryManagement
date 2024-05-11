pub struct Employee {
    pub name: String,
    pub position: String,
    pub department: String,
    pub employee_id: u32,
    pub hire_date: String,
}

impl Employee {
    pub fn new(name: &str, position: &str, department: &str, employee_id: u32, hire_date: &str) -> Self {
        Employee {
            name: name.to_string(),
            position: position.to_string(),
            department: department.to_string(),
            employee_id,
            hire_date: hire_date.to_string(),
        }
    }
}