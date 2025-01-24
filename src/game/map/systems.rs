use super::components::*;
use crate::constants::*;
use crate::game::assets::WorldAssets;
use crate::game::enemy::components::{Enemy, EnemyHealth, EnemyManager, Size};
use crate::game::enemy::utils::get_future_position;
use crate::game::map::utils::{collision, is_visible, toggle, CustomUi};
use crate::game::resources::*;
use crate::game::weapon::components::*;
use crate::game::weapon::systems::{spawn_fence, spawn_spots, spawn_wall};
use crate::game::{AppState, AudioState, GameState};
use crate::messages::Messages;
use crate::utils::*;
use bevy::color::palettes::basic::WHITE;
use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;
use bevy_egui::egui::{
    Align, Color32, CursorIcon, Layout, RichText, ScrollArea, Style, TextStyle, TextureId,
    UiBuilder,
};
use bevy_egui::{egui, EguiContexts};
use egui_dnd::dnd;
use std::f32::consts::PI;
use strum::IntoEnumIterator;
use uuid::Uuid;

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

    // Background on top and bottom of the actual map to hide fow and (de)spawning enemies
    commands.spawn((
        Sprite {
            image: asset_server.load("map/bg.png"),
            custom_size: Some(SIZE),
            ..default()
        },
        Transform::from_xyz(0., SIZE.y - MENU_PANEL_SIZE.y, 10.),
        Map,
    ));
    commands.spawn((
        Sprite {
            image: asset_server.load("map/bg.png"),
            custom_size: Some(SIZE),
            ..default()
        },
        Transform::from_xyz(0., -SIZE.y + RESOURCES_PANEL_SIZE.y, 10.),
        Map,
    ));

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
    mut commands: Commands,
    mut contexts: EguiContexts,
    player: Res<Player>,
    mut game_settings: ResMut<GameSettings>,
    mut messages: ResMut<Messages>,
    app_state: Res<State<AppState>>,
    game_state: Res<State<GameState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_audio_state: ResMut<NextState<AudioState>>,
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
                            load_game(
                                &mut commands,
                                &game_settings,
                                &mut next_app_state,
                                &mut messages,
                            );
                            ui.close_menu();
                        }
                        ui.add_enabled_ui(*app_state.get() == AppState::Day, |ui| {
                            if ui.button("Save game").clicked() {
                                save_game(&player, &game_settings, &mut messages);
                                ui.close_menu();
                            }
                        });
                        if ui.button("Quit").clicked() {
                            std::process::exit(0);
                        }
                    });
                    egui::menu::menu_button(ui, "View", |ui| {
                        if ui.button("Enemy info").clicked() {
                            game_settings.enemy_info = !game_settings.enemy_info;
                            ui.close_menu();
                        }
                    });
                    egui::menu::menu_button(ui, "Settings", |ui| {
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

                        if ui.button("Toggle audio").clicked() {
                            match game_settings.audio {
                                true => next_audio_state.set(AudioState::Stopped),
                                false => next_audio_state.set(AudioState::Playing),
                            }
                        }
                    });
                });
            });
        });
}

