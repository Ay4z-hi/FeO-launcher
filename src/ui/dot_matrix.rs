use egui::{Color32, FontId, Pos2, Stroke, Ui, Vec2};

pub struct DotMatrixStyle;

impl DotMatrixStyle {
    pub const CYAN: Color32 = Color32::from_rgb(0, 255, 204);
    pub const DARK_BG: Color32 = Color32::from_rgb(8, 12, 14);

    pub fn draw_background_grid(ui: &mut Ui) {
        let rect = ui.max_rect();
        let painter = ui.painter();
        let mut x = rect.min.x;
        while x < rect.max.x {
            let mut y = rect.min.y;
            while y < rect.max.y {
                painter.circle_filled(Pos2::new(x, y), 1.0, Color32::from_rgb(18, 30, 32));
                y += 18.0;
            }
            x += 18.0;
        }
    }

    pub fn draw_matrix_button(ui: &mut Ui, label: &str, size: Vec2) -> egui::Response {
        let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());
        let painter = ui.painter();
        
        // Fixed method call for egui 0.26 Response
        let border_color = if response.is_pointer_button_down_on() {
            Color32::WHITE
        } else {
            Self::CYAN
        };

        painter.rect_filled(rect, 0.0, Self::DARK_BG);
        painter.rect_stroke(rect, 0.0, Stroke::new(2.0_f32, border_color));
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            label,
            FontId::monospace(16.0),
            Self::CYAN,
        );
        response
    }
}
