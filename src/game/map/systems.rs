use super::components::*;
use super::constants::*;
use crate::game::components::*;
use crate::game::enemy::components::Enemy;
use crate::game::resources::{GameSettings, Player, WaveStats};
use crate::game::weapon::components::{Bullet, Weapon, WeaponId, WeaponSettings};
use crate::game::{AppState, GameState};
use crate::utils::{CustomUi, EnumDisplay};
use bevy::color::palettes::basic::WHITE;
use bevy::prelude::*;
use bevy_egui::egui::{RichText, Style, TextStyle, UiBuilder};
use bevy_egui::{egui, EguiContexts};
use catppuccin_egui;

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

    commands.spawn((
        Sprite {
            image: asset_server.load("map/wall.png"),
            custom_size: Some(WALL_SIZE),
            ..default()
        },
        Transform::from_xyz(
            -WEAPONS_PANEL_SIZE.x * 0.5,
            SIZE.y * 0.5 - MENU_PANEL_SIZE.y - MAP_SIZE.y - WALL_SIZE.y * 0.5,
            0.1,
        ),
        Wall,
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
                    egui::menu::menu_button(ui, "State", |ui| {
                        if ui
                            .add_enabled(
                                *app_state.get() == AppState::Game,
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
                    egui::menu::menu_button(ui, "Settings", |ui| {
                        if ui.button("Toggle audio").clicked() {
                            std::process::exit(0);
                        }
                    });
                });
            });
        });
}

