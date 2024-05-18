use std::sync::Arc;
use eframe::Frame;
use egui::Context;
use crate::repository::Repository;

pub struct InventoryApp {
    repo: Arc<Repository>,
}

impl InventoryApp {
    pub fn new(cc: &eframe::CreationContext<'_>, repo: Arc<Repository>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());

        InventoryApp { repo }
    }
}

impl eframe::App for InventoryApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.heading("Учёт выдачи имущества");
            });
        });
    }
}