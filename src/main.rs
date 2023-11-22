#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use egui::*;
use serde_json::{Result, Value};

fn main() -> Result<()> {
    let options = eframe::NativeOptions {
        ..Default::default()
    };

    let mut tree = Tree::default();
    let mut textb: String = "".to_string();

    let _ = eframe::run_simple_native("jsonv", options, move |ctx, _frame| {
        ctx.set_visuals(egui::style::Visuals::dark());
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.set_min_size(ui.available_size());
            tree = Tree::from_json(textb.as_str());

            ui.heading("JSON Viewer");
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
                            tree.ui(ui);
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

#[derive(Clone, Copy, PartialEq)]
enum Action {
    Keep,
    Delete,
}

#[derive(Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
struct Tree {
    children: Vec<Tree>,
    name: String,
    value: String,
}

impl Tree {
    pub fn from_json(json: &str) -> Self {
        let v = serde_json::from_str(json);

        match v {
            Ok(vv) => Self::from_value(&vv, "root"),
            Err(_) => Self {
                children: vec![],
                name: "".to_string(),
                value: "".to_string(),
            },
        }
    }

    pub fn from_value(v: &Value, name: &str) -> Self {
        match v {
            Value::Object(ref m) => {
                let mut children = vec![];
                for (key, value) in m {
                    children.push(Self::from_value(value, key));
                }
                Self {
                    children,
                    name: name.to_string(),
                    value: "".to_string(),
                }
            }

            Value::String(ref s) => Self {
                children: vec![],
                name: name.to_string(),
                value: s.to_string(),
            },

            Value::Number(ref n) => Self {
                children: vec![],
                name: name.to_string(),
                value: n.to_string(),
            },

            Value::Array(ref a) => {
                let mut children = vec![];
                for (i, item) in a.iter().enumerate() {
                    children.push(Self::from_value(item, &format!("[{}]", i)));
                }
                Self {
                    children,
                    name: name.to_string(),
                    value: "".to_string(),
                }
            }

            Value::Bool(ref b) => Self {
                children: vec![],
                name: name.to_string(),
                value: b.to_string(),
            },
            _ => Self {
                children: vec![],
                name: name.to_string(),
                value: "".to_string(),
            },
        }
    }

    pub fn ui(&mut self, ui: &mut Ui) -> Action {
        self.ui_impl(ui, 0)
    }
}

impl Tree {
    fn ui_impl(&mut self, ui: &mut Ui, depth: usize) -> Action {
        let n = self.name.clone();
        let v = self.value.clone();
        let t = if depth == 0 {
            "root".to_string()
        } else {
            format!("{} : {}", n, v)
        };

        if self.children.is_empty() {
            ui.label(t);
            Action::Keep
        } else {
            CollapsingHeader::new(t)
                .default_open(depth < 1)
                .show(ui, |ui| self.children_ui(ui, depth))
                .body_returned
                .unwrap_or(Action::Keep)
        }
    }

    fn children_ui(&mut self, ui: &mut Ui, depth: usize) -> Action {
        self.children = std::mem::take(self)
            .children
            .into_iter()
            .enumerate()
            .filter_map(|(i, mut tree)| {
                if tree.ui_impl(ui, depth + 1) == Action::Keep {
                    Some(tree)
                } else {
                    None
                }
            })
            .collect();

        Action::Keep
    }
}
