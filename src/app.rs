use crate::{employee::Employee };
use eframe::{egui};
use std::error::Error;

pub struct EmployeeInventoryApp {
    pub employee: Employee,
}

impl EmployeeInventoryApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let employee = Employee::new("John Doe", "Manager", "IT", 1234, "2022-01-01");

        EmployeeInventoryApp { employee }
    }

    pub fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(format!("Name: {}", self.employee.name));
            ui.label(format!("Position: {}", self.employee.position));
            ui.label(format!("Department: {}", self.employee.department));
            ui.label(format!("Employee ID: {}", self.employee.employee_id));
            ui.label(format!("Hire Date: {}", self.employee.hire_date));

            ui.separator();

            egui::Grid::new("inventory_grid")
                .num_columns(6)
                .show(ui, |ui| {
                    ui.label("Name");
                    ui.label("Inventory Number");
                    ui.label("Transfer Date");
                    ui.label("Condition");
                    ui.label("Value");
                    ui.label("Responsible Person");

                    // for (_, item) in self.employee.inventory.iter() {
                    //     ui.label(&item.name);
                    //     ui.label(&item.inventory_number);
                    //     ui.label(&item.transfer_date);
                    //     ui.label(&item.condition);
                    //     ui.label(format!("{}", item.value));
                    //     ui.label(&item.responsible_person);
                    // }
                });

            // if ui.button("Save to Database").clicked() {
            //     if let Err(e) = self.database.save_employee(&self.employee) {
            //         eprintln!("Error saving employee: {}", e);
            //     }
            // }
            //
            // if ui.button("Load from Database").clicked() {
            //     if let Ok(employee) = self.database.load_employee(self.employee.employee_id) {
            //         self.employee = employee;
            //     } else {
            //         eprintln!("Error loading employee from database");
            //     }
            // }
        });
    }
}