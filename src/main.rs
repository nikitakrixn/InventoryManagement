#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() -> eframe::Result<()> {

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 500.0])
            .with_min_inner_size([300.0, 200.0])
            .with_icon(
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon.png")[..])
                    .expect("Failed to load icon")
            ),
        ..Default::default()
    };

    eframe::run_native(
        "Система учета выдачи имущества сотрудникам",
        native_options,
        Box::new(|_cc| Box::new(inventory_control_system::InventoryApp::default())),
    )
}
