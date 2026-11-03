use crate::ui::dot_matrix::DotMatrixStyle;
use egui::{Align2, Color32, FontId, Pos2, Rect, Stroke, Ui, Vec2};

pub struct DownloadHud {
    pub progress: f32,
    pub current_file: String,
}

impl DownloadHud {
    pub fn new() -> Self {
        Self {
            progress: 0.0,
            current_file: String::from("INITIALIZING HORIZON ENGINE 26.3..."),
        }
    }

    pub fn render(&self, ui: &mut Ui) {
        let rect = ui.max_rect();
        let painter = ui.painter();

        let hud_rect = Rect::from_center_size(rect.center(), Vec2::new(420.0, 180.0));
        painter.rect_filled(hud_rect, 0.0, DotMatrixStyle::DARK_BG);
        painter.rect_stroke(hud_rect, 0.0, Stroke::new(1.5, DotMatrixStyle::CYAN));

        painter.text(
            Pos2::new(hud_rect.min.x + 15.0, hud_rect.min.y + 15.0),
            Align2::LEFT_TOP,
            ":: FEO NATIVE BOOTSTRAP ::",
            FontId::monospace(11.0),
            DotMatrixStyle::CYAN,
        );

        let bar_width = 390.0;
        let bar_rect = Rect::from_min_size(
            Pos2::new(hud_rect.min.x + 15.0, hud_rect.min.y + 45.0),
            Vec2::new(bar_width, 16.0),
        );
        painter.rect_stroke(bar_rect, 0.0, Stroke::new(1.0, DotMatrixStyle::CYAN));
        
        let filled = bar_width * self.progress.clamp(0.0, 1.0);
        painter.rect_filled(Rect::from_min_size(bar_rect.min, Vec2::new(filled, 16.0)), 0.0, DotMatrixStyle::CYAN);

        painter.text(
            Pos2::new(hud_rect.min.x + 15.0, hud_rect.min.y + 75.0),
            Align2::LEFT_TOP,
            format!("STATUS: {}", self.current_file),
            FontId::monospace(10.0),
            Color32::LIGHT_GRAY,
        );
    }
}
