use super::components::*;
use crate::constants::*;
use crate::game::components::*;
use crate::game::enemy::components::Enemy;
use crate::game::resources::{GameSettings, NightStats, Player};
use crate::game::weapon::components::{Bullet, Weapon};
use crate::game::enemy::spawn::EnemySpawner;
use crate::game::{AppState, GameState};
use crate::utils::{scale_duration, toggle, CustomUi};
use bevy::color::palettes::basic::WHITE;
use bevy::prelude::*;
use bevy_egui::egui::{Align, Layout, RichText, Style, TextStyle, UiBuilder};
use bevy_egui::{egui, EguiContexts};
use catppuccin_egui;
use std::ops::Deref;

pub fn set_style(mut contexts: EguiContexts) {
    let context = contexts.ctx_mut();

    let mut style = Style::default();
    style.text_styles.get_mut(&TextStyle::Body).unwrap().size = NORMAL_FONT_SIZE;
    style.text_styles.get_mut(&TextStyle::Button).unwrap().size = NORMAL_FONT_SIZE;
    style.text_styles.get_mut(&TextStyle::Heading).unwrap().size = LARGE_FONT_SIZE;
    context.set_style(style);

    catppuccin_egui::set_theme(context, catppuccin_egui::FRAPPE);
}

pub fn draw_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite {
            image: asset_server.load("map/map.png"),
            custom_size: Some(MAP_SIZE),
            ..default()
        },
        Transform::from_xyz(
            -WEAPONS_PANEL_SIZE.x * 0.5,
            SIZE.y * 0.5 - MENU_PANEL_SIZE.y - MAP_SIZE.y * 0.5,
            0.0,
        ),
        Map,
    ));

    commands
        .spawn((
            Sprite {
                color: Color::srgba(0., 0., 0., 0.3),
                custom_size: Some(Vec2::new(SIZE.x * 0.1, SIZE.y * 0.1)),
                ..default()
            },
            Transform::from_xyz(
                -WEAPONS_PANEL_SIZE.x * 0.5,
                (RESOURCES_PANEL_SIZE.y + WALL_SIZE.y) * 0.5,
                4.9,
            ),
            Visibility::Hidden,
            PauseWrapper,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text2d::new("Paused".to_string()),
                TextColor(Color::from(WHITE)),
                TextLayout::new_with_justify(JustifyText::Center),
                Transform::from_xyz(0., 0., 5.0),
                PauseText,
            ));
        });
}

pub fn menu_panel(
    mut contexts: EguiContexts,
    mut game_settings: ResMut<GameSettings>,
    app_state: Res<State<AppState>>,
    game_state: Res<State<GameState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    egui::TopBottomPanel::top("Menu")
        .exact_height(MENU_PANEL_SIZE.y)
        .show(contexts.ctx_mut(), |ui| {
            ui.horizontal_centered(|ui| {
                egui::menu::bar(ui, |ui| {
                    egui::menu::menu_button(ui, "Game", |ui| {
                        if ui.button("New game").clicked() {
                            next_app_state.set(AppState::StartGame);
                            ui.close_menu();
                        }
                        if ui.button("Load game").clicked() {
                            todo!();
                        }
                        if ui.button("Save game").clicked() {
                            todo!();
                        }
                        if ui.button("Quit").clicked() {
                            std::process::exit(0);
                        }
                    });
                    egui::menu::menu_button(ui, "Tools", |ui| {
                        if ui
                            .add_enabled(
                                *app_state.get() == AppState::Night,
                                egui::Button::new("Toggle pause"),
                            )
                            .clicked()
                        {
                            match game_state.get() {
                                GameState::Running => next_game_state.set(GameState::Paused),
                                GameState::Paused => next_game_state.set(GameState::Running),
                            }
                        }
                    });
                    egui::menu::menu_button(ui, "View", |ui| {
                        if ui.button("Enemy info").clicked() {
                            game_settings.enemy_info = !game_settings.enemy_info;
                            ui.close_menu();
                        }
                        if ui.button("Settings").clicked() {
                            ui.close_menu();
                            todo!();
                        }
                    });
                });
            });
        });
}

