use crate::game::resources::Player;
use bevy::math::{Vec2, Vec3};
use bevy_egui::egui;
use bevy_egui::egui::{epaint, Response, RichText, TextureId, Ui, WidgetText};
use std::time::Duration;

/// Scale a Duration by a factor
pub fn scale_duration(duration: Duration, scale: f32) -> Duration {
    let sec = (duration.as_secs() as f32 + duration.subsec_nanos() as f32 * 1e-9) * scale;
    Duration::new(sec.trunc() as u64, (sec.fract() * 1e9) as u32)
}

/// Custom syntax sugar for repetitive UI elements
pub trait CustomUi {
    fn add_button(&mut self, text: impl Into<WidgetText>) -> Response;
    fn add_image(&mut self, id: impl Into<TextureId>, size: impl Into<epaint::Vec2>) -> Response;
    fn add_night_stats(&mut self, player: &Player);
}

impl CustomUi for Ui {
    fn add_button(&mut self, text: impl Into<WidgetText>) -> Response {
        self.add_sized([120., 40.], egui::Button::new(text))
    }

    fn add_image(&mut self, id: impl Into<TextureId>, size: impl Into<epaint::Vec2>) -> Response {
        self.add(egui::widgets::Image::new(egui::load::SizedTexture::new(
            id, size,
        )))
    }

    fn add_night_stats(&mut self, player: &Player) {
        self.add_space(30.);

        self.horizontal(|ui| {
            ui.add_space(220.);
            egui::Grid::new("night stats")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.label(RichText::new("Enemy").strong());
                    ui.label(RichText::new("Killed / Spawned").strong());
                    ui.end_row();

                    player
                        .stats
                        .get(&player.day)
                        .unwrap()
                        .enemies
                        .iter()
                        .for_each(|(k, v)| {
                            ui.label(*k);
                            ui.label(format!("{} / {}", v.killed, v.spawned));
                            ui.end_row();
                        });
                });
        });

        self.add_space(30.);
    }
}

/// Custom IOS style toggle for UI
pub fn toggle(on: &mut bool) -> impl egui::Widget + '_ {
    move |ui: &mut Ui| {
        let desired_size = ui.spacing().interact_size.y * egui::vec2(2.0, 1.0);
        let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
        if response.clicked() {
            *on = !*on;
            response.mark_changed();
        }

        response.widget_info(|| {
            egui::WidgetInfo::selected(egui::WidgetType::Checkbox, ui.is_enabled(), *on, "")
        });

        if ui.is_rect_visible(rect) {
            let how_on = ui.ctx().animate_bool_responsive(response.id, *on);
            let visuals = ui.style().interact_selectable(&response, *on);
            let rect = rect.expand(visuals.expansion);
            let radius = 0.5 * rect.height();
            ui.painter()
                .rect(rect, radius, visuals.bg_fill, visuals.bg_stroke);
            let circle_x = egui::lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
            let center = egui::pos2(circle_x, rect.center().y);
            ui.painter()
                .circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
        }

        response
    }
}

/// AABB collision detection
pub fn collision(pos1: &Vec3, size1: &Vec2, pos2: &Vec3, size2: &Vec2) -> bool {
    let p1_min = pos1 - Vec3::new(size1.x / 3.0, size1.y / 3.0, 0.0);
    let p1_max = pos1 + Vec3::new(size1.x / 3.0, size1.y / 3.0, 0.0);

    let p2_min = pos2 - Vec3::new(size2.x / 3.0, size2.y / 3.0, 0.0);
    let p2_max = pos2 + Vec3::new(size2.x / 3.0, size2.y / 3.0, 0.0);

    p1_max.x > p2_min.x && p1_min.x < p2_max.x && p1_max.y > p2_min.y && p1_min.y < p2_max.y
}
