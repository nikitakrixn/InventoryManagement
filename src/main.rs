#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;
use crate::app::InventoryApp;
use crate::repository::Repository;

mod models;
mod db;
mod repository;
mod app;

#[tokio::main]
async fn main() -> eframe::Result<()> {

    dotenv::dotenv().ok();

    let repo = Arc::new(Repository::new().await.unwrap());

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([620.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Управление выдачей имущества",
        native_options,
        Box::new(|cc| Box::new(InventoryApp::new(cc, repo)))
    )
}