pub fn resources_panel(
    mut contexts: EguiContexts,
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
                        .desired_width(220.)
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
                            .desired_width(170.)
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

                ui.add_image(bullets_texture, [20., 20.])
                    .on_hover_text("Bullets");
                ui.add(egui::Label::new(
                    format!("{:.0}", player.resources.bullets,),
                ));

                ui.add_space(10.);

                ui.add_image(gasoline_texture, [20., 20.])
                    .on_hover_text("Gasoline");
                ui.add(egui::Label::new(format!(
                    "{:.0}",
                    player.resources.gasoline,
                )));

                ui.add_space(10.);

                ui.add_image(materials_texture, [20., 20.])
                    .on_hover_text("Materials");
                ui.add(egui::Label::new(format!(
                    "{:.0}",
                    player.resources.materials,
                )));

                ui.add_space(10.);

                ui.add_image(technology_texture, [20., 20.])
                    .on_hover_text("Technology");
                ui.add(egui::Label::new(format!(
                    "{:.0}",
                    player.resources.technology,
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
    fence_q: Query<SpriteQ, (With<FenceComponent>, Without<FogOfWar>)>,
    wall_q: Query<SpriteQ, (With<WallComponent>, Without<FogOfWar>)>,
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
                if player.weapons.spots.iter().any(|w| match w.weapon {
                    Some(w) => w == WeaponName::MachineGun,
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
                if player.weapons.spots.iter().any(|w| match w.weapon {
                    Some(w) => w == WeaponName::Canon,
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
                if player.weapons.spots.iter().any(|w| match w.weapon {
                    Some(w) => w == WeaponName::Flamethrower,
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
                if player.weapons.spots.iter().any(|w| match w.weapon {
                    Some(w) => w == WeaponName::Artillery,
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
                if player.weapons.spots.iter().any(|w| match w.weapon {
                    Some(w) => w == WeaponName::AAA,
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
                if player.weapons.spots.iter().any(|w| match w.weapon {
                    Some(w) => w == WeaponName::Mortar,
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
                if player.weapons.spots.iter().any(|w| match w.weapon {
                    Some(w) => w == WeaponName::Turret,
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
                if player.weapons.spots.iter().any(|w| match w.weapon {
                    Some(w) => w == WeaponName::MissileLauncher,
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

                if player.weapons.mines > 0 || player.weapons.bombs > 0 {
                    ui.separator();

                    ui.add_enabled_ui(player.weapons.mines > 0, |ui| {
                        ui.add_space(7.);
                        ui.horizontal(|ui| {
                            ui.add_image(mine_texture, [20., 15.]);
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
                                    .on_hover_text("Launch!")
                            });

                            ui.selectable_value(&mut player.weapons.settings.bomb, FireStrategy::Density, FireStrategy::Density.name())
                                .on_hover_text("Launch at highest enemy density location.");
                            ui.selectable_value(&mut player.weapons.settings.bomb, FireStrategy::Strongest, FireStrategy::Strongest.name())
                                .on_hover_text("Launch at strongest enemy.");

                            if label.inner.clicked() {
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
                                    bomb.movement = Movement::Location(if player.has_tech(TechnologyName::Aimbot) {
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

                if (player.fence.max_health > 0. && player.has_tech(TechnologyName::Electricity)) || player.has_tech(TechnologyName::Spotlight) {
                    ui.separator();

                    if player.fence.max_health > 0. && player.has_tech(TechnologyName::Electricity) {
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

                    if player.has_tech(TechnologyName::Spotlight) {
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

pub fn day_panel(
    mut commands: Commands,
    fence_q: Query<SpriteQ, With<FenceComponent>>,
    wall_q: Query<SpriteQ, With<WallComponent>>,
    weapon_q: Query<Entity, With<Weapon>>,
    mut contexts: EguiContexts,
    mut player: ResMut<Player>,
    mut weapons: ResMut<WeaponManager>,
    mut messages: ResMut<Messages>,
    mut game_settings: ResMut<GameSettings>,
    mut next_state: ResMut<NextState<AppState>>,
    assets: Local<WorldAssets>,
    asset_server: Res<AssetServer>,
    window: Query<&Window>,
) {
    let window_size = window.single().size();

    let population_texture = contexts.add_image(assets.get_image("population"));
    let soldier_texture = contexts.add_image(assets.get_image("soldier"));
    let combat_texture = contexts.add_image(assets.get_image("combat"));
    let armorer_texture = contexts.add_image(assets.get_image("armorer"));
    let bullets_texture = contexts.add_image(assets.get_image("bullets"));
    let refiner_texture = contexts.add_image(assets.get_image("refiner"));
    let gasoline_texture = contexts.add_image(assets.get_image("gasoline"));
    let constructor_texture = contexts.add_image(assets.get_image("constructor"));
    let materials_texture = contexts.add_image(assets.get_image("materials"));
    let scientist_texture = contexts.add_image(assets.get_image("scientist"));
    let technology_texture = contexts.add_image(assets.get_image("technology"));
    let up_texture = contexts.add_image(assets.get_image("up-arrow"));
    let repair_texture = contexts.add_image(assets.get_image("repair"));
    let spots_texture = contexts.add_image(assets.get_image("spots"));
    let damage_texture = contexts.add_image(assets.get_image("damage"));
    let explosion_texture = contexts.add_image(assets.get_image("explosion"));
    let range_texture = contexts.add_image(assets.get_image("range"));
    let reload_texture = contexts.add_image(assets.get_image("reload"));
    let penetration_texture = contexts.add_image(assets.get_image("penetration"));
    let targets_texture = contexts.add_image(assets.get_image("targets"));
    let tick_texture = contexts.add_image(assets.get_image("tick"));
    let cross_texture = contexts.add_image(assets.get_image("cross"));
    let idle_texture = contexts.add_image(assets.get_image("idle"));
    let clock_texture = contexts.add_image(assets.get_image("clock"));
    let armory_texture = contexts.add_image(assets.get_image("armory"));
    let refinery_texture = contexts.add_image(assets.get_image("refinery"));
    let factory_texture = contexts.add_image(assets.get_image("factory"));
    let laboratory_texture = contexts.add_image(assets.get_image("laboratory"));
    let wall_texture = contexts.add_image(assets.get_image("wall-shop"));
    let fence_texture = contexts.add_image(assets.get_image("fence-shop"));
    let lightning_texture = contexts.add_image(assets.get_image("lightning"));
    let aaa_texture = contexts.add_image(assets.get_image("aaa"));
    let artillery_texture = contexts.add_image(assets.get_image("artillery"));
    let canon_texture = contexts.add_image(assets.get_image("canon"));
    let flamethrower_texture = contexts.add_image(assets.get_image("flamethrower"));
    let machine_gun_texture = contexts.add_image(assets.get_image("machine-gun"));
    let missile_launcher_texture = contexts.add_image(assets.get_image("ml"));
    let mortar_texture = contexts.add_image(assets.get_image("mortar"));
    let turret_texture = contexts.add_image(assets.get_image("turret"));
    let mine_texture = contexts.add_image(assets.get_image("mine-shop"));
    let bomb_texture = contexts.add_image(assets.get_image("bomb-shop"));
    let nuke_texture = contexts.add_image(assets.get_image("nuke-shop"));

    egui::Window::new("info panel")
        .title_bar(false)
        .fixed_size((MAP_SIZE.x * 0.6, MAP_SIZE.y * 0.7))
        .fixed_pos((
            (window_size.x - WEAPONS_PANEL_SIZE.x) * 0.5 - MAP_SIZE.x * 0.3,
            (window_size.y - RESOURCES_PANEL_SIZE.y) * 0.5 - MAP_SIZE.y * 0.4,
        ))
        .show(contexts.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut game_settings.day_tab,
                    DayTabs::Overview,
                    DayTabs::Overview.name(),
                );
                ui.add_space(5.);
                ui.selectable_value(
                    &mut game_settings.day_tab,
                    DayTabs::Population,
                    DayTabs::Population.name(),
                );
                ui.add_space(5.);
                ui.selectable_value(
                    &mut game_settings.day_tab,
                    DayTabs::Constructions,
                    DayTabs::Constructions.name(),
                );
                ui.add_space(5.);
                ui.selectable_value(
                    &mut game_settings.day_tab,
                    DayTabs::Armory,
                    DayTabs::Armory.name(),
                );
                ui.add_space(5.);
                ui.selectable_value(
                    &mut game_settings.day_tab,
                    DayTabs::Technology,
                    DayTabs::Technology.name(),
                );
                if player.has_tech(TechnologyName::Charts) {
                    ui.add_space(5.);
                    ui.selectable_value(
                        &mut game_settings.day_tab,
                        DayTabs::Expeditions,
                        DayTabs::Expeditions.name(),
                    );
                }
            });

            ui.separator();
            ui.add_space(10.);

            let new_resources = player.new_resources();

            match game_settings.day_tab {
                DayTabs::Overview => {
                    ui.vertical_centered(|ui| {
                        ui.add_space(20.);
                        ui.heading(format!("You survived night {}!", player.day - 1));
                        ui.add_space(20.);
                    });

                    ui.add_scroll("overview", MAP_SIZE.x * 0.1, |ui| {
                        ui.label(
                            "The day has finally arrived. The sun is rising and the bugs \
                            are retreating. Upgrade your weapons and prepare for tonight...",
                        );
                    });

                    ui.add_night_stats(&player, player.day - 1);

                    ui.with_layout(Layout::right_to_left(Align::RIGHT), |ui| {
                        ui.add_space(20.);

                        if ui
                            .add_button(format!("   Continue to night {}   ", player.day))
                            .clicked()
                        {
                            if player.population.idle == 0 {
                                player.resolve_expedition();
                                next_state.set(AppState::Night);
                            } else {
                                messages.error("You have idle population!");
                            }
                        }
                    });
                }
                DayTabs::Population => {
                    ui.add_scroll("res1", MAP_SIZE.x * 0.1, |ui| {
                        ui.add_space(15.);
                        ui.label(
                            "Distribute the population over the resources. The number of \
                            workers on a specific trait determines the amount of that resource \
                            you will get tomorrow. Make sure to not let anyone idle!",
                        );
                        ui.add_space(15.);
                    });

                    ui.add_scroll("res2", MAP_SIZE.x * 0.125, |ui| {
                        let soldiers = ui
                            .horizontal_centered(|ui| {
                                ui.add_image(soldier_texture, [40., 40.]);
                                let label = ui
                                    .add_text("Soldiers", 100.)
                                    .on_hover_cursor(CursorIcon::PointingHand);
                                let mut soldiers = player.population.soldier.clone();
                                ui.add(egui::Slider::new(
                                    &mut soldiers,
                                    0..=player.population.soldier + player.population.idle,
                                ))
                                .on_hover_text("Assign the population to produce bullets.");
                                ui.add_space(10.);
                                ui.add_image(combat_texture, [20., 20.]).on_hover_text("Combat strength.");
                                ui.label(format!("x{}", player.get_soldier_damage()));

                                if label.clicked() {
                                    soldiers = player.population.soldier + player.population.idle;
                                }

                                soldiers
                            })
                            .inner;

                        ui.add_space(15.);

                        let armorers = ui
                            .horizontal_centered(|ui| {
                                ui.add_image(armorer_texture, [40., 40.]);
                                let label = ui
                                    .add_text("Armorers", 100.)
                                    .on_hover_cursor(CursorIcon::PointingHand);
                                let mut armorers = player.population.armorer.clone();
                                ui.add(egui::Slider::new(
                                    &mut armorers,
                                    0..=player.population.armorer + player.population.idle,
                                ))
                                .on_hover_text("Assign the population to produce bullets.");
                                ui.add_space(10.);
                                ui.add_image(bullets_texture, [20., 20.]);
                                ui.label(format!("+{}", new_resources.bullets));

                                if label.clicked() {
                                    armorers = player.population.armorer + player.population.idle;
                                }

                                armorers
                            })
                            .inner;

                        ui.add_space(15.);

                        let refiners = ui
                            .horizontal_centered(|ui| {
                                ui.add_image(refiner_texture, [40., 40.]);
                                let label = ui
                                    .add_text("Refiners", 100.)
                                    .on_hover_cursor(CursorIcon::PointingHand);
                                let mut refiners = player.population.refiner.clone();
                                ui.add(egui::Slider::new(
                                    &mut refiners,
                                    0..=player.population.refiner + player.population.idle,
                                ))
                                .on_hover_text("Assign the population to produce gasoline.");
                                ui.add_space(10.);
                                ui.add_image(gasoline_texture, [20., 20.]);
                                ui.label(format!("+{}", new_resources.gasoline));

                                if label.clicked() {
                                    refiners = player.population.refiner + player.population.idle;
                                }

                                refiners
                            })
                            .inner;

                        ui.add_space(15.);

                        let constructors = ui
                            .horizontal_centered(|ui| {
                                ui.add_image(constructor_texture, [40., 40.]);
                                let label = ui
                                    .add_text("Constructors", 100.)
                                    .on_hover_cursor(CursorIcon::PointingHand);
                                let mut constructors = player.population.constructor.clone();
                                ui.add(egui::Slider::new(
                                    &mut constructors,
                                    0..=player.population.constructor + player.population.idle,
                                ))
                                .on_hover_text("Assign the population to produce materials.");
                                ui.add_space(10.);
                                ui.add_image(materials_texture, [20., 20.]);
                                ui.label(format!("+{}", new_resources.materials));

                                if label.clicked() {
                                    constructors =
                                        player.population.constructor + player.population.idle;
                                }

                                constructors
                            })
                            .inner;

                        ui.add_space(15.);

                        let scientists = ui
                            .horizontal_centered(|ui| {
                                ui.add_image(scientist_texture, [40., 40.]);
                                let label = ui
                                    .add_text("Scientists", 100.)
                                    .on_hover_cursor(CursorIcon::PointingHand);
                                let mut scientists = player.population.scientist.clone();
                                ui.add(egui::Slider::new(
                                    &mut scientists,
                                    0..=player.population.scientist + player.population.idle,
                                ))
                                .on_hover_text("Assign the population to research technology.");
                                ui.add_space(10.);
                                ui.add_image(technology_texture, [20., 20.]);
                                ui.label(format!("+{}", new_resources.technology));

                                if label.clicked() {
                                    scientists =
                                        player.population.scientist + player.population.idle;
                                }

                                scientists
                            })
                            .inner;

                        ui.add_space(15.);

                        ui.horizontal_centered(|ui| {
                            ui.add_image(idle_texture, [40., 40.]);
                            ui.add_text("Idle", 100.);
                            ui.strong(format!("{}", player.population.idle));
                        });

                        //Resolve population choices
                        if soldiers + armorers + refiners + constructors + scientists
                            <= player.population.total()
                        {
                            player.population = Population {
                                soldier: soldiers,
                                armorer: armorers,
                                refiner: refiners,
                                constructor: constructors,
                                scientist: scientists,
                                idle: player.population.total()
                                    - soldiers
                                    - armorers
                                    - refiners
                                    - constructors
                                    - scientists,
                            };
                        }
                    });
                }
                DayTabs::Constructions => {
                    ui.add_space(5.);
                    ui.horizontal(|ui| {
                        ui.add_space(20.);
                        ui.heading("Resources");
                    });
                    ui.add_space(15.);

                    let frame = egui::Frame::none()
                        .fill(Color32::from_black_alpha(190))
                        .rounding(5.0)
                        .inner_margin(egui::vec2(10., 5.));

                    ui.horizontal(|ui| {
                        ui.add_space(30.);
                        ui.add_image(armory_texture, [130., 130.]);
                        ui.add_space(20.);
                        ui.vertical(|ui| {
                            let cost = ((player.constructions.armory + 1) * 100) as f32;

                            ui.strong("Armory");
                            ui.label(format!("Level: {}", player.constructions.armory));
                            ui.label(format!("Armorers: {}", player.population.armorer));
                            ui.label(format!("Production: +{:.0}", new_resources.bullets)).on_hover_text("Production of bullets per night.");
                            ui.add_space(10.);
                            ui.horizontal(|ui| {
                                let button = ui.add_upgrade_button(up_texture).on_hover_text("Upgrade to increase bullet production.");
                                ui.strong(format!("{}", cost));
                                ui.add_image(materials_texture, [20., 20.]);
                                if button.clicked() {
                                    if player.resources.materials >= cost {
                                        player.resources.materials -= cost;
                                        player.constructions.armory += 1;
                                    } else {
                                        messages.error("Not enough materials.");
                                    }
                                }
                            });
                        });

                        ui.add_space(40.);

                        ui.add_image(refinery_texture, [130., 130.]);
                        ui.add_space(20.);
                        ui.vertical(|ui| {
                            let cost = ((player.constructions.refinery + 1) * 100) as f32;

                            ui.strong("Refinery");
                            ui.label(format!("Level: {}", player.constructions.refinery));
                            ui.label(format!("Refiners: {}", player.population.refiner));
                            ui.label(format!("Production: +{:.0}", new_resources.gasoline)).on_hover_text("Production of gasoline per night.");
                            ui.add_space(10.);
                            ui.horizontal(|ui| {
                                let button = ui.add_upgrade_button(up_texture).on_hover_text("Upgrade to increase gasoline production.");
                                ui.strong(format!("{}", cost));
                                ui.add_image(materials_texture, [20., 20.]);
                                if button.clicked() {
                                    if player.resources.materials >= cost {
                                        player.resources.materials -= cost;
                                        player.constructions.refinery += 1;
                                    } else {
                                        messages.error("Not enough materials.");
                                    }
                                }
                            });
                        });
                    });

                    ui.add_space(35.);

                    ui.horizontal(|ui| {
                        ui.add_space(30.);
                        ui.add_image(factory_texture, [130., 130.]);
                        ui.add_space(20.);
                        ui.vertical(|ui| {
                            let cost = ((player.constructions.factory + 1) * 100) as f32;

                            ui.strong("Factory");
                            ui.label(format!("Level: {}", player.constructions.factory));
                            ui.label(format!("Constructors: {}", player.population.constructor));
                            ui.label(format!("Production: +{:.0}", new_resources.materials)).on_hover_text("Production of materials per night.");
                            ui.add_space(10.);
                            ui.horizontal(|ui| {
                                let button = ui.add_upgrade_button(up_texture).on_hover_text("Upgrade to increase materials production.");
                                ui.strong(format!("{}", cost));
                                ui.add_image(materials_texture, [20., 20.]);
                                if button.clicked() {
                                    if player.resources.materials >= cost {
                                        player.resources.materials -= cost;
                                        player.constructions.factory += 1;
                                    } else {
                                        messages.error("Not enough materials.");
                                    }
                                }
                            });
                        });

                        ui.add_space(40.);

                        ui.add_image(laboratory_texture, [130., 130.]);
                        ui.add_space(20.);
                        ui.vertical(|ui| {
                            let cost = ((player.constructions.laboratory + 1) * 100) as f32;

                            ui.strong("Laboratory");
                            ui.label(format!("Level: {}", player.constructions.laboratory));
                            ui.label(format!("Scientists: {}", player.population.scientist));
                            ui.label(format!("Production: +{:.0}", new_resources.technology)).on_hover_text("Production of technology per night.");
                            ui.add_space(10.);
                            ui.horizontal(|ui| {
                                let button = ui.add_upgrade_button(up_texture).on_hover_text("Upgrade to increase technology production.");
                                ui.strong(format!("{}", cost));
                                ui.add_image(materials_texture, [20., 20.]);
                                if button.clicked() {
                                    if player.resources.materials >= cost {
                                        player.resources.materials -= cost;
                                        player.constructions.laboratory += 1;
                                    } else {
                                        messages.error("Not enough materials.");
                                    }
                                }
                            });
                        });
                    });

                    ui.add_space(35.);

                    ui.horizontal(|ui| {
                        ui.add_space(20.);
                        ui.heading("Defense");
                    });
                    ui.add_space(15.);

                    ui.horizontal(|ui| {
                        ui.add_space(30.);
                        ui.vertical(|ui| {
                            ui.add_space(20.);
                            ui.add_image(wall_texture, [130., 130.]);

                            ui.add_space(-30.);
                            ui.horizontal(|ui| {
                                frame.show(ui, |ui| {
                                    ui.add_image(spots_texture, [25., 25.]);
                                    ui.strong(format!("{} / {}", player.weapons.spots.len(), MAX_SPOTS));
                                }).response.on_hover_text("Available spots / maximum spots.");
                            });
                        });
                        ui.add_space(20.);
                        ui.vertical(|ui| {
                            ui.strong("Wall");
                            ui.add_space(10.);
                            ui.horizontal(|ui| {
                                let cost = player.wall.max_health;

                                let button = ui.add_upgrade_button(up_texture).on_hover_text("Upgrade to increase the max health.");
                                ui.strong(format!("{}", cost));
                                ui.add_image(materials_texture, [20., 20.]);
                                if button.clicked() {
                                    if player.resources.materials >= cost {
                                        player.resources.materials -= cost;
                                        player.wall.health += 1000.;
                                        player.wall.max_health += 1000.;
                                    } else {
                                        messages.error("Not enough materials.");
                                    }
                                }
                            });
                            ui.add_space(10.);
                            ui.horizontal(|ui| {
                                ui.add_enabled_ui(player.wall.health < player.wall.max_health, |ui| {
                                    let cost = 100.;

                                    let button = ui.add_upgrade_button(repair_texture)
                                        .on_hover_text("Add 500 health to the wall. Double click to repair to full health.");
                                    ui.strong(format!("{}", cost));
                                    ui.add_image(materials_texture, [20., 20.]);
                                    if button.clicked() {
                                        if player.resources.materials >= cost {
                                            player.resources.materials -= cost;
                                            player.wall.health += 500.;
                                        } else {
                                            messages.error("Not enough materials.");
                                        }
                                    }

                                    // Double-click to repair to full health
                                    if button.double_clicked() {
                                        let cost = ((player.wall.max_health - player.wall.health) as u32 + 499 / 5) as f32;
                                        if player.resources.materials >= cost {
                                            player.resources.materials -= cost;
                                            player.wall.health = player.wall.max_health;
                                        } else {
                                            messages.error("Not enough materials.");
                                        }
                                    }
                                });
                            });
                            ui.add_space(10.);
                            ui.add_enabled_ui(player.weapons.spots.len() < MAX_SPOTS, |ui| {
                                ui.horizontal(|ui| {
                                    let cost = 500.;

                                    let button = ui.add_upgrade_button(spots_texture)
                                        .on_hover_text("Add an extra weapon on the wall.")
                                        .on_disabled_hover_text("Maximum number of weapons reached.");
                                    ui.strong(format!("{}", cost));
                                    ui.add_image(technology_texture, [20., 20.]);
                                    if button.clicked() {
                                        if player.resources.technology >= cost {
                                            player.resources.technology -= cost;
                                            player.weapons.spots.push(Spot { id: Uuid::new_v4(), weapon: None });
                                        } else {
                                            messages.error("Not enough technology.");
                                        }
                                    }
                                });
                            });

                            spawn_wall(&mut commands, &wall_q, &player, &asset_server);

                            if player.wall.health > player.wall.max_health {
                                player.wall.health = player.wall.max_health;
                            }
                        });

                        ui.add_space(40.);

                        ui.vertical(|ui| {
                            ui.add_space(20.);
                            ui.add_image(fence_texture, [130., 130.]);

                            if player.fence.max_health > 0. && player.has_tech(TechnologyName::Electricity) {
                                ui.add_space(-30.);
                                ui.horizontal(|ui| {
                                    frame.show(ui, |ui| {
                                        ui.add_image(lightning_texture, [25., 25.]);
                                        ui.strong(format!("x{}", player.fence.damage));
                                    }).response.on_hover_text("Damage to adjacent enemies.");
                                });
                            }
                        });
                        ui.add_space(20.);
                        ui.vertical(|ui| {
                            ui.strong("Fence");
                            ui.add_space(10.);
                            ui.horizontal(|ui| {
                                let cost = 100. + player.fence.max_health;

                                let button = ui.add_upgrade_button(up_texture).on_hover_text("Upgrade to increase the max health.");
                                ui.strong(format!("{}", cost));
                                ui.add_image(materials_texture, [20., 20.]);
                                if button.clicked() {
                                    if player.resources.materials >= cost {
                                        player.resources.materials -= cost;
                                        player.fence.health += 100.;
                                        player.fence.max_health += 100.;
                                    } else {
                                        messages.error("Not enough materials.");
                                    }
                                }
                            });
                            ui.add_space(10.);
                            ui.horizontal(|ui| {
                                ui.add_enabled_ui(player.fence.health < player.fence.max_health, |ui| {
                                    let cost = 100.;

                                    let button = ui.add_upgrade_button(repair_texture)
                                        .on_hover_text("Add 300 health to the fence. Double click to repair to full health.");
                                    ui.strong(format!("{}", cost));
                                    ui.add_image(materials_texture, [20., 20.]);
                                    if button.clicked() {
                                        if player.resources.materials >= cost {
                                            player.resources.materials -= cost;
                                            player.fence.health += 300.;
                                        } else {
                                            messages.error("Not enough materials.");
                                        }
                                    }

                                    // Double-click to repair to full health
                                    if button.double_clicked() {
                                        let cost = ((player.fence.max_health - player.fence.health) as u32 + 299 / 3) as f32;
                                        if player.resources.materials >= cost {
                                            player.resources.materials -= cost;
                                            player.fence.health = player.fence.max_health;
                                        } else {
                                            messages.error("Not enough materials.");
                                        }
                                    }
                                });
                            });
                            ui.add_space(10.);
                            ui.add_enabled_ui(player.fence.max_health > 0. && player.has_tech(TechnologyName::Electricity), |ui| {
                                ui.horizontal(|ui| {
                                    let cost = player.fence.damage * 20.;

                                    let button = ui.add_upgrade_button(lightning_texture)
                                        .on_hover_text("Increase the fence's damage.")
                                        .on_disabled_hover_text("Requires the electricity technology.");
                                    ui.strong(format!("{}", cost));
                                    ui.add_image(technology_texture, [20., 20.]);
                                    if button.clicked() {
                                        if player.resources.technology >= cost {
                                            player.resources.technology -= cost;
                                            player.fence.damage += 5.;
                                        } else {
                                            messages.error("Not enough technology.");
                                        }
                                    }
                                });
                            });

                            spawn_fence(&mut commands, &fence_q, &player, &asset_server);

                            if player.fence.health > player.fence.max_health {
                                player.fence.health = player.fence.max_health;
                            }
                        });
                    });
                }
                DayTabs::Armory => {
                    let frame = egui::Frame::none()
                        .fill(Color32::from_black_alpha(190))
                        .rounding(15.)
                        .inner_margin(egui::vec2(5., 5.));

                    let textures: HashMap<&str, TextureId> = HashMap::from([
                        ("MachineGun", machine_gun_texture),
                        ("Canon", canon_texture),
                        ("Flamethrower", flamethrower_texture),
                        ("MissileLauncher", missile_launcher_texture),
                        ("Turret", turret_texture),
                        ("Artillery", artillery_texture),
                        ("AAA", aaa_texture),
                        ("Mortar", mortar_texture),
                        ("spots", spots_texture),
                        ("cross", cross_texture),
                        ("up", up_texture),
                        ("damage", damage_texture),
                        ("explosion", explosion_texture),
                        ("reload", reload_texture),
                        ("range", range_texture),
                        ("penetration", penetration_texture),
                        ("targets", targets_texture),
                        ("materials", materials_texture),
                        ("technology", technology_texture),
                    ]);

                    ui.add_space(10.);
                    ScrollArea::vertical()
                        .id_salt("weapons")
                        .max_width(MAP_SIZE.x * 0.6)
                        .max_height(MAP_SIZE.y * 0.55)
                        .show(ui, |ui| {
                            ui.add_space(5.);
                            ui.horizontal(|ui| {
                                ui.add_space(20.);
                                ui.heading("Weapons");
                                ui.add_space(550.);
                            });

                            ui.horizontal(|ui| {
                                ui.add_space(70.);
                                ui.add_weapon(&textures, &mut weapons.machine_gun, &mut player, &mut messages);
                                ui.add_space(70.);
                                ui.add_weapon(&textures, &mut weapons.canon, &mut player, &mut messages);
                            });
                            ui.add_space(30.);
                            ui.horizontal(|ui| {
                                ui.add_space(70.);
                                ui.add_weapon(&textures, &mut weapons.aaa, &mut player, &mut messages);
                                ui.add_space(70.);
                                ui.add_weapon(&textures, &mut weapons.flamethrower, &mut player, &mut messages);
                            });
                            ui.add_space(30.);
                            ui.horizontal(|ui| {
                                ui.add_space(70.);
                                ui.add_weapon(&textures, &mut weapons.mortar, &mut player, &mut messages);
                                ui.add_space(70.);
                                ui.add_weapon(&textures, &mut weapons.artillery, &mut player, &mut messages);
                            });
                            ui.add_space(30.);
                            ui.add_enabled_ui(player.has_tech(TechnologyName::Homing), |ui| {
                                ui.horizontal(|ui| {
                                    ui.add_space(70.);
                                    ui.add_weapon(&textures, &mut weapons.turret, &mut player, &mut messages);
                                    ui.add_space(70.);
                                    ui.add_weapon(&textures, &mut weapons.missile_launcher, &mut player, &mut messages);
                                });
                            }).response.on_disabled_hover_text("Requires the homing technology.");

                            ui.add_space(35.);
                            ui.horizontal(|ui| {
                                ui.add_space(20.);
                                ui.heading("Explosives");
                            });

                            ui.add_space(15.);
                            ui.add_enabled_ui(player.has_tech(TechnologyName::Explosives), |ui| {
                                ui.horizontal(|ui| {
                                    ui.add_space(85.);

                                    ui.vertical(|ui| {
                                        ui.add_space(20.);
                                        ui.add_image(mine_texture, [50., 30.])
                                            .on_hover_text("\
                                                Small explosive placed at random locations. Explodes when an \
                                                enemy walks over (is never triggered by flying enemies). You \
                                                can decide at which enemy size to detonate. Maximum of 25 allowed.")
                                            .on_disabled_hover_text("Requires the explosives technology.");

                                        ui.add_space(-10.);
                                        ui.horizontal(|ui| {
                                            ui.add_space(-10.);
                                            frame.show(ui, |ui| {
                                                ui.strong(format!("{}", player.weapons.mines));
                                            });
                                            let button = ui.add_upgrade_button(up_texture).on_hover_text("Buy a mine.");

                                            if button.clicked() {
                                                if player.weapons.mines >= MAX_MINES {
                                                    messages.error("Maximum number of mines reached.");
                                                } else {
                                                    if player.resources >= weapons.mine.price {
                                                        player.resources -= &weapons.mine.price;
                                                        player.weapons.mines += 1;
                                                    } else {
                                                        messages.error("Not enough resources.");
                                                    }
                                                }
                                            }
                                        });
                                    });
                                    ui.vertical(|ui| {
                                        ui.strong("Mine");
                                        ui.add_space(10.);
                                        ui.horizontal(|ui| {
                                            ui.strong(format!("{}", weapons.mine.price.bullets));
                                            ui.add_image(bullets_texture, [20., 20.]);
                                        });
                                        ui.horizontal(|ui| {
                                            ui.strong(format!("{}", weapons.mine.price.gasoline));
                                            ui.add_image(gasoline_texture, [20., 20.]);
                                        });
                                    });

                                    ui.add_space(70.);

                                    ui.vertical(|ui| {
                                        ui.add_space(20.);
                                        ui.add_image(bomb_texture, [50., 30.])
                                            .on_hover_text("\
                                                Large explosive that flies towards the selected target \
                                                (strongest enemy or densest location). Maximum of 5 allowed.")
                                            .on_disabled_hover_text("Requires the explosives technology.");

                                        ui.add_space(-10.);
                                        ui.horizontal(|ui| {
                                            ui.add_space(-10.);
                                            frame.show(ui, |ui| {
                                                ui.strong(format!("{}", player.weapons.bombs));
                                            });
                                            let button = ui.add_upgrade_button(up_texture).on_hover_text("Buy a bomb.");

                                            if button.clicked() {
                                                if player.weapons.bombs >= MAX_BOMBS {
                                                    messages.error("Maximum number of bombs reached.");
                                                } else {
                                                    if player.resources >= weapons.bomb.price {
                                                        player.resources -= &weapons.bomb.price;
                                                        player.weapons.bombs += 1;
                                                    } else {
                                                        messages.error("Not enough resources.");
                                                    }
                                                }
                                            }
                                        });
                                    });
                                    ui.vertical(|ui| {
                                        ui.strong("Bomb");
                                        ui.add_space(10.);
                                        ui.horizontal(|ui| {
                                            ui.strong(format!("{}", weapons.bomb.price.bullets));
                                            ui.add_image(bullets_texture, [20., 20.]);
                                        });
                                        ui.horizontal(|ui| {
                                            ui.strong(format!("{}", weapons.bomb.price.gasoline));
                                            ui.add_image(gasoline_texture, [20., 20.]);
                                        });
                                    });

                                    ui.add_enabled_ui(player.has_tech(TechnologyName::Physics), |ui| {
                                        ui.add_space(70.);

                                        ui.vertical(|ui| {
                                            ui.add_space(20.);
                                            ui.add_image(nuke_texture, [50., 30.])
                                                .on_hover_text("\
                                                    Huge explosive that, after a short delay, kills every enemy \
                                                    on the map. The fence and wall are also destroyed. Maximum \
                                                    of 1 allowed.")
                                                .on_disabled_hover_text("Requires the explosives and physics technology.");

                                            ui.add_space(-10.);
                                            ui.horizontal(|ui| {
                                                ui.add_space(-10.);
                                                frame.show(ui, |ui| {
                                                    ui.strong(format!("{}", player.weapons.nuke));
                                                });
                                                let button = ui.add_upgrade_button(up_texture).on_hover_text("Buy a nuke.");

                                                if button.clicked() {
                                                    if player.weapons.nuke >= MAX_NUKES {
                                                        messages.error("Maximum number of nukes reached.");
                                                    } else {
                                                        if player.resources >= weapons.nuke.price {
                                                            player.resources -= &weapons.nuke.price;
                                                            player.weapons.nuke += 1;
                                                        } else {
                                                            messages.error("Not enough resources.");
                                                        }
                                                    }
                                                }
                                            });
                                        });
                                        ui.vertical(|ui| {
                                            ui.strong("Nuke");
                                            ui.add_space(10.);
                                            ui.horizontal(|ui| {
                                                ui.strong(format!("{}", weapons.nuke.price.bullets));
                                                ui.add_image(bullets_texture, [20., 20.]);
                                            });
                                            ui.horizontal(|ui| {
                                                ui.strong(format!("{}", weapons.nuke.price.gasoline));
                                                ui.add_image(gasoline_texture, [20., 20.]);
                                            });
                                        });
                                    });
                                });
                            });
                        });

                ui.add_space(15.);
                ui.horizontal(|ui| {
                    ui.add_space(20.);
                    ui.heading("Wall");
                });

                ui.add_space(15.);
                ui.horizontal(|ui| {
                        ui.add_space(ui.available_width() * 0.5 - player.weapons.spots.len() as f32 * 36.);

                        dnd(ui, "armory").show_vec(&mut player.weapons.spots, |ui, item, handle, _| {
                            handle.ui(ui, |ui| {
                                let texture = match item.weapon {
                                    Some(WeaponName::AAA) => aaa_texture,
                                    Some(WeaponName::Artillery) => artillery_texture,
                                    Some(WeaponName::Canon) => canon_texture,
                                    Some(WeaponName::Flamethrower) => flamethrower_texture,
                                    Some(WeaponName::MachineGun) => machine_gun_texture,
                                    Some(WeaponName::MissileLauncher) => missile_launcher_texture,
                                    Some(WeaponName::Mortar) => mortar_texture,
                                    Some(WeaponName::Turret) => turret_texture,
                                    None => cross_texture,
                                };

                                let response = ui.add_image(texture, [50., 50.]);
                                if let Some(w) = item.weapon {
                                    response.on_hover_text(w.name());
                                }
                            });

                            ui.add_space(10.);
                        });

                        spawn_spots(&mut commands, &weapon_q, &player, &weapons, &asset_server);
                    });
                }
                DayTabs::Technology => {
                    for category in TechnologyCategory::iter() {
                        ui.add_space(5.);
                        ui.horizontal(|ui| {
                            ui.add_space(20.);
                            ui.heading(category.name());
                        });
                        ui.add_space(15.);

                        ui.horizontal(|ui| {
                            for t in Technology::iter().filter(|t| t.category == category) {
                                ui.add_space(10.);

                                ui.add_enabled_ui(!player.has_tech(t.name), |ui| {
                                    let response = ui.add_technology(
                                        &t,
                                        &player,
                                        technology_texture,
                                        tick_texture,
                                    );
                                    if response.clicked() {
                                        if player.resources.technology >= t.price {
                                            player.resources.technology -= t.price;
                                            player.technology.insert(t.name);
                                            messages.info(format!(
                                                "Technology {} researched.",
                                                t.name.name()
                                            ));
                                        } else {
                                            messages.error("Not enough resources!");
                                        }
                                    }
                                });
                            }
                        });

                        ui.add_space(15.);
                    }
                }
                DayTabs::Expeditions => {
                    ui.add_space(10.);

                    ui.add_scroll("exp", MAP_SIZE.x * 0.1, |ui| {
                        ui.add_space(15.);
                        ui.label(
                            "\
                            Send expeditions to explore the surroundings. The larger the \
                            expedition, the more resources it costs and the longer it takes \
                            to return, but the larger the possible rewards. But be aware, \
                            some expeditions never return...",
                        );
                        ui.add_space(25.);
                    });

                    let textures = HashMap::from([
                        ("population", population_texture),
                        ("gasoline", gasoline_texture),
                        ("materials", materials_texture),
                        ("clock", clock_texture),
                    ]);

                    if let Some(expedition) = &player.expedition {
                        ui.add_scroll("exp2", MAP_SIZE.x * 0.1, |ui| {
                            ui.add_space(15.);
                            ui.strong(format!(
                                "A {} expedition was send {} days ago. We haven't heard from them since...",
                                expedition.name.name().to_lowercase(),
                                expedition.day)
                            );
                            ui.add_space(15.);
                        });
                    } else {
                        ui.horizontal(|ui| {
                            for expedition in Expedition::iter() {
                                ui.add_space(20.);
                                let response = ui.add_expedition(&expedition, &textures);
                                if response.clicked() {
                                    if player.population.idle >= expedition.population {
                                        if player.resources >= expedition.price {
                                            player.population.idle -= expedition.population;
                                            player.resources -= &expedition.price;
                                            player.expedition = Some(expedition.clone());

                                            messages.info(format!(
                                                "{} expedition send.",
                                                expedition.name.name()
                                            ));
                                        } else {
                                            messages.error("Not enough resources!");
                                        }
                                    } else {
                                        messages.error("Not enough idle population!");
                                    }
                                }
                            }
                        });
                    }
                }
            }

            ui.add_space(15.);
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

                        ui.add_scroll("start", MAP_SIZE.x * 0.1, |ui| {
                            ui.add_space(15.);
                            ui.label(
                                "The world has been conquered by insects. Together with a \
                                handful of survivors, you have built a fortress to defend yourself \
                                from their ferocious attacks. Every night, an ever-increasing swarm \
                                attacks the fortress. Kill them before they enter the fortress and \
                                finish the remaining population!\n\n\
                                During the day, you can collect resources, research technologies, \
                                send expeditions, and most importantly, upgrade your weapon arsenal \
                                to prepare yourself for the following night. During the attack, you \
                                can choose how/when to use the weapons you have at your disposal. \
                                But be careful, everything has a cost! Manage your resources wisely \
                                or you won't be able to stop the bugs tomorrow...");
                            ui.add_space(15.);
                        });

                        if ui.add_button("Start game").clicked() {
                            next_state.set(AppState::Night);
                        }
                    },
                    AppState::GameOver => {
                        ui.add_image(game_over_texture,[400., 100.]);

                        ui.heading(format!("You survived {} nights!", player.day - 1));

                        ui.add_night_stats(&player, player.day);

                        ui.horizontal(|ui| {
                            ui.add_space(200.);
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
                                    ui.strong(e.name);
                                    ui.label(format!("Size: {:?}", e.size));
                                    ui.label(format!("Health: {}", e.health));
                                    ui.label(format!("Armor: {}", e.armor))
                                        .on_hover_text("Armor reduces incoming damage.");
                                    ui.label(format!("Speed: {:.0}", e.speed)).on_hover_text(
                                        "As percentage of the map's height per second.",
                                    );
                                    ui.label(format!("Can fly: {}", e.flies))
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

pub fn expedition_panel(
    mut contexts: EguiContexts,
    mut player: ResMut<Player>,
    assets: Local<WorldAssets>,
    window: Query<&Window>,
) {
    let population_texture = contexts.add_image(assets.get_image("population"));
    let bullets_texture = contexts.add_image(assets.get_image("bullets"));
    let gasoline_texture = contexts.add_image(assets.get_image("gasoline"));
    let materials_texture = contexts.add_image(assets.get_image("materials"));
    let mine_texture = contexts.add_image(assets.get_image("mine"));
    let bomb_texture = contexts.add_image(assets.get_image("bomb"));

    if let Some(expedition) = player.expedition.clone() {
        if matches!(expedition.status, ExpeditionStatus::Ongoing) {
            return;
        }

        let window_size = window.single().size();

        egui::Window::new("expedition panel")
            .title_bar(false)
            .fixed_size((MAP_SIZE.x * 0.4, MAP_SIZE.y * 0.4))
            .fixed_pos((
                (window_size.x - WEAPONS_PANEL_SIZE.x) * 0.5 - MAP_SIZE.x * 0.2,
                (window_size.y - RESOURCES_PANEL_SIZE.y) * 0.5 - MAP_SIZE.y * 0.2,
            ))
            .show(contexts.ctx_mut(), |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(10.);

                    match &expedition.status {
                        ExpeditionStatus::Lost => {
                            ui.heading("An expedition has been lost!");
                            ui.add_space(20.);
                            ui.add_scroll("exp", 50., |ui| {
                                ui.label(format!(
                                    "A {} expedition was send out {} days ago. \
                                    There is no longer any hope of them returning...",
                                    expedition.name.name().to_lowercase(),
                                    expedition.day
                                ));
                            });
                        }
                        ExpeditionStatus::Returned(reward) => {
                            ui.heading(format!(
                                "The expedition returned after {} days!",
                                expedition.day
                            ));
                            ui.add_space(20.);
                            ui.label("The expedition brought back the following:");

                            if reward.population > 0 {
                                ui.add_space(10.);
                                ui.horizontal(|ui| {
                                    ui.add_space(160.);
                                    ui.add_image(population_texture, [20., 20.]);
                                    ui.label(format!("Population: {:.0}", reward.population));
                                });
                            }

                            if reward.resources.bullets > 0. {
                                ui.add_space(10.);
                                ui.horizontal(|ui| {
                                    ui.add_space(160.);
                                    ui.add_image(bullets_texture, [20., 20.]);
                                    ui.label(format!("Bullets: {:.0}", reward.resources.bullets));
                                });
                            }

                            if reward.resources.gasoline > 0. {
                                ui.add_space(10.);
                                ui.horizontal(|ui| {
                                    ui.add_space(160.);
                                    ui.add_image(gasoline_texture, [20., 20.]);
                                    ui.label(format!("Gasoline: {:.0}", reward.resources.gasoline));
                                });
                            }

                            if reward.resources.materials > 0. {
                                ui.add_space(10.);
                                ui.horizontal(|ui| {
                                    ui.add_space(160.);
                                    ui.add_image(materials_texture, [20., 20.]);
                                    ui.label(format!(
                                        "Materials: {:.0}",
                                        reward.resources.materials
                                    ));
                                });
                            }

                            if reward.mines > 0 {
                                ui.add_space(10.);
                                ui.horizontal(|ui| {
                                    ui.add_space(160.);
                                    ui.add_image(mine_texture, [20., 20.]);
                                    ui.label(format!("Mines: {:.0}", reward.mines));
                                });
                            }

                            if reward.bombs > 0 {
                                ui.add_space(10.);
                                ui.horizontal(|ui| {
                                    ui.add_space(160.);
                                    ui.add_image(bomb_texture, [20., 20.]);
                                    ui.label(format!("Bombs: {:.0}", reward.bombs));
                                });
                            }
                        }
                        _ => unreachable!(),
                    }
                });

                ui.with_layout(Layout::right_to_left(Align::RIGHT), |ui| {
                    ui.add_space(10.);
                    if ui.add_button("Ok").clicked() {
                        player.resolve_expedition();
                    }
                });

                ui.add_space(10.);
            });
    }
}

pub fn run_animations(
    mut commands: Commands,
    mut animation_q: Query<(Entity, &Transform, &mut AnimationComponent, &mut Sprite)>,
    mut enemy_q: Query<(Entity, &Transform, &mut Enemy)>,
    fence_q: Query<&Transform, With<FenceComponent>>,
    wall_q: Query<&Transform, With<WallComponent>>,
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
    fence_q: Query<Entity, With<FenceComponent>>,
    wall_q: Query<Entity, With<WallComponent>>,
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
                    .entry(enemy.name.to_string())
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
    bullet_q: Query<Entity, (With<Bullet>, Without<Mine>)>,
    enemy_q: Query<Entity, With<Enemy>>,
) {
    animation_q
        .iter()
        .for_each(|a| commands.entity(a).try_despawn());
    bullet_q
        .iter()
        .for_each(|b| commands.entity(b).try_despawn());
    enemy_q
        .iter()
        .for_each(|e| commands.entity(e).despawn_recursive());
}
