use egui::{Pos2, Rect, Stroke, Ui, Vec2};
use crate::ui::dot_matrix::DotMatrixStyle;

pub struct DownloadHud {
    pub progress: f32,
}

impl DownloadHud {
    pub fn new() -> Self {
        Self { progress: 0.0 }
    }

    pub fn render(&self, ui: &mut Ui) {
        let rect = ui.max_rect();
        let painter = ui.painter();
        let hud_rect = Rect::from_center_size(rect.center(), Vec2::new(420.0, 180.0));
        
        painter.rect_filled(hud_rect, 0.0, DotMatrixStyle::DARK_BG);
        painter.rect_stroke(hud_rect, 0.0, Stroke::new(1.5_f32, DotMatrixStyle::CYAN));

        let bar_rect = Rect::from_min_size(
            Pos2::new(hud_rect.min.x + 15.0, hud_rect.min.y + 45.0),
            Vec2::new(390.0, 16.0),
        );
        painter.rect_stroke(bar_rect, 0.0, Stroke::new(1.0_f32, DotMatrixStyle::CYAN));

        let filled = 390.0 * self.progress.clamp(0.0, 1.0);
        painter.rect_filled(
            Rect::from_min_size(bar_rect.min, Vec2::new(filled, 16.0)),
            0.0,
            DotMatrixStyle::CYAN,
        );
    }
}
