use crate::controls::json_parser::PojavControlLayout;
use egui::{Color32, Pos2, Rect, Sense, Stroke, Vec2};

pub struct TouchOverlayEngine {
    pub layout: PojavControlLayout,
    pub sensitivity: f32,
}

impl TouchOverlayEngine {
    pub fn new(layout: PojavControlLayout) -> Self {
        Self {
            layout,
            sensitivity: 1.2,
        }
    }

    pub fn render(&mut self, ctx: &egui::Context) {
        let screen_rect = ctx.screen_rect();
        
        // Inspect pointers using correct egui API field
        let _pointer_count = ctx.input(|i| i.pointer.press_origin().is_some() as usize);

        egui::Area::new("touch_overlay")
            .interactable(true)
            .fixed_pos(Pos2::ZERO)
            .show(ctx, |ui| {
                let joy_center = Pos2::new(screen_rect.width() * 0.15, screen_rect.height() * 0.75);
                ui.painter().circle_stroke(joy_center, 65.0_f32, Stroke::new(1.5_f32, Color32::from_rgb(0, 255, 204)));
                ui.painter().circle_filled(joy_center, 10.0, Color32::from_rgb(0, 255, 204));

                for node in &self.layout.control_nodes {
                    let pos = Pos2::new(node.x * screen_rect.width(), node.y * screen_rect.height());
                    let rect = Rect::from_center_size(pos, Vec2::new(node.width, node.height));
                    let response = ui.allocate_rect(rect, Sense::click_and_drag());
                    
                    // Fixed method call for egui 0.26 Response
                    let stroke_color = if response.is_pointer_button_down_on() {
                        Color32::WHITE
                    } else {
                        Color32::from_rgb(0, 255, 204)
                    };

                    if node.is_round {
                        ui.painter().circle_stroke(pos, node.width / 2.0, Stroke::new(2.0_f32, stroke_color));
                    } else {
                        ui.painter().rect_stroke(rect, 0.0, Stroke::new(2.0_f32, stroke_color));
                    }
                    ui.painter().text(
                        pos,
                        egui::Align2::CENTER_CENTER,
                        &node.name,
                        egui::FontId::monospace(13.0),
                        stroke_color,
                    );
                }
            });
    }
}
