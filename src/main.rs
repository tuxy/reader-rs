#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console

use eframe::egui;
use env_logger;

fn main() -> Result<(), eframe::Error> {

    // Adding a config system to be able to switch settings such as always on top, etc...
    env_logger::init(); 
    let options = eframe::NativeOptions { // Setting native window options. TODO: Options to change this?
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1060.0, 640.0]),
        ..Default::default()
    };
    eframe::run_native(
        "rust-totp", 
        options, // Option presets?
        Box::new(|_cc| {
            Ok(Box::<Secrets>::default())
        }),
    )

}
struct Secrets { // Variable init for vars "thrown" into the impl for App
    text: String,
}

impl Default for Secrets { // The vars actually "thrown" into the impl for App by default (On startup)
    fn default() -> Self {
        Self {
            text: String::from("World!"),
        }
    }
}

impl eframe::App for Secrets {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::TopBottomPanel::top("search").show(ctx, |ui| {
                ui.label("Search Bar");
            });
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.label("Contents");
            });
        });
    }
}