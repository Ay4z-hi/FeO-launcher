use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ControlNode {
    pub name: String,
    pub keycode: i32,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub is_round: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PojavControlLayout {
    pub version: i32,
    pub control_nodes: Vec<ControlNode>,
}

impl PojavControlLayout {
    pub fn default_feo_layout() -> Self {
        Self {
            version: 1,
            control_nodes: vec![
                ControlNode { name: "PRI".to_string(), keycode: -1, x: 0.85, y: 0.60, width: 70.0, height: 70.0, is_round: true },
                ControlNode { name: "SEC".to_string(), keycode: -2, x: 0.73, y: 0.75, width: 70.0, height: 70.0, is_round: true },
                ControlNode { name: "INV".to_string(), keycode: 69, x: 0.88, y: 0.35, width: 50.0, height: 50.0, is_round: false },
                ControlNode { name: "CHAT".to_string(), keycode: 84, x: 0.50, y: 0.05, width: 60.0, height: 40.0, is_round: false },
                ControlNode { name: "FLY".to_string(), keycode: 32, x: 0.73, y: 0.55, width: 50.0, height: 50.0, is_round: true },
            ],
        }
    }
                             }