pub fn resources_panel(
    mut contexts: EguiContexts,
    mut weapon_q: Query<&mut Weapon>,
    app_state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<GameState>>,
    player: Res<Player>,
    mut night_stats: ResMut<NightStats>,
    time: Res<Time>,
    mut game_settings: ResMut<GameSettings>,
    images: Local<Images>,
) {
    let day_texture = contexts.add_image(images.day.clone_weak());
    let night_texture = contexts.add_image(images.night.clone_weak());
    let person_texture = contexts.add_image(images.person.clone_weak());
    let wall_texture = contexts.add_image(images.wall.clone_weak());
    let fence_texture = contexts.add_image(images.fence.clone_weak());
    let bullets_texture = contexts.add_image(images.bullets.clone_weak());
    let gasoline_texture = contexts.add_image(images.gasoline.clone_weak());
    let materials_texture = contexts.add_image(images.materials.clone_weak());
    let spot_texture = contexts.add_image(images.spot.clone_weak());
    let hourglass_texture = contexts.add_image(images.hourglass.clone_weak());
    let clock_texture = contexts.add_image(images.clock.clone_weak());

    egui::TopBottomPanel::bottom("Resources")
        .exact_height(RESOURCES_PANEL_SIZE.y)
        .show(contexts.ctx_mut(), |ui| {
            ui.horizontal_centered(|ui| {
                ui.add_space(5.);

                match *app_state.get() {
                    AppState::Day => ui.add_image(day_texture, [20., 20.]).on_hover_text("Day"),
                    _ => ui
                        .add_image(night_texture, [20., 20.])
                        .on_hover_text("Night"),
                };
                ui.add(egui::Label::new(player.day.to_string()));

                ui.add_space(5.);
                ui.separator();
                ui.add_space(5.);

                ui.add_image(person_texture, [20., 20.])
                    .on_hover_text("Survivors");
                ui.add(egui::Label::new(player.survivors.to_string()));

                ui.add_space(15.);

                ui.add_image(wall_texture, [20., 20.])
                    .on_hover_text("Fortress strength");
                ui.add(
                    egui::ProgressBar::new(player.wall.health / player.wall.max_health)
                        .desired_width(200.)
                        .desired_height(20.)
                        .text(
                            RichText::new(format!(
                                "{:.0} / {}",
                                player.wall.health, player.wall.max_health
                            ))
                            .size(NORMAL_FONT_SIZE),
                        ),
                );

                if player.fence.max_health > 0. {
                    ui.add_space(5.);

                    ui.add_image(fence_texture, [20., 20.])
                        .on_hover_text("Fence strength");
                    ui.add(
                        egui::ProgressBar::new(player.fence.health / player.fence.max_health)
                            .desired_width(100.)
                            .desired_height(20.)
                            .text(
                                RichText::new(format!(
                                    "{:.0} / {}",
                                    player.fence.health, player.fence.max_health
                                ))
                                .size(NORMAL_FONT_SIZE),
                            ),
                    );
                }

                ui.add_space(5.);
                ui.separator();
                ui.add_space(5.);

                ui.add_image(bullets_texture, [20., 20.])
                    .on_hover_text("Bullets");
                ui.add(egui::Label::new(format!("{:.0}", player.resources.bullets)));

                ui.add_space(15.);

                ui.add_image(gasoline_texture, [20., 20.])
                    .on_hover_text("Gasoline");
                ui.add(egui::Label::new(format!(
                    "{:.0}",
                    player.resources.gasoline
                )));

                ui.add_space(15.);

                ui.add_image(materials_texture, [20., 20.])
                    .on_hover_text("Materials");
                ui.add(egui::Label::new(format!(
                    "{:.0}",
                    player.resources.materials
                )));

                ui.add_space(5.);
                ui.separator();
                ui.add_space(5.);

                ui.add_image(spot_texture, [20., 20.])
                    .on_hover_text("Occupied / Total spots on wall");
                ui.add(egui::Label::new(format!(
                    "{} / {}",
                    player.weapons.spots.iter().filter(|&x| x.is_some()).count(),
                    player.weapons.spots.len()
                )));

                ui.scope_builder(
                    UiBuilder {
                        invisible: *app_state.get() != AppState::Night,
                        ..default()
                    },
                    |ui| {
                        ui.add_space(5.);
                        ui.separator();

                        ui.add_image(hourglass_texture, [20., 20.])
                            .on_hover_text("Remaining night time");
                        night_stats
                            .timer
                            .tick(scale_duration(time.delta(), game_settings.speed));
                        ui.add(egui::Label::new(format!(
                            "{}s",
                            night_stats.timer.remaining().as_secs()
                        )));

                        ui.add_space(15.);

                        ui.add_image(clock_texture, [20., 20.])
                            .on_hover_text("Game speed");
                        let speed = ui.add(
                            egui::DragValue::new(&mut game_settings.speed)
                                .range(0..=MAX_GAME_SPEED as u32)
                                .fixed_decimals(1)
                                .speed(GAME_SPEED_STEP)
                                .suffix("x"),
                        );

                        if speed.changed() {
                            if game_settings.speed == 0. {
                                next_state.set(GameState::Paused);
                            } else {
                                weapon_q.iter_mut().for_each(|mut w| {
                                    let params = player.weapons.settings.get(w.as_ref());
                                    w.update(params, game_settings.as_ref())
                                });
                                next_state.set(GameState::Running);
                            }
                        }
                    },
                );
            });
        });
}

