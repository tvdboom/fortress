use crate::constants::{FOW_SIZE, MAP_SIZE};
use crate::game::enemy::components::Enemy;
use crate::game::resources::{Expedition, Player, Technology};
use crate::utils::NameFromEnum;
use bevy::prelude::{Transform, Vec2 as BVec2, Vec3};
use bevy::utils::HashMap;
use bevy_egui::egui::*;
use std::hash::Hash;

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
    fn add_text(&mut self, text: impl Into<WidgetText>, width: f32) -> Response;
    fn add_scroll<R>(
        &mut self,
        id: impl Hash,
        indent: f32,
        add_contents: impl FnOnce(&mut Ui) -> R,
    );
    fn add_night_stats(&mut self, player: &Player, day: u32);
    fn add_technology(
        &mut self,
        technology: &Technology,
        player: &Player,
        tick_texture: TextureId,
        tech_texture: TextureId,
    ) -> Response;

    fn add_expedition(
        &mut self,
        expedition: &Expedition,
        textures: &HashMap<&str, TextureId>,
    ) -> Response;
}

impl CustomUi for Ui {
    fn add_button(&mut self, text: impl Into<WidgetText>) -> Response {
        self.add_sized([120., 40.], Button::new(text))
    }

    fn add_image(&mut self, id: impl Into<TextureId>, size: impl Into<Vec2>) -> Response {
        self.add(Image::new(load::SizedTexture::new(id, size)))
    }

    fn add_text(&mut self, text: impl Into<WidgetText>, width: f32) -> Response {
        self.add_sized([width, self.available_height()], Label::new(text))
    }

    fn add_scroll<R>(
        &mut self,
        id: impl Hash,
        indent: f32,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) {
        ScrollArea::vertical()
            .id_salt(id)
            .max_width(MAP_SIZE.x * 0.5)
            .show(self, |ui| {
                ui.horizontal(|ui| {
                    ui.add_space(indent);
                    ui.with_layout(Layout::top_down(Align::LEFT), add_contents);
                })
            });
    }

    fn add_night_stats(&mut self, player: &Player, day: u32) {
        let stats = player.stats.get(&day).unwrap();

        self.add_space(30.);

        self.horizontal(|ui| {
            ui.add_space(20.);

            ui.columns(3, |cols| {
                Grid::new("enemy stats")
                    .num_columns(2)
                    .spacing([20.0, 4.0])
                    .striped(true)
                    .show(&mut cols[0], |ui| {
                        ui.strong("Enemy");
                        ui.strong("Killed / Total");
                        ui.end_row();

                        stats.enemies.iter().for_each(|(k, v)| {
                            ui.label(*k);
                            ui.label(format!("{} / {}", v.killed, v.spawned));
                            ui.end_row();
                        });
                    });

                Grid::new("resources stats")
                    .num_columns(2)
                    .spacing([20.0, 4.0])
                    .striped(true)
                    .show(&mut cols[1], |ui| {
                        ui.strong("Resources");
                        ui.strong("Consumed");
                        ui.end_row();
                        ui.label("Bullets");
                        ui.label(format!("{:.0}", stats.resources.bullets));
                        ui.end_row();
                        ui.label("Gasoline");
                        ui.label(format!("{:.0}", stats.resources.gasoline));
                        ui.end_row();
                        ui.label("Materials");
                        ui.label(format!("{:.0}", stats.resources.materials));
                    });

                Grid::new("population stats")
                    .num_columns(2)
                    .spacing([20.0, 4.0])
                    .striped(true)
                    .show(&mut cols[2], |ui| {
                        ui.strong("Population");
                        ui.strong("Died / Total");
                        ui.end_row();
                        ui.label("Soldiers");
                        ui.label(format!(
                            "{} / {}",
                            stats.population.soldier,
                            player.population.soldier + stats.population.soldier
                        ));
                        ui.end_row();
                        ui.label("Armorers");
                        ui.label(format!(
                            "{} / {}",
                            stats.population.armorer,
                            player.population.armorer + stats.population.armorer
                        ));
                        ui.end_row();
                        ui.label("Refiners");
                        ui.label(format!(
                            "{} / {}",
                            stats.population.refiner,
                            player.population.refiner + stats.population.refiner
                        ));
                        ui.end_row();
                        ui.label("Constructor");
                        ui.label(format!(
                            "{} / {}",
                            stats.population.constructor,
                            player.population.constructor + stats.population.constructor
                        ));
                        ui.end_row();
                        ui.label("Scientists");
                        ui.label(format!(
                            "{} / {}",
                            stats.population.scientist,
                            player.population.scientist + stats.population.scientist
                        ));
                    });
            });
        });

        self.add_space(30.);
    }

