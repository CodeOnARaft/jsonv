use egui::{CollapsingHeader, Ui};
use serde_json::Value;

#[derive(Clone, Copy, PartialEq)]
pub enum Action {
    Keep,
}

#[derive(Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Tree {
    children: Vec<Tree>,
    name: String,
    value: String,
}

impl Tree {
    pub fn from_json(json: &str) -> Option<Tree> {
        let v = serde_json::from_str(json);

        match v {
            Ok(vv) => return Some(Self::from_value(&vv, "root")),
            Err(e) => return None,
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
    pub fn ui_impl(&mut self, ui: &mut Ui, depth: usize) -> Action {
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
