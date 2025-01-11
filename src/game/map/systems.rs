use super::components::*;
use crate::constants::*;
use crate::game::assets::WorldAssets;
use crate::game::enemy::components::{Enemy, EnemyManager, Size};
use crate::game::enemy::utils::EnemySelection;
use crate::game::map::utils::{collision, toggle, CustomUi};
use crate::game::resources::{GameSettings, NightStats, Player};
use crate::game::weapon::components::*;
use crate::game::weapon::utils::resolve_impact;
use crate::game::{AppState, GameState};
use crate::utils::*;
use bevy::color::palettes::basic::WHITE;
use bevy::prelude::*;
use bevy_egui::egui::{Align, CursorIcon, Layout, RichText, Style, TextStyle, UiBuilder};
use bevy_egui::{egui, EguiContexts};

pub fn set_style(mut contexts: EguiContexts) {
    let context = contexts.ctx_mut();

    let mut style = Style::default();
    style.text_styles.get_mut(&TextStyle::Body).unwrap().size = NORMAL_FONT_SIZE;
    style.text_styles.get_mut(&TextStyle::Button).unwrap().size = NORMAL_FONT_SIZE;
    style.text_styles.get_mut(&TextStyle::Heading).unwrap().size = LARGE_FONT_SIZE;
    context.set_style(style);
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
            MAP_Z,
        ),
        Map,
    ));

    commands.spawn((
        Sprite {
            image: asset_server.load("map/fow.png"),
            custom_size: Some(FOW_SIZE),
            ..default()
        },
        Transform::from_xyz(
            -WEAPONS_PANEL_SIZE.x * 0.5,
            SIZE.y * 0.5 - MENU_PANEL_SIZE.y - FOW_SIZE.y * 0.5,
            FOW_Z,
        ),
        FogOfWar,
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
                f32::MAX - 0.1,
            ),
            Visibility::Hidden,
            PauseWrapper,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text2d::new("Paused".to_string()),
                TextColor(Color::from(WHITE)),
                TextLayout::new_with_justify(JustifyText::Center),
                Transform::from_xyz(0., 0., f32::MAX),
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
    assets: Local<WorldAssets>,
) {
    let day_texture = contexts.add_image(assets.get_image("day"));
    let night_texture = contexts.add_image(assets.get_image("night"));
    let person_texture = contexts.add_image(assets.get_image("person"));
    let wall_texture = contexts.add_image(assets.get_image("wall"));
    let fence_texture = contexts.add_image(assets.get_image("fence"));
    let bullets_texture = contexts.add_image(assets.get_image("bullets"));
    let gasoline_texture = contexts.add_image(assets.get_image("gasoline"));
    let materials_texture = contexts.add_image(assets.get_image("materials"));
    let spot_texture = contexts.add_image(assets.get_image("spot"));
    let hourglass_texture = contexts.add_image(assets.get_image("hourglass"));
    let clock_texture = contexts.add_image(assets.get_image("clock"));

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
                            .desired_width(160.)
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
                                weapon_q.iter_mut().for_each(|mut w| w.update(&player));
                                next_state.set(GameState::Running);
                            }
                        }
                    },
                );
            });
        });
}

