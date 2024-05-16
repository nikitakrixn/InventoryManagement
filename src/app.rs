use std::fs;
use crate::assigned_item::AssignedItem;
use crate::employee::Employee;
use crate::inventory_item::InventoryItem;

pub struct InventoryApp {
    pub inventory: Vec<InventoryItem>,
    pub employees: Vec<Employee>,
    pub assigned_items: Vec<AssignedItem>,
    pub selected_employee: Option<u32>,
}

impl Default for InventoryApp {
    fn default() -> Self {
        let inventory = load_inventory_from_file("inventory.json");
        let employees = load_employees_from_file("employees.json");
        let assigned_items = load_assigned_items_from_file("assigned_items.json");
        InventoryApp {
            inventory,
            employees,
            assigned_items,
            selected_employee: None,
        }
    }
}

impl InventoryApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        Self::default()
    }
    fn update_selected_employee(&mut self, employee_id: Option<u32>) {
        self.selected_employee = employee_id;
    }
}

impl eframe::App for InventoryApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut selected_employee = self.selected_employee;
        egui::SidePanel::left("Employee").show(ctx, |ui| {
            ui.heading("Сотрудники:");
            ui.add_space(16.);
            ui.horizontal(|ui| {
                egui::ComboBox::from_label("")
                    .selected_text(
                        selected_employee
                            .and_then(|id| self.employees.iter().find(|e| e.id == id))
                            .map(|e| e.fullname.clone())
                            .unwrap_or_else(|| "Выберите сотрудника".to_string()),
                    )
                    .show_ui(ui, |ui| {
                        for employee in &self.employees {
                            ui.selectable_value(&mut selected_employee, Some(employee.id), employee.fullname.clone());
                        }
                    });
                if let Some(employee_id) = selected_employee {
                    self.update_selected_employee(Some(employee_id));
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Inventory Management");
            ui.separator();
        });
    }
}

fn load_inventory_from_file(filename: &str) -> Vec<InventoryItem> {
    let contents = fs::read_to_string(filename).expect("Failed to read inventory file");
    serde_json::from_str(&contents).expect("Failed to parse inventory JSON")
}

fn load_employees_from_file(filename: &str) -> Vec<Employee> {
    let contents = fs::read_to_string(filename).expect("Failed to read employees file");
    serde_json::from_str(&contents).expect("Failed to parse employees JSON")
}

fn load_assigned_items_from_file(filename: &str) -> Vec<AssignedItem> {
    let contents = fs::read_to_string(filename).expect("Failed to read assigned items file");
    serde_json::from_str(&contents).expect("Failed to parse assigned items JSON")
}