pub fn weapons_panel(
    mut contexts: EguiContexts,
    mut weapon_q: Query<&mut Weapon>,
    mut player: ResMut<Player>,
    game_settings: Res<GameSettings>,
    app_state: Res<State<AppState>>,
    images: Local<Images>,
) {
    let weapon_texture = contexts.add_image(images.weapon.clone_weak());
    let lightning_texture = contexts.add_image(images.lightning.clone_weak());

    egui::SidePanel::right("Weapons panel")
        .exact_width(WEAPONS_PANEL_SIZE.x)
        .resizable(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.add_enabled_ui(*app_state.get() == AppState::Night, |ui| {
                ui.add_space(5.);
                ui.vertical_centered(|ui| {
                    ui.horizontal(|ui| {
                        ui.add_space(65.);
                        ui.add_image(weapon_texture, [30., 30.]);
                        ui.heading("Weapons");
                    });
                });

                ui.add_space(5.);
                ui.separator();
                ui.add_space(5.);

                // Sentry gun
                ui.horizontal(|ui| {
                    let sg = &mut player.weapons.settings.sentry_gun;

                    ui.add(egui::Label::new(format!("{}: ", sg.name)));

                    let sentry_gun_slider = ui
                        .add(egui::Slider::new(&mut sg.fire_rate, 0..=sg.max_fire_rate))
                        .on_hover_text("Sentry guns shoot N bullets per second.");

                    if sentry_gun_slider.changed() {
                        weapon_q
                            .iter_mut()
                            .filter(|w| matches!(*w.deref(), Weapon::SentryGun { .. }))
                            .for_each(|mut w| w.as_mut().update(sg, game_settings.as_ref()))
                    }
                });

                if player.fence.max_health > 0. {
                    ui.add_space(5.);
                    ui.separator();
                    ui.add_space(5.);

                    ui.add_enabled_ui(player.fence.health > 0., |ui| {
                        ui.horizontal(|ui| {
                            ui.add(egui::Label::new("Enable electric fence: "));
                            ui.add(toggle(&mut player.fence.enabled)).on_hover_text(
                                "The electric fence does damage to adjacent enemies, but costs gasoline.",
                            );

                            if player.fence.enabled {
                                ui.add_image(lightning_texture, [20., 20.]);
                            }
                        });
                    });
                }
            });
        });
}