pub fn weapons_panel(
    mut commands: Commands,
    mut contexts: EguiContexts,
    mut weapon_q: Query<&mut Weapon>,
    enemy_q: Query<EnemyQ, (With<Enemy>, Without<FogOfWar>)>,
    fence_q: Query<SpriteQ, (With<Fence>, Without<FogOfWar>)>,
    wall_q: Query<SpriteQ, (With<Wall>, Without<FogOfWar>)>,
    mut fow_q: Query<&mut Transform, With<FogOfWar>>,
    mut player: ResMut<Player>,
    app_state: Res<State<AppState>>,
    game_state: Res<State<GameState>>,
    weapons: Res<WeaponManager>,
    assets: Local<WorldAssets>,
    asset_server: Res<AssetServer>,
) {
    let weapon_texture = contexts.add_image(assets.get_image("weapon"));
    let lightning_texture = contexts.add_image(assets.get_image("lightning"));
    let fence_texture = contexts.add_image(assets.get_image("fence"));
    let bomb_texture = contexts.add_image(assets.get_image("bomb"));
    let mine_texture = contexts.add_image(assets.get_image("mine"));
    let spotlight_texture = contexts.add_image(assets.get_image("spotlight"));
    let bulb_texture = contexts.add_image(assets.get_image("bulb"));
    let bullets_texture = contexts.add_image(assets.get_image("bullets"));
    let gasoline_texture = contexts.add_image(assets.get_image("gasoline"));

    egui::SidePanel::right("Weapons panel")
        .exact_width(WEAPONS_PANEL_SIZE.x)
        .resizable(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.add_enabled_ui(*app_state.get() == AppState::Night, |ui| {
                ui.add_space(7.);

                ui.vertical_centered(|ui| {
                    ui.horizontal(|ui| {
                        ui.add_space(65.);
                        ui.add_image(weapon_texture, [30., 30.]);
                        ui.heading("Weapons");
                    });
                });

                ui.add_space(7.);

                ui.separator();

                // Machine gun
                if player.weapons.spots.iter().any(|w| match w {
                    Some(w) => *w == WeaponName::MachineGun,
                    None => false,
                }) {
                    ui.add_space(7.);
                    ui.horizontal(|ui| {
                        let label = ui.add(egui::Label::new(format!("{:?}: ", WeaponName::MachineGun))).on_hover_cursor(CursorIcon::PointingHand);

                        ui.add(egui::Slider::new(&mut player.weapons.settings.sentry_gun_fire_rate, 0..=MAX_MACHINE_GUN_FIRE_RATE))
                            .on_hover_text("Shoot N bullets per second.");

                        if label.clicked() {
                            player.weapons.settings.sentry_gun_fire_rate = if player.weapons.settings.sentry_gun_fire_rate > 0 {
                                0
                            } else {
                                MAX_MACHINE_GUN_FIRE_RATE
                            };
                        }
                    });
                }

                // Flamethrower
                if player.weapons.spots.iter().any(|w| match w {
                    Some(w) => *w == WeaponName::Flamethrower,
                    None => false,
                }) {
                    ui.add_space(7.);
                    ui.horizontal(|ui| {
                        let label = ui.add(egui::Label::new(format!("{:?}: ", WeaponName::Flamethrower))).on_hover_cursor(CursorIcon::PointingHand);

                        ui.add(egui::Slider::new(&mut player.weapons.settings.flamethrower_power, 0..=MAX_FLAMETHROWER_POWER))
                            .on_hover_text("More power means more range, but costs more.");

                        if label.clicked() {
                            player.weapons.settings.flamethrower_power = if player.weapons.settings.flamethrower_power > 0 {
                                0
                            } else {
                                MAX_FLAMETHROWER_POWER
                            };
                        }
                    });
                }

                // AAA
                if player.weapons.spots.iter().any(|w| match w {
                    Some(w) => *w == WeaponName::AAA,
                    None => false,
                }) {
                    ui.add_space(7.);
                    ui.horizontal(|ui| {
                        ui.add(egui::Label::new(format!("{:?}: ", WeaponName::AAA)));
                        ui.selectable_value(&mut player.weapons.settings.aaa_fire_strategy, AAAFireStrategy::None, AAAFireStrategy::None.name())
                            .on_hover_text("Don't fire.");
                        ui.selectable_value(&mut player.weapons.settings.aaa_fire_strategy, AAAFireStrategy::All, AAAFireStrategy::All.name())
                            .on_hover_text("Fire at all enemies dealing reduced damage.");
                        ui.selectable_value(&mut player.weapons.settings.aaa_fire_strategy, AAAFireStrategy::Airborne, AAAFireStrategy::Airborne.name())
                            .on_hover_text("Fire only at flying enemies, dealing more damage.");
                    });
                }

                // Mortar
                if player.weapons.spots.iter().any(|w| match w {
                    Some(w) => *w == WeaponName::Mortar,
                    None => false,
                }) {
                    ui.add_space(7.);
                    ui.horizontal(|ui| {
                        ui.add(egui::Label::new(format!("{:?}: ", WeaponName::Mortar)));
                        ui.selectable_value(&mut player.weapons.settings.mortar_shell, MortarShell::None, MortarShell::None.name())
                            .on_hover_text("Don't fire.");
                        ui.selectable_value(&mut player.weapons.settings.mortar_shell, MortarShell::Light, MortarShell::Light.name())
                            .on_hover_text("Light shells do standard damage and don't damage structures.");
                        ui.selectable_value(&mut player.weapons.settings.mortar_shell, MortarShell::Heavy, MortarShell::Heavy.name())
                            .on_hover_text("Heavy shells do more damage, but cost more and damage structures.");
                    });
                }
                
                // Turret
                if player.weapons.spots.iter().any(|w| match w {
                    Some(w) => *w == WeaponName::Turret,
                    None => false,
                }) {
                    ui.add_space(7.);
                    ui.horizontal(|ui| {
                        ui.add(egui::Label::new(format!("{:?}: ", WeaponName::Turret)));
                        ui.selectable_value(&mut player.weapons.settings.turret_fire_strategy, FireStrategy::None, FireStrategy::None.name())
                            .on_hover_text("Don't fire.");
                        ui.selectable_value(&mut player.weapons.settings.turret_fire_strategy, FireStrategy::Closest, FireStrategy::Closest.name())
                            .on_hover_text("Fire on the closest enemy.");
                        ui.selectable_value(&mut player.weapons.settings.turret_fire_strategy, FireStrategy::Strongest, FireStrategy::Strongest.name())
                            .on_hover_text("Fire on the strongest enemy.");
                    });
                }

                // Missile launcher
                if player.weapons.spots.iter().any(|w| match w {
                    Some(w) => *w == WeaponName::MissileLauncher,
                    None => false,
                }) {
                    ui.add_space(7.);
                    ui.horizontal(|ui| {
                        let label = ui.add(egui::Label::new(format!("{:?}: ", WeaponName::MissileLauncher))).on_hover_cursor(CursorIcon::PointingHand);

                        ui.add(egui::Slider::new(&mut player.weapons.settings.missile_launcher_shells, 0..=MAX_MISSILE_LAUNCHER_SHELLS))
                            .on_hover_text("Shoot N shells per firing round.");

                        if label.clicked() {
                            player.weapons.settings.missile_launcher_shells = if player.weapons.settings.missile_launcher_shells > 0 {
                                0
                            } else {
                                MAX_MISSILE_LAUNCHER_SHELLS
                            };
                        }
                    });
                }

                // Update all weapons with the selected settings
                weapon_q
                    .iter_mut()
                    .for_each(|mut w| w.as_mut().update(&player));

                ui.add_space(7.);

                if player.weapons.bombs > 0 || player.weapons.mines > 0 {
                    ui.separator();

                    ui.add_enabled_ui(player.weapons.bombs > 0, |ui| {
                        ui.add_space(7.);
                        ui.horizontal(|ui| {
                            ui.add_image(bomb_texture, [20., 20.]);
                            let label = ui.add(egui::Label::new(format!("Bomb ({}): ", player.weapons.bombs)))
                                .on_hover_cursor(CursorIcon::PointingHand)
                                .on_hover_text("Launch!");
                            ui.selectable_value(&mut player.weapons.settings.bombing_strategy, FireStrategy::Density, FireStrategy::Density.name())
                                .on_hover_text("Launch at highest enemy density location.");
                            ui.selectable_value(&mut player.weapons.settings.bombing_strategy, FireStrategy::Strongest, FireStrategy::Strongest.name())
                                .on_hover_text("Launch at strongest enemy.");

                            if label.clicked() && *game_state.get() == GameState::Running {
                                player.weapons.bombs -= 1;

                                let mut bomb = weapons.bomb.clone();

                                let (_, enemy_t, enemy) = match player.weapons.settings.bombing_strategy {
                                    FireStrategy::Density => enemy_q.iter().get_most_dense(&bomb.impact),
                                    FireStrategy::Strongest => enemy_q.iter().get_strongest(),
                                    _ => unreachable!(),
                                };

                                let pos = Vec3::new(enemy_t.translation.x, SIZE.y * 0.5, 3.0);
                                bomb.max_distance = calculate_distance(
                                    enemy,
                                    &enemy_t.translation,
                                    &bomb,
                                    &pos,
                                    get_structure_top(fence_q.get_single()),
                                    get_structure_top(wall_q.get_single()),
                                    player.technology.movement_prediction
                                ).length();

                                commands.spawn((
                                    Sprite {
                                        image: asset_server.load(bomb.image),
                                        custom_size: Some(bomb.dim),
                                        ..default()
                                    },
                                    Transform {
                                        translation: pos,
                                        rotation: Quat::from_rotation_z(bomb.angle),
                                        ..default()
                                    },
                                    bomb,
                                ));
                            }
                        });
                    });

                    ui.add_enabled_ui(player.weapons.mines > 0, |ui| {
                        ui.add_space(7.);
                        ui.horizontal(|ui| {
                            ui.add_image(mine_texture, [20., 20.]);
                            ui.add(egui::Label::new(format!("Mine ({}): ", player.weapons.mines)));
                            ui.selectable_value(&mut player.weapons.settings.mine_sensibility, Size::Small, Size::Small.name())
                                .on_hover_text("Detonate for all enemies.");
                            ui.selectable_value(&mut player.weapons.settings.mine_sensibility, Size::Medium, Size::Medium.name())
                                .on_hover_text("Detonate for medium and large enemies.");
                            ui.selectable_value(&mut player.weapons.settings.mine_sensibility, Size::Large, Size::Large.name())
                                .on_hover_text("Detonate only for large enemies.");
                        });
                    });

                    ui.add_space(7.);
                }

                if player.fence.max_health > 0. || player.technology.spotlight {
                    ui.separator();

                    if player.fence.max_health > 0. {
                        ui.add_space(7.);
                        ui.add_enabled_ui(player.fence.health > 0. && player.resources >= player.fence.cost, |ui| {
                            ui.horizontal(|ui| {
                                ui.add_image(fence_texture, [20., 25.]);
                                let label = ui.add(egui::Label::new("Fence: ")).on_hover_cursor(CursorIcon::PointingHand);
                                ui.add(toggle(&mut player.fence.enabled)).on_hover_text(
                                    "Electrifying the fence does damage to adjacent enemies, but costs gasoline.",
                                );

                                if label.clicked() {
                                    player.fence.enabled = !player.fence.enabled;
                                }

                                if player.fence.enabled {
                                    ui.add_image(lightning_texture, [20., 20.]);
                                }
                            });
                        });
                    }

                    if player.technology.spotlight {
                        ui.add_space(7.);
                        ui.add_enabled_ui(*game_state.get() == GameState::Running && player.resources >= player.spotlight.cost, |ui| {
                            ui.horizontal(|ui| {
                                ui.add_image(spotlight_texture, [20., 20.]);
                                let label = ui.add(egui::Label::new("Spotlight: ")).on_hover_cursor(CursorIcon::PointingHand);
                                ui.add(egui::Slider::new(&mut player.spotlight.power, 0..=MAX_SPOTLIGHT_POWER).show_value(false))
                                    .on_hover_text("More power means more visibility, but costs more gasoline.");

                                if player.spotlight.power > 0 {
                                    ui.add_image(bulb_texture, [20., 20.]);
                                }

                                if label.clicked() {
                                    player.spotlight.power = if player.spotlight.power > 0 {
                                        0
                                    } else {
                                        MAX_SPOTLIGHT_POWER
                                    }
                                }

                                if let Ok(mut fow) = fow_q.get_single_mut() {
                                    fow.translation.y = SIZE.y * 0.5
                                        - MENU_PANEL_SIZE.y
                                        - FOW_SIZE.y * 0.5
                                        + (FOW_SIZE.y / MAX_SPOTLIGHT_POWER as f32 * player.spotlight.power as f32);
                                }
                            });
                        });
                    }

                    ui.add_space(7.);
                }

                ui.separator();
                ui.add_space(7.);

                ui.horizontal(|ui| {
                    ui.add_image(bullets_texture, [20., 25.]);
                    ui.add_enabled_ui(player.resources.materials >= 300., |ui| {
                        let bullet_button_100 = ui.add_sized([30., 30.], egui::Button::new("+100"))
                            .on_hover_text("Buy 100 bullets for 300 materials.");
                        if bullet_button_100.clicked() {
                            player.resources.bullets += 100.;
                            player.resources.materials -= 300.;
                        }
                    });

                    ui.add_enabled_ui(player.resources.materials >= 1500., |ui| {
                        let bullet_button_500 = ui.add_sized([30., 30.], egui::Button::new("+500"))
                            .on_hover_text("Buy 500 bullets for 1500 materials.");
                        if bullet_button_500.clicked() {
                            player.resources.bullets += 500.;
                            player.resources.materials -= 1500.;
                        }
                    });

                    ui.add_space(13.);

                    ui.add_image(gasoline_texture, [20., 20.]);
                    ui.add_enabled_ui(player.resources.materials >= 300., |ui| {
                        let gasoline_button = ui.add_sized([30., 30.], egui::Button::new("+100"))
                            .on_hover_text("Buy 100 gasoline for 300 materials.");
                        if gasoline_button.clicked() {
                            player.resources.gasoline += 100.;
                            player.resources.materials -= 300.;
                        }
                    });

                    ui.add_enabled_ui(player.resources.materials >= 1500., |ui| {
                        let gasoline_button_500 = ui.add_sized([30., 30.], egui::Button::new("+500"))
                            .on_hover_text("Buy 500 gasoline for 1500 materials.");
                        if gasoline_button_500.clicked() {
                            player.resources.gasoline += 500.;
                            player.resources.materials -= 1500.;
                        }
                    });
                });
            });
        });
}