pub fn resources_panel(
    mut contexts: EguiContexts,
    mut weapon_q: Query<&mut Weapon>,
    weapon_settings: ResMut<WeaponSettings>,
    app_state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<GameState>>,
    player: Res<Player>,
    mut wave_stats: ResMut<WaveStats>,
    time: Res<Time>,
    mut game_settings: ResMut<GameSettings>,
    images: Local<Images>,
) {
    let day_texture = contexts.add_image(images.day.clone_weak());
    let fortress_texture = contexts.add_image(images.fortress.clone_weak());
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

                ui.add_image(day_texture, [20., 20.]).on_hover_text("Day");
                ui.add(egui::Label::new(player.day.to_string()));

                ui.add_space(5.);
                ui.separator();
                ui.add_space(5.);

                ui.add_image(fortress_texture, [20., 20.])
                    .on_hover_text("Fortress strength");
                ui.add(
                    egui::ProgressBar::new(player.wall.health / player.wall.max_health)
                    .desired_width(200.)
                    .desired_height(20.)
                    .text(
                        RichText::new(format!(
                            "{} / {}",
                            player.wall.health, player.wall.max_health
                        ))
                        .size(NORMAL_FONT_SIZE),
                    ),
                );

                ui.add_space(5.);
                ui.separator();
                ui.add_space(5.);

                ui.add_image(bullets_texture, [20., 20.])
                    .on_hover_text("Bullets");
                ui.add(egui::Label::new(player.resources.bullets.to_string()));

                ui.add_space(15.);

                ui.add_image(gasoline_texture, [20., 20.])
                    .on_hover_text("Gasoline");
                ui.add(egui::Label::new(player.resources.gasoline.to_string()));

                ui.add_space(15.);

                ui.add_image(materials_texture, [20., 20.])
                    .on_hover_text("Materials");
                ui.add(egui::Label::new(player.resources.materials.to_string()));

                ui.add_space(5.);
                ui.separator();
                ui.add_space(5.);

                ui.add_image(spot_texture, [20., 20.])
                    .on_hover_text("Occupied / Total spots on wall");
                ui.add(egui::Label::new(format!(
                    "{} / {}",
                    player.weapons.iter().filter(|&x| x.is_some()).count(),
                    player.wall.max_spots
                )));

                ui.scope_builder(
                    UiBuilder {
                        invisible: *app_state.get() != AppState::Game,
                        ..default()
                    },
                    |ui| {
                        ui.add_space(5.);
                        ui.separator();

                        ui.add_space(180.);

                        ui.add_image(hourglass_texture, [20., 20.])
                            .on_hover_text("Remaining night time");
                        wave_stats.time.tick(scale_duration(time.delta() * game_settings.speed));
                        ui.add(egui::Label::new(format!("{}s", wave_stats.time.remaining().as_secs())));

                        ui.add_space(15.);

                        ui.add_image(clock_texture, [20., 20.])
                            .on_hover_text("Game speed");
                        let speed = ui.add(
                            egui::DragValue::new(&mut game_settings.speed)
                                .range(0..=5)
                                .fixed_decimals(1)
                                .speed(0.5)
                                .suffix("x"),
                        );

                        if speed.changed() {
                            if game_settings.speed == 0. {
                                next_state.set(GameState::Paused);
                            } else {
                                weapon_q.iter_mut().for_each(|mut w| {
                                    w.as_mut()
                                        .update(weapon_settings.as_ref(), game_settings.as_ref())
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
    mut weapon_settings: ResMut<WeaponSettings>,
    game_settings: Res<GameSettings>,
    app_state: Res<State<AppState>>,
    images: Local<Images>,
) {
    let weapon_texture = contexts.add_image(images.weapon.clone_weak());

    egui::SidePanel::right("Weapons panel")
        .exact_width(WEAPONS_PANEL_SIZE.x)
        .resizable(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.add_enabled_ui(*app_state.get() == AppState::Game, |ui| {
                ui.add_space(5.);
                ui.vertical_centered(|ui| {
                    ui.horizontal(|ui| {
                        ui.add_space(55.);
                        ui.add_image(weapon_texture, [30., 30.]);
                        ui.heading("Weapons");
                    });
                });

                ui.add_space(5.);
                ui.separator();
                ui.add_space(5.);

                // Sentry gun
                ui.horizontal(|ui| {
                    let settings = weapon_settings
                        .as_mut()
                        .get_params_mut(&WeaponId::SentryGun);

                    ui.add(egui::Label::new(format!("{}: ", settings.name)));

                    let sentry_gun_slider = ui
                        .add(egui::Slider::new(
                            &mut settings.fire_rate,
                            0..=settings.max_fire_rate,
                        ))
                        .on_hover_text("Sentry guns shoot N bullets per second.");

                    if sentry_gun_slider.dragged() {
                        weapon_q
                            .iter_mut()
                            .filter(|w| w.id == WeaponId::SentryGun)
                            .for_each(|mut w| {
                                w.as_mut().update(weapon_settings.as_ref(), game_settings.as_ref())
                            })
                    }
                });
            });
        });
}

pub fn start_end_game_panel(
    mut contexts: EguiContexts,
    player: Res<Player>,
    app_state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    images: Local<Images>,
) {
    let game_over_texture = contexts.add_image(images.game_over.clone_weak());

    egui::Window::new("start/end game")
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
                                        egui::Layout::top_down(egui::Align::LEFT),
                                        |ui| {
                                            ui.add_space(5.);
                                            ui.label(
                                                "The world has been conquered by monsters. Together \
                                                with a handful of survivors, you have build a fortress \
                                                to defend yourself from their ferocious attacks.\n\n\
                                                Every night, an ever increasing swarm of monsters attacks \
                                                the fortress. Kill them before they reach the wall! \
                                                When they do, they hit the wall, reducing its resistance. \
                                                If the wall is destroyed, the monsters can freely enter \
                                                the fortress and kill everyone inside (the game is lost). \
                                                \n\nDuring the day, you can collect resources and upgrade \
                                                your weapon arsenal to prepare yourself for the following \
                                                night. During the attack, you can choose how/when to use \
                                                the weapons you have to your disposal. But be careful, \
                                                everything has a cost! Manage your resources wisely or \
                                                you won't be able to stop the monsters tomorrow...");
                                            ui.add_space(5.);
                                        })
                                })
                            });

                        ui.add_space(15.);

                        if ui.add_button("Start game").clicked() {
                            next_state.set(AppState::Game);
                        }
                    },
                    AppState::GameOver => {
                        ui.add_image(game_over_texture,[400., 100.]);

                        ui.heading(format!("You survived {} days!", player.day - 1));

                        ui.add_space(30.);

                        ui.horizontal(|ui| {
                            ui.add_space(200.);
                            egui::Grid::new("wave stats")
                                .num_columns(2)
                                .spacing([40.0, 4.0])
                                .striped(true)
                                .show(ui, |ui| {
                                    ui.label("Enemy");
                                    ui.label("Killed / Spawned");
                                    ui.end_row();

                                    player.stats
                                        .get(&player.day)
                                        .unwrap()
                                        .enemies.iter().for_each(|(k, v)| {
                                            ui.label(k.name());
                                            ui.label(format!("{} / {}", v.killed, v.spawned));
                                            ui.end_row();
                                        });
                                });
                        });

                        ui.add_space(30.);

                        ui.horizontal(|ui| {
                            ui.add_space(170.);

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
