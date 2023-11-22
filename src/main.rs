#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod gui;

use egui::*;
use gui::tree::Tree;
use rfd::FileDialog;
use serde_json::Result;

fn main() -> Result<()> {
    let options = eframe::NativeOptions {
        ..Default::default()
    };

    let mut tree = Tree::default();
    let mut textb: String = "".to_string();

    let _ = eframe::run_simple_native("jsonv - JSON Viewer", options, move |ctx, _frame| {
        ctx.set_visuals(egui::style::Visuals::dark());
        egui::CentralPanel::default().show(ctx, |ui| {
            let option = Tree::from_json(textb.as_str());
            let mut show_tree = false;
            if option.is_some() {
                tree = option.unwrap();
                show_tree = true;
            }

            ui.menu_button("File", |ui| {
                if ui.button("Open...").clicked() {
                    ui.close_menu();
                    let files = FileDialog::new()
                        .add_filter("json", &["json", "js"])
                        .set_directory("/")
                        .pick_file();

                    match files {
                        Some(file) => {
                            textb = std::fs::read_to_string(file).unwrap();
                        }
                        None => {}
                    }
                }
                ui.separator();
                if ui.button("Exit").clicked() {
                    std::process::exit(0);
                }
            });
            ui.separator();
            ui.with_layout(
                egui::Layout::left_to_right(Align::Min).with_cross_justify(true),
                |ui| {
                    let h = ui.available_height();
                    let w = ui.available_width() - 20.0;

                    egui::ScrollArea::vertical()
                        .id_source("first")
                        .show(ui, |ui| {
                            ui.set_min_height(h);
                            ui.set_min_width(w / 2.0);
                            if show_tree == false {
                                ui.label(RichText::new("Unable to parse JSON").color(Color32::RED));
                            } else {
                                tree.ui(ui);
                            }
                        });
                    ui.add(egui::Separator::default().vertical());
                    egui::ScrollArea::vertical()
                        .id_source("second")
                        .show(ui, |ui| {
                            ui.set_min_height(h);
                            ui.set_min_width(w / 2.0);
                            TextEdit::multiline(&mut textb)
                                .min_size(Vec2 { x: w / 2.0, y: h })
                                .ui(ui);
                        });
                },
            );
        });
    });

    Ok(())
}