pub fn info_panel(
    mut contexts: EguiContexts,
    player: Res<Player>,
    app_state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    assets: Local<WorldAssets>,
    window: Query<&Window>,
) {
    let window_size = window.single().size();
    let game_over_texture = contexts.add_image(assets.get_image("game_over"));

    egui::Window::new("info panel")
        .title_bar(false)
        .fixed_size((MAP_SIZE.x * 0.6, MAP_SIZE.y * 0.8))
        .fixed_pos(
            (
                (window_size.x - WEAPONS_PANEL_SIZE.x) * 0.5  - MAP_SIZE.x * 0.3,
                (window_size.y - RESOURCES_PANEL_SIZE.y) * 0.5 - MAP_SIZE.y * 0.4,
            )
        )
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

                        ui.add_space(15.);

                        ui.label(
                            "The night is over. The sun is rising and the bugs are \
                            retreating. You can now collect resources and upgrade your \
                            weapons before the next night.");

                        ui.add_night_stats(&player);

                        ui.with_layout(Layout::right_to_left(Align::RIGHT), |ui| {
                            ui.add_space(10.);

                            if ui.add_button("Continue").clicked() {
                                next_state.set(AppState::Night);
                            }
                        });
                    },
                    AppState::GameOver => {
                        ui.add_image(game_over_texture,[400., 100.]);

                        ui.heading(format!("You survived {} nights!", player.day - 1));

                        ui.add_night_stats(&player);

                        ui.horizontal(|ui| {
                            ui.add_space(205.);

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
    enemies: Res<EnemyManager>,
    assets: Local<WorldAssets>,
    window: Query<&Window>,
) {
    let window_size = window.single().size();

    if game_settings.enemy_info {
        let textures = enemies
            .list
            .iter()
            .map(|e| contexts.add_image(assets.get_image(e.name)))
            .collect::<Vec<_>>();

        egui::Window::new("Enemy info")
            .collapsible(false)
            .open(&mut game_settings.enemy_info)
            .fixed_size((MAP_SIZE.x * 0.3, MAP_SIZE.y * 0.6))
            .default_pos((
                (window_size.x - WEAPONS_PANEL_SIZE.x) * 0.5 - MAP_SIZE.x * 0.15,
                (window_size.y - RESOURCES_PANEL_SIZE.y) * 0.5 - MAP_SIZE.y * 0.4,
            ))
            .show(contexts.ctx_mut(), |ui| {
                ui.add_space(15.);

                egui::ScrollArea::vertical()
                    .max_width(SIZE.x * 0.4)
                    .show(ui, |ui| {
                        enemies.list.iter().enumerate().for_each(|(i, e)| {
                            if i > 0 {
                                ui.add_space(20.);
                            }

                            ui.horizontal(|ui| {
                                ui.add_image(*textures.get(i).unwrap(), [105., 120.])
                                    .on_hover_text(e.name);

                                ui.add_space(20.);

                                ui.vertical(|ui| {
                                    ui.label(format!("Name: {}", e.name));
                                    ui.label(format!("Size: {:?}", e.size));
                                    ui.label(format!("Health: {}", e.health));
                                    ui.label(format!("Armor: {}", e.armor))
                                        .on_hover_text("Armor reduces incoming damage.");
                                    ui.label(format!("Speed: {}", e.speed)).on_hover_text(
                                        "As percentage of the map's size per second.",
                                    );
                                    ui.label(format!("Can fly: {}", e.can_fly))
                                        .on_hover_text("Flying bugs can pass over constructions.");
                                    ui.label(format!("Damage: {}", e.damage)).on_hover_text(
                                        "Damage dealt to constructions or survivors.",
                                    );
                                })
                            });
                        });
                    });

                ui.add_space(15.);
            });
    }
}

pub fn run_animations(
    mut commands: Commands,
    mut animation_q: Query<(Entity, &Transform, &mut AnimationComponent, &mut Sprite)>,
    mut enemy_q: Query<(Entity, &Transform, &mut Enemy)>,
    fence_q: Query<(Entity, &Transform), With<Fence>>,
    wall_q: Query<(Entity, &Transform), With<Wall>>,
    mut night_stats: ResMut<NightStats>,
    mut player: ResMut<Player>,
    game_settings: Res<GameSettings>,
    time: Res<Time>,
) {
    for (entity, t, mut animation, mut sprite) in animation_q.iter_mut() {
        animation
            .timer
            .tick(scale_duration(time.delta(), game_settings.speed));

        if animation.timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index += 1;

                // Resolve explosion damage at third of the animation
                if atlas.index == animation.last_index / 3 {
                    if let Some(Explosion { radius, damage, .. }) = &animation.explosion {
                        // Resolve damage to structures
                        if let Some((entity, t2)) = fence_q.iter().next() {
                            if collision(
                                &t.translation,
                                &Vec2::splat(*radius),
                                &t2.translation,
                                &FENCE_SIZE,
                            ) {
                                if player.fence.health > damage.penetration {
                                    player.fence.health -= damage.penetration;
                                } else {
                                    player.fence.health = 0.;
                                    commands.entity(entity).try_despawn();
                                }
                            }
                        }
                        if let Some((entity, t2)) = wall_q.iter().next() {
                            if collision(
                                &t.translation,
                                &Vec2::splat(*radius),
                                &t2.translation,
                                &WALL_SIZE,
                            ) {
                                if player.wall.health > damage.penetration {
                                    player.wall.health -= damage.penetration;
                                } else {
                                    player.wall.health = 0.;
                                    commands.entity(entity).try_despawn();
                                }
                            }
                        }

                        // Resolve the impact on all enemies in radius
                        enemy_q
                            .iter_mut()
                            .filter(|(_, &t2, enemy)| {
                                collision(
                                    &t.translation,
                                    &Vec2::splat(*radius),
                                    &t2.translation,
                                    &enemy.dim,
                                )
                            })
                            .for_each(|(enemy_e, _, mut enemy)| {
                                resolve_impact(
                                    &mut commands,
                                    enemy_e,
                                    &mut enemy,
                                    damage,
                                    &mut night_stats,
                                )
                            });
                    }
                } else if atlas.index == animation.last_index {
                    commands.entity(entity).try_despawn();
                }
            }
        }
    }
}

pub fn clear_map(
    mut commands: Commands,
    animation_q: Query<Entity, With<AnimationComponent>>,
    weapon_q: Query<Entity, With<Weapon>>,
    bullet_q: Query<Entity, With<Bullet>>,
    enemy_q: Query<Entity, With<Enemy>>,
) {
    animation_q
        .iter()
        .for_each(|a| commands.entity(a).try_despawn());
    weapon_q
        .iter()
        .for_each(|w| commands.entity(w).try_despawn());
    bullet_q
        .iter()
        .for_each(|b| commands.entity(b).try_despawn());
    enemy_q
        .iter()
        .for_each(|e| commands.entity(e).despawn_recursive());
}
