mod controls;
mod downloader;
mod ui;

use controls::json_parser::PojavControlLayout;
use controls::overlay::TouchOverlayEngine;
use ui::dot_matrix::DotMatrixStyle;
use ui::hud::DownloadHud;

#[cfg(target_os = "android")]
use android_activity::AndroidApp;

use egui::{Color32, FontId, Pos2, Rect, Sense, Stroke, Vec2};

#[derive(PartialEq)]
enum AppState {
    Menu,
    ActiveGameLoop,
}

pub struct FeOLauncherApp {
    state: AppState,
    overlay_engine: TouchOverlayEngine,
    hud: DownloadHud,
    is_loading: bool,
}

impl FeOLauncherApp {
    pub fn new() -> Self {
        Self {
            state: AppState::Menu,
            overlay_engine: TouchOverlayEngine::new(PojavControlLayout::default_feo_layout()),
            hud: DownloadHud::new(),
            is_loading: false,
        }
    }
}

impl eframe::App for FeOLauncherApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            DotMatrixStyle::draw_background_grid(ui);

            match self.state {
                AppState::Menu => {
                    ui.vertical_centered(|ui| {
                        ui.add_space(40.0);
                        ui.heading(
                            egui::RichText::new("FEO LAUNCHER :: HORIZON 26.3")
                                .font(FontId::monospace(22.0))
                                .color(DotMatrixStyle::CYAN),
                        );
                        ui.add_space(50.0);

                        if DotMatrixStyle::draw_matrix_button(ui, "[ PLAY NATIVE GAME ]", Vec2::new(240.0, 52.0)).clicked() {
                            self.is_loading = true;
                        }

                        if self.is_loading {
                            ui.add_space(30.0);
                            self.hud.render(ui);
                            if self.hud.progress < 1.0 {
                                self.hud.progress += 0.01_f32;
                                ctx.request_repaint();
                            } else {
                                self.state = AppState::ActiveGameLoop;
                            }
                        }
                    });
                }
                AppState::ActiveGameLoop => {
                    ui.painter().rect_filled(ctx.screen_rect(), 0.0, Color32::from_rgb(5, 8, 10));
                    self.overlay_engine.render(ctx);

                    let exit_rect = Rect::from_min_size(Pos2::new(20.0, 20.0), Vec2::new(80.0, 35.0));
                    if ui.allocate_rect(exit_rect, Sense::click()).clicked() {
                        self.state = AppState::Menu;
                        self.is_loading = false;
                        self.hud.progress = 0.0;
                    }
                    ui.painter().rect_filled(exit_rect, 0.0, DotMatrixStyle::DARK_BG);
                    ui.painter().rect_stroke(exit_rect, 0.0, Stroke::new(1.0_f32, DotMatrixStyle::CYAN));
                    ui.painter().text(exit_rect.center(), egui::Align2::CENTER_CENTER, "EXIT", FontId::monospace(12.0), DotMatrixStyle::CYAN);
                }
            }
        });
    }
}

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(_app: AndroidApp) {
    android_logger::init_once(android_logger::Config::default().with_max_level(log::LevelFilter::Info));
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "FeO Launcher",
        options,
        Box::new(|_cc| Box::new(FeOLauncherApp::new())),
    ).unwrap();
}