    fn add_technology(
        &mut self,
        technology: &Technology,
        player: &Player,
        tech_texture: TextureId,
        tick_texture: TextureId,
    ) -> Response {
        self.scope_builder(
            UiBuilder::new()
                .id_salt(technology.name.name())
                .sense(Sense::click()),
            |ui| {
                let response = ui.response();
                let visuals = ui.style().interact(&response);

                Frame::canvas(ui.style())
                    .fill(visuals.bg_fill.gamma_multiply(0.3))
                    .stroke(visuals.bg_stroke)
                    .inner_margin(ui.spacing().menu_margin)
                    .show(ui, |ui| {
                        ui.set_width(140.);

                        if player.has_tech(technology.name) {
                            // Draw tick when technology is researched
                            let pos = ui.max_rect().min;
                            ui.painter().image(
                                tick_texture,
                                Rect::from_min_size(
                                    pos2(pos.x - 20., pos.y - 20.),
                                    [50., 50.].into(),
                                ),
                                Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
                                Color32::WHITE,
                            );
                        }

                        ui.vertical_centered(|ui| {
                            ui.add_space(5.);
                            Label::new(RichText::new(technology.name.name()).strong())
                                .selectable(false)
                                .ui(ui);

                            ui.add_space(25.);

                            ui.with_layout(Layout::right_to_left(Align::RIGHT), |ui| {
                                ui.horizontal(|ui| {
                                    ui.add_space(5.);
                                    ui.add_image(tech_texture, [15., 15.]);
                                    Label::new(technology.price.to_string())
                                        .selectable(false)
                                        .ui(ui);
                                });
                            });
                        });
                    });
            },
        )
        .response
        .on_hover_text(technology.description)
        .on_hover_cursor(CursorIcon::PointingHand)
    }

    fn add_expedition(
        &mut self,
        expedition: &Expedition,
        textures: &HashMap<&str, TextureId>,
    ) -> Response {
        self.scope_builder(
            UiBuilder::new()
                .id_salt(expedition.name.name())
                .sense(Sense::click()),
            |ui| {
                let response = ui.response();
                let visuals = ui.style().interact(&response);

                Frame::canvas(ui.style())
                    .fill(visuals.bg_fill.gamma_multiply(0.3))
                    .stroke(visuals.bg_stroke)
                    .inner_margin(ui.spacing().menu_margin)
                    .show(ui, |ui| {
                        ui.set_width(180.);

                        ui.vertical_centered(|ui| {
                            ui.add_space(5.);
                            Label::new(RichText::new(expedition.name.name()).strong())
                                .selectable(false)
                                .ui(ui);

                            ui.add_space(100.);

                            ui.horizontal(|ui| {
                                ui.add_space(50.);
                                ui.add_image(textures["population"], [25., 25.]);
                                Label::new(expedition.population.to_string())
                                    .selectable(false)
                                    .ui(ui);
                            });

                            ui.add_space(10.);

                            ui.horizontal(|ui| {
                                ui.add_space(50.);
                                ui.add_image(textures["gasoline"], [25., 25.]);
                                Label::new(expedition.price.gasoline.to_string())
                                    .selectable(false)
                                    .ui(ui);
                            });

                            ui.add_space(10.);

                            ui.horizontal(|ui| {
                                ui.add_space(50.);
                                ui.add_image(textures["materials"], [25., 25.]);
                                Label::new(expedition.price.materials.to_string())
                                    .selectable(false)
                                    .ui(ui);
                            });

                            ui.add_space(10.);

                            ui.horizontal(|ui| {
                                ui.add_space(50.);
                                ui.add_image(textures["clock"], [25., 25.]);
                                Label::new(expedition.duration)
                                    .selectable(false)
                                    .ui(ui);
                            });

                            ui.add_space(100.);
                        });
                    });
            },
        )
        .response
        .on_hover_cursor(CursorIcon::PointingHand)
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