pub fn info_panel(
    mut contexts: EguiContexts,
    player: Res<Player>,
    app_state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    images: Local<Images>,
) {
    let game_over_texture = contexts.add_image(images.game_over.clone_weak());

    egui::Window::new("info panel")
        .title_bar(false)
        .fixed_size((MAP_SIZE.x * 0.6, MAP_SIZE.y * 0.8))
        .fixed_pos((MAP_SIZE.x * 0.2, MAP_SIZE.y * 0.2))
        .show(contexts.ctx_mut(), |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(10.);
                match *app_state.get() {
                    AppState::StartGame => {
                        ui.heading("Welcome to Fortress!");

                        ui.add_space(15.);

                        egui::ScrollArea::vertical()
                            .max_width(SIZE.x * 0.4)
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.add_space(85.);
                                    ui.with_layout(
                                        Layout::top_down(Align::LEFT),
                                        |ui| {
                                            ui.add_space(5.);
                                            ui.label(
                                                "The world has been conquered by insects. Together \
                                                with a handful of survivors, you have build a fortress \
                                                to defend yourself from their ferocious attacks.\n\n\
                                                Every night, an ever increasing swarm of insects attacks \
                                                the fortress. Kill them before they reach the wall! \
                                                When they do, they hit the wall, reducing its resistance. \
                                                If the wall is destroyed, the monsters can freely enter \
                                                the fortress and kill everyone inside.\n\n \
                                                During the day, you can collect resources and upgrade \
                                                your weapon arsenal to prepare yourself for the following \
                                                night. During the attack, you can choose how/when to use \
                                                the weapons you have to your disposal. But be careful, \
                                                everything has a cost! Manage your resources wisely or \
                                                you won't be able to stop the insects tomorrow...");
                                            ui.add_space(5.);
                                        })
                                })
                            });

                        ui.add_space(15.);

                        if ui.add_button("Start game").clicked() {
                            next_state.set(AppState::Night);
                        }
                    },
                    AppState::EndNight => {
                        ui.heading(format!("You survived night {}!", player.day));

                        ui.add_night_stats(player);

                        ui.with_layout(Layout::right_to_left(Align::RIGHT), |ui| {
                            ui.add_space(10.);

                            if ui.add_button("Continue").clicked() {
                                next_state.set(AppState::Day);
                            }
                        });
                    },
                    AppState::GameOver => {
                        ui.add_image(game_over_texture,[400., 100.]);

                        ui.heading(format!("You survived {} nights!", player.day - 1));

                        ui.add_night_stats(player);

                        ui.horizontal(|ui| {
                            ui.add_space(190.);

                            if ui.add_button("New game").clicked() {
                                next_state.set(AppState::StartGame);
                            }

                            ui.add_space(20.);

                            if ui.add_button("Quit").clicked() {
                                std::process::exit(0);
                            }
                        });
                    },
                    _ => unreachable!(),
                }

                ui.add_space(10.);
            });
        });
}

pub fn enemy_info_panel(
    mut contexts: EguiContexts,
    mut game_settings: ResMut<GameSettings>,
    spawner: Res<EnemySpawner>,
    images: Local<Images>,
) {
    if game_settings.enemy_info {
        let textures = spawner
            .enemies
            .iter()
            .map(|e| contexts.add_image(images.enemies.get(e.name).unwrap().clone_weak()))
            .collect::<Vec<_>>();

        egui::Window::new("Enemy info")
            .collapsible(false)
            .open(&mut game_settings.enemy_info)
            .fixed_size((MAP_SIZE.x * 0.6, MAP_SIZE.y * 0.8))
            .default_pos((MAP_SIZE.x * 0.2, MAP_SIZE.y * 0.2))
            .show(contexts.ctx_mut(), |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(10.);

                    egui::ScrollArea::vertical()
                        .max_width(SIZE.x * 0.4)
                        .show(ui, |ui| {
                            spawner.enemies.iter().enumerate().for_each(|(i, e)| {
                                ui.add_space(20.);
                                ui.horizontal(|ui| {
                                    ui.add_image(*textures.get(i).unwrap(), [140., 160.]);

                                    egui::Grid::new("Enemy info")
                                        .num_columns(2)
                                        .spacing([4.0, 4.0])
                                        .striped(true)
                                        .show(ui, |ui| {
                                            ui.label("Name");
                                            ui.label(e.name);
                                            ui.end_row();

                                            ui.label("Health");
                                            ui.label(e.health.to_string());
                                            ui.end_row();

                                            ui.label("Armor");
                                            ui.label(e.armor.to_string());
                                            ui.end_row();
                                        });
                                });
                            });
                        });
                });
            });
    }
}

pub fn clear_map(
    mut commands: Commands,
    weapon_q: Query<Entity, With<Weapon>>,
    bullet_q: Query<Entity, With<Bullet>>,
    enemy_q: Query<Entity, With<Enemy>>,
) {
    weapon_q.iter().for_each(|w| commands.entity(w).despawn());
    bullet_q.iter().for_each(|b| commands.entity(b).despawn());
    enemy_q
        .iter()
        .for_each(|e| commands.entity(e).despawn_recursive());
}
