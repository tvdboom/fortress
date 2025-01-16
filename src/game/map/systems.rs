use super::components::*;
use crate::constants::*;
use crate::game::assets::WorldAssets;
use crate::game::enemy::components::{Enemy, EnemyHealth, EnemyManager, Size};
use crate::game::enemy::utils::get_future_position;
use crate::game::map::utils::{collision, is_visible, toggle, CustomUi};
use crate::game::resources::{GameSettings, NightStats, Player, Resources};
use crate::game::weapon::components::*;
use crate::game::{AppState, GameState};
use crate::messages::Messages;
use crate::utils::*;
use bevy::color::palettes::basic::WHITE;
use bevy::prelude::*;
use bevy_egui::egui::{Align, CursorIcon, Layout, RichText, Style, TextStyle, UiBuilder};
use bevy_egui::{egui, EguiContexts};
use std::f32::consts::PI;

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
                FOW_Z,
            ),
            Visibility::Hidden,
            PauseWrapper,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text2d::new("Paused".to_string()),
                TextColor(Color::from(WHITE)),
                TextLayout::new_with_justify(JustifyText::Center),
                Transform::from_xyz(0., 0., FOW_Z),
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
                        ui.add_enabled_ui(*app_state.get() == AppState::Day, |ui| {
                            if ui.button("Save game").clicked() {
                                todo!();
                            }
                        });
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
    mut game_settings: ResMut<GameSettings>,
    time: Res<Time>,
    assets: Local<WorldAssets>,
) {
    let day_texture = contexts.add_image(assets.get_image("day"));
    let night_texture = contexts.add_image(assets.get_image("night"));
    let soldier_texture = contexts.add_image(assets.get_image("soldier"));
    let population_texture = contexts.add_image(assets.get_image("population"));
    let wall_texture = contexts.add_image(assets.get_image("wall"));
    let fence_texture = contexts.add_image(assets.get_image("fence"));
    let bullets_texture = contexts.add_image(assets.get_image("bullets"));
    let gasoline_texture = contexts.add_image(assets.get_image("gasoline"));
    let materials_texture = contexts.add_image(assets.get_image("materials"));
    let technology_texture = contexts.add_image(assets.get_image("technology"));
    let hourglass_texture = contexts.add_image(assets.get_image("hourglass"));
    let clock_texture = contexts.add_image(assets.get_image("clock"));

    egui::TopBottomPanel::bottom("Resources")
        .exact_height(RESOURCES_PANEL_SIZE.y)
        .show(contexts.ctx_mut(), |ui| {
            ui.horizontal_centered(|ui| {
                match *app_state.get() {
                    AppState::Day => ui.add_image(day_texture, [20., 20.]).on_hover_text("Day"),
                    _ => ui
                        .add_image(night_texture, [20., 20.])
                        .on_hover_text("Night"),
                };
                ui.add(egui::Label::new(player.day.to_string()));

                ui.separator();

                if player.population.soldier > 0 {
                    ui.add_image(soldier_texture, [20., 20.])
                        .on_hover_text("Soldiers");
                    ui.add(egui::Label::new(player.population.soldier.to_string()));
                } else {
                    ui.add_image(population_texture, [20., 20.])
                        .on_hover_text("Population");
                    ui.add(egui::Label::new(player.population.total().to_string()));
                }

                ui.add_space(10.);

                ui.add_image(wall_texture, [20., 20.])
                    .on_hover_text("Fortress strength");
                ui.add(
                    egui::ProgressBar::new(player.wall.health / player.wall.max_health)
                        .desired_width(170.)
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
                    ui.add_space(10.);

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

                ui.separator();

                ui.add_image(bullets_texture, [10., 20.])
                    .on_hover_text("Bullets");
                ui.add(egui::Label::new(format!(
                    "{:.0} (+{})",
                    player.resources.bullets,
                    player.population.armorer * RESOURCE_FACTOR
                )));

                ui.add_space(10.);

                ui.add_image(gasoline_texture, [20., 20.])
                    .on_hover_text("Gasoline");
                ui.add(egui::Label::new(format!(
                    "{:.0} (+{})",
                    player.resources.gasoline,
                    player.population.refiner * RESOURCE_FACTOR
                )));

                ui.add_space(10.);

                ui.add_image(materials_texture, [20., 20.])
                    .on_hover_text("Materials");
                ui.add(egui::Label::new(format!(
                    "{:.0} (+{})",
                    player.resources.materials,
                    player.population.harvester * RESOURCE_FACTOR
                )));

                ui.add_space(10.);

                ui.add_image(technology_texture, [20., 20.])
                    .on_hover_text("Technology");
                ui.add(egui::Label::new(format!(
                    "{:.0} (+{})",
                    player.resources.technology,
                    player.population.scientist * RESOURCE_FACTOR
                )));

                ui.scope_builder(
                    UiBuilder {
                        invisible: *app_state.get() != AppState::Night,
                        ..default()
                    },
                    |ui| {
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

                        ui.add_space(10.);

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
    mut messages: ResMut<Messages>,
    mut night_stats: ResMut<NightStats>,
    app_state: Res<State<AppState>>,
    game_state: Res<State<GameState>>,
    weapons: Res<WeaponManager>,
    assets: Local<WorldAssets>,
    asset_server: Res<AssetServer>,
) {
    let weapon_texture = contexts.add_image(assets.get_image("weapon"));
    let lightning_texture = contexts.add_image(assets.get_image("lightning"));
    let fence_texture = contexts.add_image(assets.get_image("fence"));
    let mine_texture = contexts.add_image(assets.get_image("mine"));
    let bomb_texture = contexts.add_image(assets.get_image("bomb"));
    let nuke_texture = contexts.add_image(assets.get_image("nuke"));
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

                // Store the old settings to only update weapons that changed
                let old_s = player.weapons.settings.clone();

                // Machine gun
                if player.weapons.spots.iter().any(|w| match w {
                    Some(w) => *w == WeaponName::MachineGun,
                    None => false,
                }) {
                    ui.add_space(7.);
                    ui.horizontal(|ui| {
                        let label = ui.add(egui::Label::new(format!("{:?}: ", WeaponName::MachineGun))).on_hover_cursor(CursorIcon::PointingHand);

                        ui.add(egui::Slider::new(&mut player.weapons.settings.machine_gun, 0..=MAX_MACHINE_GUN_FIRE_RATE))
                            .on_hover_text("Shoot N bullets per second.");

                        if label.clicked() {
                            player.weapons.settings.machine_gun = if player.weapons.settings.machine_gun > 0 {
                                0
                            } else {
                                MAX_MACHINE_GUN_FIRE_RATE
                            };
                        }
                    });
                }

                // Canon
                if player.weapons.spots.iter().any(|w| match w {
                    Some(w) => *w == WeaponName::Canon,
                    None => false,
                }) {
                    ui.add_space(7.);
                    ui.horizontal(|ui| {
                        ui.add(egui::Label::new(format!("{:?}: ", WeaponName::Canon)));
                        ui.selectable_value(&mut player.weapons.settings.canon, AirFireStrategy::None, AirFireStrategy::None.name())
                            .on_hover_text("Don't fire.");
                        ui.selectable_value(&mut player.weapons.settings.canon, AirFireStrategy::Grounded, AirFireStrategy::Grounded.name())
                            .on_hover_text("Fire only at grounded (non-flying) enemies.");
                        ui.selectable_value(&mut player.weapons.settings.canon, AirFireStrategy::Airborne, AirFireStrategy::Airborne.name())
                            .on_hover_text("Fire only at flying enemies.");
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

                        ui.add(egui::Slider::new(&mut player.weapons.settings.flamethrower, 0..=MAX_FLAMETHROWER_POWER))
                            .on_hover_text("More power means more range, but costs more.");

                        if label.clicked() {
                            player.weapons.settings.flamethrower = if player.weapons.settings.flamethrower > 0 {
                                0
                            } else {
                                MAX_FLAMETHROWER_POWER
                            };
                        }
                    });
                }

                // Artillery
                if player.weapons.spots.iter().any(|w| match w {
                    Some(w) => *w == WeaponName::Artillery,
                    None => false,
                }) {
                    ui.add_space(7.);
                    ui.horizontal(|ui| {
                        ui.add(egui::Label::new(format!("{:?}: ", WeaponName::Artillery)));
                        ui.selectable_value(&mut player.weapons.settings.artillery, FireStrategy::None, FireStrategy::None.name())
                            .on_hover_text("Don't fire.");
                        ui.selectable_value(&mut player.weapons.settings.artillery, FireStrategy::Closest, FireStrategy::Closest.name())
                            .on_hover_text("Fire on the closest enemy.");
                        ui.selectable_value(&mut player.weapons.settings.artillery, FireStrategy::Strongest, FireStrategy::Strongest.name())
                            .on_hover_text("Fire on the strongest enemy.");
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
                        ui.selectable_value(&mut player.weapons.settings.aaa, AirFireStrategy::None, AirFireStrategy::None.name())
                            .on_hover_text("Don't fire.");
                        ui.selectable_value(&mut player.weapons.settings.aaa, AirFireStrategy::All, AirFireStrategy::All.name())
                            .on_hover_text("Fire at all enemies dealing reduced damage.");
                        ui.selectable_value(&mut player.weapons.settings.aaa, AirFireStrategy::Airborne, AirFireStrategy::Airborne.name())
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
                        ui.selectable_value(&mut player.weapons.settings.mortar, MortarShell::None, MortarShell::None.name())
                            .on_hover_text("Don't fire.");
                        ui.selectable_value(&mut player.weapons.settings.mortar, MortarShell::Light, MortarShell::Light.name())
                            .on_hover_text("Light shells do standard damage and don't damage structures.");
                        ui.selectable_value(&mut player.weapons.settings.mortar, MortarShell::Heavy, MortarShell::Heavy.name())
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
                        ui.add(
                            egui::ProgressBar::new(player.weapons.settings.turret / MAX_TURRET_POWER)
                                .desired_width(120.)
                                .desired_height(20.)
                                .show_percentage()
                        );

                        let locked = weapon_q.iter().any(|w| w.name == WeaponName::Turret && w.fire_strategy == FireStrategy::Strongest);
                        ui.add_enabled_ui(!locked && player.weapons.settings.turret > MAX_TURRET_POWER * 0.2 && *game_state.get() == GameState::Running, |ui| {
                            let button = ui.add_sized([50., 20.], egui::Button::new("Fire!"));

                            if button.clicked() {
                                if let Some(mut turret) = weapon_q.iter_mut().find(|w| w.name == WeaponName::Turret) {
                                    turret.fire_strategy = FireStrategy::Strongest;
                                }
                            }
                        });
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

                        ui.add(egui::Slider::new(&mut player.weapons.settings.missile_launcher, 0..=MAX_MISSILE_LAUNCHER_SHELLS))
                            .on_hover_text("Shoot N shells per firing round.");

                        if label.clicked() {
                            player.weapons.settings.missile_launcher = if player.weapons.settings.missile_launcher > 0 {
                                0
                            } else {
                                MAX_MISSILE_LAUNCHER_SHELLS
                            };
                        }
                    });
                }

                // Update weapons with the changed settings
                weapon_q
                    .iter_mut()
                    .filter(|w| match w.name {
                        WeaponName::AAA => old_s.aaa != player.weapons.settings.aaa,
                        WeaponName::Artillery => old_s.artillery != player.weapons.settings.artillery,
                        WeaponName::Canon => old_s.canon != player.weapons.settings.canon,
                        WeaponName::MachineGun => old_s.machine_gun != player.weapons.settings.machine_gun,
                        WeaponName::Flamethrower => old_s.flamethrower != player.weapons.settings.flamethrower,
                        WeaponName::Mortar => old_s.mortar != player.weapons.settings.mortar,
                        WeaponName::Turret => old_s.turret != player.weapons.settings.turret,
                        WeaponName::MissileLauncher => old_s.missile_launcher != player.weapons.settings.missile_launcher,
                    })
                    .for_each(|mut w| w.as_mut().update(&player));

                ui.add_space(7.);

                if player.weapons.bombs > 0 || player.weapons.mines > 0 {
                    ui.separator();

                    ui.add_enabled_ui(player.weapons.mines > 0, |ui| {
                        ui.add_space(7.);
                        ui.horizontal(|ui| {
                            ui.add_image(mine_texture, [20., 20.]);
                            ui.add(egui::Label::new(format!("Mine ({}): ", player.weapons.mines)));
                            ui.selectable_value(&mut player.weapons.settings.mine, Size::Small, Size::Small.name())
                                .on_hover_text("Detonate for all enemies.");
                            ui.selectable_value(&mut player.weapons.settings.mine, Size::Medium, Size::Medium.name())
                                .on_hover_text("Detonate for medium and large enemies.");
                            ui.selectable_value(&mut player.weapons.settings.mine, Size::Large, Size::Large.name())
                                .on_hover_text("Detonate only for large enemies.");
                        });
                    });

                    ui.add_enabled_ui(player.weapons.bombs > 0, |ui| {
                        ui.add_space(7.);
                        ui.horizontal(|ui| {
                            ui.add_image(bomb_texture, [20., 20.]);
                            let label = ui.add_enabled_ui(*game_state.get() == GameState::Running, |ui| {
                                ui.add(egui::Label::new(format!("Bomb ({}): ", player.weapons.bombs)))
                                    .on_hover_cursor(CursorIcon::PointingHand)
                                    .on_hover_text("Launch!");
                            });

                            ui.selectable_value(&mut player.weapons.settings.bomb, FireStrategy::Density, FireStrategy::Density.name())
                                .on_hover_text("Launch at highest enemy density location.");
                            ui.selectable_value(&mut player.weapons.settings.bomb, FireStrategy::Strongest, FireStrategy::Strongest.name())
                                .on_hover_text("Launch at strongest enemy.");

                            if label.response.clicked() {
                                let mut bomb = weapons.bomb.clone();

                                let explosion = match &bomb.impact {
                                    Impact::Explosion(e) => e,
                                    _ => unreachable!(),
                                };

                                let visible_enemies = enemy_q
                                    .iter()
                                    .filter(|(_, enemy_t, enemy)| is_visible(fow_q.get_single().unwrap(), enemy_t, enemy))
                                    .collect::<Vec<_>>();

                                if let Some((_, enemy_t, enemy)) = match player.weapons.settings.bomb {
                                    FireStrategy::Strongest => {
                                        visible_enemies
                                            .iter()
                                            .max_by(|(_, _, e1), (_, _, e2)| {
                                                e1.max_health.partial_cmp(&e2.max_health).unwrap()
                                            })
                                    },
                                    FireStrategy::Density => {
                                        visible_enemies
                                            .iter()
                                            .max_by(|(_, t1, _), (_, t2, _)| {
                                                let density_a = visible_enemies
                                                    .iter()
                                                    .filter(|(_, t, _)| {
                                                        t1.translation.distance(t.translation) <= explosion.radius
                                                    })
                                                    .count();

                                                let density_b = visible_enemies
                                                    .iter()
                                                    .filter(|(_, t, _)| {
                                                        t2.translation.distance(t.translation) <= explosion.radius
                                                    })
                                                    .count();

                                                density_a.cmp(&density_b)
                                            })
                                    },
                                    _ => unreachable!(),
                                }
                                {
                                    let start = Vec3::new(enemy_t.translation.x, SIZE.y * 0.5, WEAPON_Z);

                                    // Calculate the detonation's position
                                    bomb.movement = Movement::Location(if player.technology.movement_prediction {
                                        get_future_position(
                                            enemy_t.translation,
                                            enemy.speed,
                                            start,
                                            bomb.speed,
                                            fence_q.get_single(),
                                            wall_q.get_single(),
                                        )
                                    } else {
                                        enemy_t.translation
                                    });

                                    commands.spawn((
                                        Sprite {
                                            image: asset_server.load(bomb.image),
                                            custom_size: Some(bomb.dim),
                                            ..default()
                                        },
                                        Transform {
                                            translation: start,
                                            rotation: Quat::from_rotation_z(-PI * 0.5),
                                            ..default()
                                        },
                                        bomb,
                                    ));

                                    player.weapons.bombs -= 1;
                                }
                            }
                        });
                    });

                    ui.add_space(7.);

                    ui.add_enabled_ui(player.weapons.nuke > 0 && *game_state.get() == GameState::Running, |ui| {
                        ui.add_space(7.);
                        ui.horizontal(|ui| {
                            ui.add_image(nuke_texture, [20., 20.]);
                            ui.add(egui::Label::new(format!("Nuke ({}): ", player.weapons.nuke)));
                            let button = ui.add_sized([60., 20.], egui::Button::new("Launch!"));

                            if button.clicked() {
                                let nuke = weapons.nuke.clone();
                                messages.info("A nuke is launched");

                                commands.spawn((
                                    Sprite {
                                        image: asset_server.load(nuke.image),
                                        custom_size: Some(nuke.dim),
                                        ..default()
                                    },
                                    Transform {
                                        translation: Vec3::new(-WEAPONS_PANEL_SIZE.x * 0.5, SIZE.y * 0.5, WEAPON_Z),
                                        rotation: Quat::from_rotation_z(-PI * 0.5),
                                        ..default()
                                    },
                                    nuke,
                                ));

                                player.weapons.nuke -= 1;
                            }
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
                            let cost = Resources {
                                bullets: 100.,
                                materials: -300.,
                                ..default()
                            };
                            player.resources += &cost;
                            night_stats.resources += &cost;
                            night_stats.warnings.no_bullets = false;
                        }
                    });

                    ui.add_enabled_ui(player.resources.materials >= 1500., |ui| {
                        let bullet_button_500 = ui.add_sized([30., 30.], egui::Button::new("+500"))
                            .on_hover_text("Buy 500 bullets for 1500 materials.");
                        if bullet_button_500.clicked() {
                            let cost = Resources {
                                bullets: 500.,
                                materials: -1500.,
                                ..default()
                            };
                            player.resources += &cost;
                            night_stats.resources += &cost;
                            night_stats.warnings.no_bullets = false;

                        }
                    });

                    ui.add_space(13.);

                    ui.add_image(gasoline_texture, [20., 20.]);
                    ui.add_enabled_ui(player.resources.materials >= 300., |ui| {
                        let gasoline_button = ui.add_sized([30., 30.], egui::Button::new("+100"))
                            .on_hover_text("Buy 100 gasoline for 300 materials.");
                        if gasoline_button.clicked() {
                            let cost = Resources {
                                gasoline: 100.,
                                materials: -300.,
                                ..default()
                            };

                            player.resources += &cost;
                            night_stats.resources += &cost;
                            night_stats.warnings.no_gasoline = false;
                        }
                    });

                    ui.add_enabled_ui(player.resources.materials >= 1500., |ui| {
                        let gasoline_button_500 = ui.add_sized([30., 30.], egui::Button::new("+500"))
                            .on_hover_text("Buy 500 gasoline for 1500 materials.");
                        if gasoline_button_500.clicked() {
                            let cost = Resources {
                                gasoline: 500.,
                                materials: -1500.,
                                ..default()
                            };

                            player.resources += &cost;
                            night_stats.resources += &cost;
                            night_stats.warnings.no_gasoline = false;
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
                                                "The world has been conquered by insects. Together with a handful of survivors, \
                                                you have built a fortress to defend yourself from their ferocious attacks. \
                                                Every night, an ever-increasing swarm of attacks the fortress. Kill them before \
                                                they enter the fortress and kill the remaining population!\n\n \
                                                During the day, you can collect resources and upgrade your weapon arsenal to \
                                                prepare yourself for the following night. During the attack, you can choose \
                                                how/when to use the weapons you have at your disposal. But be careful, everything \
                                                has a cost! Manage your resources wisely or you won't be able to stop the insects \
                                                tomorrow...");
                                            ui.add_space(5.);
                                        })
                                })
                            });

                        ui.add_space(15.);

                        if ui.add_button("Start game").clicked() {
                            next_state.set(AppState::Night);
                        }
                    },
                    AppState::Day => {
                        ui.heading(format!("You survived night {}!", player.day));

                        ui.add_space(15.);

                        ui.label(
                            "The day has finally arrived. The sun is rising and the bugs \
                            are retreating. Upgrade your weapons and prepare for tonight...");

                        egui::ScrollArea::vertical()
                            .max_width(SIZE.x * 0.4)
                            .show(ui, |ui| {
                                ui.add_night_stats(&player);
                            });

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
                            ui.add_space(215.);

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
                ui.add_space(25.);

                egui::ScrollArea::vertical()
                    .max_width(MAP_SIZE.x * 0.25)
                    .show(ui, |ui| {
                        enemies.list.iter().enumerate().for_each(|(i, e)| {
                            if i > 0 {
                                ui.add_space(25.);
                            }

                            ui.horizontal(|ui| {
                                ui.add_space(30.);
                                ui.add_image(*textures.get(i).unwrap(), [105., 120.])
                                    .on_hover_text(e.name);

                                ui.add_space(20.);

                                ui.vertical(|ui| {
                                    ui.label(RichText::new(e.name).strong());
                                    ui.label(format!("Size: {:?}", e.size));
                                    ui.label(format!("Health: {}", e.health));
                                    ui.label(format!("Armor: {}", e.armor))
                                        .on_hover_text("Armor reduces incoming damage.");
                                    ui.label(format!("Speed: {:.0}", e.speed)).on_hover_text(
                                        "As percentage of the map's height per second.",
                                    );
                                    ui.label(format!("Can fly: {}", e.can_fly))
                                        .on_hover_text("Flying bugs can pass over constructions.");
                                    ui.label(format!("Damage: {}", e.damage)).on_hover_text(
                                        "Damage dealt to constructions or population.",
                                    );
                                });

                                ui.add_space(25.);
                            });
                        });
                    });

                ui.add_space(25.);
            });
    }
}

pub fn run_animations(
    mut commands: Commands,
    mut animation_q: Query<(Entity, &Transform, &mut AnimationComponent, &mut Sprite)>,
    mut enemy_q: Query<(Entity, &Transform, &mut Enemy)>,
    fence_q: Query<&Transform, With<Fence>>,
    wall_q: Query<&Transform, With<Wall>>,
    mut player: ResMut<Player>,
    game_settings: Res<GameSettings>,
    time: Res<Time>,
) {
    for (animation_e, animation_t, mut animation, mut sprite) in animation_q.iter_mut() {
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
                        if let Some(fence_t) = fence_q.iter().next() {
                            if collision(
                                &animation_t.translation,
                                &Vec2::splat(2. * radius),
                                &fence_t.translation,
                                &FENCE_SIZE,
                            ) {
                                player.fence.health -= damage.penetration.min(player.fence.health);
                            }
                        }
                        if let Some(wall_t) = wall_q.iter().next() {
                            if collision(
                                &animation_t.translation,
                                &Vec2::splat(2. * radius),
                                &wall_t.translation,
                                &WALL_SIZE,
                            ) {
                                player.wall.health -= damage.penetration.min(player.wall.health);
                            }
                        }

                        // Resolve the impact on all enemies in radius
                        enemy_q
                            .iter_mut()
                            .filter(|(_, &t2, enemy)| {
                                collision(
                                    &animation_t.translation,
                                    &Vec2::splat(2. * radius),
                                    &t2.translation,
                                    &enemy.dim,
                                )
                            })
                            .for_each(|(_, _, ref mut enemy)| {
                                enemy.health -= damage.calculate(enemy).min(enemy.health)
                            });
                    }
                } else if atlas.index == animation.last_index {
                    commands.entity(animation_e).try_despawn();
                }
            }
        }
    }
}

