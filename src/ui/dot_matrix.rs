use egui::{Color32, Pos2, Response, Sense, Stroke, Ui, Vec2};

pub struct DotMatrixStyle;

impl DotMatrixStyle {
    pub const CYAN: Color32 = Color32::from_rgb(0, 255, 204);
    pub const DARK_BG: Color32 = Color32::from_rgb(8, 12, 14);
    pub const GRID_DOT: Color32 = Color32::from_rgb(18, 30, 32);

    pub fn draw_background_grid(ui: &mut Ui) {
        let rect = ui.max_rect();
        let painter = ui.painter();
        let spacing = 18.0;

        let mut x = rect.min.x;
        while x < rect.max.x {
            let mut y = rect.min.y;
            while y < rect.max.y {
                painter.circle_filled(Pos2::new(x, y), 1.0, Self::GRID_DOT);
                y += spacing;
            }
            x += spacing;
        }
    }

    pub fn draw_matrix_button(ui: &mut Ui, label: &str, size: Vec2) -> Response {
        let (rect, response) = ui.allocate_exact_size(size, Sense::click());
        let painter = ui.painter();

        let border_color = if response.is_pointer_button_down() {
            Color32::WHITE
        } else if response.hovered() {
            Self::CYAN
        } else {
            Color32::from_rgb(0, 160, 130)
        };

        painter.rect_filled(rect, 0.0, Self::DARK_BG);
        painter.rect_stroke(rect, 0.0, Stroke::new(2.0, border_color));

        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            label,
            egui::FontId::monospace(16.0),
            Self::CYAN,
        );

        response
    }
}
