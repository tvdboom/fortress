use crate::constants::FOW_SIZE;
use crate::game::enemy::components::Enemy;
use crate::game::resources::Player;
use bevy::prelude::{Transform, Vec2 as BVec2, Vec3};
use bevy_egui::egui::*;

/// Whether an enemy is behind the fog of war
pub fn is_visible(fow_t: &Transform, enemy_t: &Transform, enemy: &Enemy) -> bool {
    fow_t.translation.y - FOW_SIZE.y * 0.45 >= enemy_t.translation.y - enemy.dim.y * 0.5
}

/// AABB collision detection
pub fn collision(pos1: &Vec3, size1: &BVec2, pos2: &Vec3, size2: &BVec2) -> bool {
    let p1_min = pos1 - Vec3::new(size1.x / 3.0, size1.y / 3.0, 0.0);
    let p1_max = pos1 + Vec3::new(size1.x / 3.0, size1.y / 3.0, 0.0);

    let p2_min = pos2 - Vec3::new(size2.x / 3.0, size2.y / 3.0, 0.0);
    let p2_max = pos2 + Vec3::new(size2.x / 3.0, size2.y / 3.0, 0.0);

    p1_max.x > p2_min.x && p1_min.x < p2_max.x && p1_max.y > p2_min.y && p1_min.y < p2_max.y
}

/// Custom syntactic sugar for repetitive UI elements
pub trait CustomUi {
    fn add_button(&mut self, text: impl Into<WidgetText>) -> Response;
    fn add_image(&mut self, id: impl Into<TextureId>, size: impl Into<Vec2>) -> Response;
    fn add_night_stats(&mut self, player: &Player);
}

impl CustomUi for Ui {
    fn add_button(&mut self, text: impl Into<WidgetText>) -> Response {
        self.add_sized([120., 40.], Button::new(text))
    }

    fn add_image(&mut self, id: impl Into<TextureId>, size: impl Into<Vec2>) -> Response {
        self.add(Image::new(load::SizedTexture::new(id, size)))
    }

    fn add_night_stats(&mut self, player: &Player) {
        self.add_space(30.);

        self.horizontal(|ui| {
            ui.add_space(140.);

            ui.spacing_mut().item_spacing.x = -85.; // Reduce space between columns
            ui.columns(2, |columns| {
                Grid::new("enemy stats")
                    .num_columns(2)
                    .spacing([20.0, 4.0])
                    .striped(true)
                    .show(&mut columns[0], |ui| {
                        ui.label(RichText::new("Enemy").strong());
                        ui.label(RichText::new("Killed / Total").strong());
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

                Grid::new("resources stats")
                    .num_columns(2)
                    .spacing([20.0, 4.0])
                    .striped(true)
                    .show(&mut columns[1], |ui| {
                        ui.label(RichText::new("Resources").strong());
                        ui.label(RichText::new("Consumed").strong());
                        ui.end_row();
                        ui.label("Bullets");
                        ui.label(format!(
                            "{:.0}",
                            player.stats.get(&player.day).unwrap().resources.bullets
                        ));
                        ui.end_row();
                        ui.label("Gasoline");
                        ui.label(format!(
                            "{:.0}",
                            player.stats.get(&player.day).unwrap().resources.gasoline
                        ));
                        ui.end_row();
                        ui.label("Materials");
                        ui.label(format!(
                            "{:.0}",
                            player.stats.get(&player.day).unwrap().resources.materials
                        ));
                    });
            });
        });

        self.add_space(30.);
    }
}

/// Custom IOS style toggle for UI
pub fn toggle(on: &mut bool) -> impl Widget + '_ {
    move |ui: &mut Ui| {
        let desired_size = ui.spacing().interact_size.y * Vec2::new(2.0, 1.0);
        let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click());
        if response.clicked() {
            *on = !*on;
            response.mark_changed();
        }

        response
            .widget_info(|| WidgetInfo::selected(WidgetType::Checkbox, ui.is_enabled(), *on, ""));

        if ui.is_rect_visible(rect) {
            let how_on = ui.ctx().animate_bool_responsive(response.id, *on);
            let visuals = ui.style().interact_selectable(&response, *on);
            let rect = rect.expand(visuals.expansion);
            let radius = 0.5 * rect.height();
            ui.painter()
                .rect(rect, radius, visuals.bg_fill, visuals.bg_stroke);
            let circle_x = lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
            let center = Pos2::new(circle_x, rect.center().y);
            ui.painter()
                .circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
        }

        response
    }
}