pub fn update_game(
    mut commands: Commands,
    weapon_q: Query<&Weapon>,
    enemy_q: Query<EnemyQ, (With<Enemy>, Without<EnemyHealth>)>,
    children_q: Query<&Children>,
    fence_q: Query<Entity, With<Fence>>,
    wall_q: Query<Entity, With<Wall>>,
    mut health_q: Query<(&mut Transform, &mut Sprite), With<EnemyHealth>>,
    mut player: ResMut<Player>,
    mut night_stats: ResMut<NightStats>,
    mut messages: ResMut<Messages>,
    game_settings: Res<GameSettings>,
    time: Res<Time>,
) {
    // Update turret's power
    if let Some(turret) = weapon_q.iter().find(|w| w.name == WeaponName::Turret) {
        if player.weapons.settings.turret < MAX_TURRET_POWER {
            let cost = TURRET_POWER_COST * game_settings.speed * time.delta_secs();
            if player.resources.gasoline > cost {
                player.resources.gasoline -= cost;
                night_stats.resources.gasoline += cost;

                // The default is to power-up in 10 seconds, but
                // this decreases with the fire_timer's duration
                let timer = turret.fire_timer.clone().unwrap().duration().as_secs_f32();
                player.weapons.settings.turret += MAX_TURRET_POWER / DEFAULT_TURRET_POWER_TIME
                    * timer.powf(-1.)
                    * game_settings.speed
                    * time.delta_secs()
            }
        }
    }

    // Update resources
    if player.fence.enabled {
        let fence_cost = &player.fence.cost * game_settings.speed * time.delta_secs();
        if player.resources >= fence_cost {
            player.resources -= &fence_cost;
            night_stats.resources += &fence_cost;
        } else {
            player.fence.enabled = false;
        }
    }

    let spotlight_cost = &player.spotlight.cost
        * player.spotlight.power as f32
        * game_settings.speed
        * time.delta_secs();

    if player.resources >= spotlight_cost {
        player.resources -= &spotlight_cost;
        night_stats.resources += &spotlight_cost;
    } else {
        player.spotlight.power = 0;
    }

    // Warn on low resources
    if player.resources.bullets < 200. * player.day as f32 && !night_stats.warnings.low_bullets {
        messages.warning("Low bullets");
        night_stats.warnings.low_bullets = true;
    }
    if player.resources.gasoline < 200. * player.day as f32 && !night_stats.warnings.low_gasoline {
        messages.warning("Low gasoline");
        night_stats.warnings.low_gasoline = true;
    }
    if player.resources.bullets < 5. && !night_stats.warnings.no_bullets {
        messages.error("No bullets");
        night_stats.warnings.no_bullets = true;
    }
    if player.resources.gasoline < 5. && !night_stats.warnings.no_gasoline {
        messages.error("No gasoline");
        night_stats.warnings.no_gasoline = true;
    }

    // Despawn structures
    if let Ok(fence_e) = fence_q.get_single() {
        if player.fence.health == 0. {
            messages.warning("The fence is broken");
            commands.entity(fence_e).try_despawn();
        }
    }

    if let Ok(wall_e) = wall_q.get_single() {
        if player.wall.health == 0. {
            messages.error("The wall is broken");
            commands.entity(wall_e).try_despawn();
        }
    }

    // Update enemy health bars and despawn enemies
    for (enemy_e, _, enemy) in enemy_q.iter() {
        if enemy.health < enemy.max_health {
            if enemy.health == 0. {
                commands.entity(enemy_e).despawn_recursive();

                night_stats
                    .enemies
                    .entry(enemy.name)
                    .and_modify(|status| status.killed += 1);
            } else {
                for child in children_q.iter_descendants(enemy_e) {
                    if let Ok((mut sprite_t, mut sprite)) = health_q.get_mut(child) {
                        if let Some(size) = sprite.custom_size.as_mut() {
                            let full_size = enemy.dim.x * 0.8 - 2.0;
                            size.x = full_size * enemy.health / enemy.max_health;
                            sprite_t.translation.x = (size.x - full_size) * 0.5;
                        }
                    }
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
