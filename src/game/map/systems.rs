use super::components::*;
use super::constants::*;
use crate::game::components::*;
use crate::game::resources::Player;
use crate::game::systems::{pause_game, resume_game};
use crate::game::weapon::components::{Weapon, WeaponSettings};
use crate::game::{AppState, GameState};
use bevy::color::palettes::basic::WHITE;
use bevy::prelude::*;
use bevy_egui::egui::{RichText, Style, TextStyle};
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

pub fn setup_map(mut commands: Commands, player: Res<Player>, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    // Spawn background images ======================================= >>

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

    // Spawn hidden pause banner ===================================== >>

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

    // Spawn weapons ================================================= >>

    let weapon = Weapon::sentry_gun();

    let mut pos = -SIZE.x * 0.5;
    for _ in 0..player.weapons.sentry_gun {
        pos += MAP_SIZE.x / (player.weapons.sentry_gun + 1) as f32;

        commands.spawn((
            Sprite {
                image: asset_server.load(&weapon.image),
                custom_size: Some(weapon.size),
                ..default()
            },
            Transform::from_xyz(
                pos,
                -SIZE.y * 0.5 + RESOURCES_PANEL_SIZE.y + WALL_SIZE.y * 0.5,
                2.0,
            ),
            weapon.clone(),
        ));
    }
}

pub fn menu_panel(
    mut contexts: EguiContexts,
    vis_q: Query<&mut Visibility, With<PauseWrapper>>,
    app_state: Res<State<AppState>>,
    game_state: Res<State<GameState>>,
    next_state: ResMut<NextState<GameState>>,
) {
    egui::TopBottomPanel::top("Menu")
        .exact_height(MENU_PANEL_SIZE.y)
        .show(contexts.ctx_mut(), |ui| {
            ui.horizontal_centered(|ui| {
                egui::menu::bar(ui, |ui| {
                    egui::menu::menu_button(ui, "Game", |ui| {
                        if ui.button("New game").clicked() {
                            std::process::exit(0);
                        }
                        if ui.button("Load game").clicked() {
                            std::process::exit(0);
                        }
                        if ui.button("Save game").clicked() {
                            std::process::exit(0);
                        }
                        if ui.button("Quit").clicked() {
                            std::process::exit(0);
                        }
                    });
                    if *app_state.get() == AppState::Game {
                        egui::menu::menu_button(ui, "State", |ui| {
                            if ui.button("Toggle pause").clicked() {
                                match game_state.get() {
                                    GameState::Running => pause_game(vis_q, next_state),
                                    GameState::Paused => resume_game(vis_q, next_state),
                                }
                            }
                        });
                    }
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
    mut player: ResMut<Player>,
    images: Local<Images>,
) {
    let day_texture = contexts.add_image(images.day.clone_weak());
    let fortress_texture = contexts.add_image(images.fortress.clone_weak());
    let bullets_texture = contexts.add_image(images.bullets.clone_weak());
    let gasoline_texture = contexts.add_image(images.gasoline.clone_weak());
    let materials_texture = contexts.add_image(images.materials.clone_weak());

    egui::TopBottomPanel::bottom("Resources")
        .exact_height(RESOURCES_PANEL_SIZE.y)
        .show(contexts.ctx_mut(), |ui| {
            ui.horizontal_centered(|ui| {
                ui.add_space(5.);

                ui.add(egui::widgets::Image::new(egui::load::SizedTexture::new(
                    day_texture,
                    [20., 20.],
                )));
                ui.add(egui::Label::new(player.day.to_string()));

                ui.add_space(5.);
                ui.separator();
                ui.add_space(5.);

                ui.add(egui::widgets::Image::new(egui::load::SizedTexture::new(
                    fortress_texture,
                    [20., 20.],
                )));
                ui.add(
                    egui::ProgressBar::new(
                        player.wall.health as f32 / player.wall.max_health as f32,
                    )
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

                ui.add(egui::widgets::Image::new(egui::load::SizedTexture::new(
                    bullets_texture,
                    [20., 20.],
                )));
                ui.add(egui::Label::new(player.resources.bullets.to_string()));

                ui.add_space(15.);

                ui.add(egui::widgets::Image::new(egui::load::SizedTexture::new(
                    gasoline_texture,
                    [20., 20.],
                )));
                ui.add(egui::Label::new(player.resources.gasoline.to_string()));

                ui.add_space(15.);

                ui.add(egui::widgets::Image::new(egui::load::SizedTexture::new(
                    materials_texture,
                    [20., 20.],
                )));
                ui.add(egui::Label::new(player.resources.materials.to_string()));
            });
        });
}

pub fn weapons_panel(
    mut contexts: EguiContexts,
    mut weapon_q: Query<&mut Weapon>,
    mut settings: ResMut<WeaponSettings>,
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
                        ui.add_space(35.);
                        ui.add(egui::widgets::Image::new(egui::load::SizedTexture::new(
                            weapon_texture,
                            [30., 30.],
                        )));
                        ui.heading("Weapons");
                    });
                });

                ui.add_space(5.);
                ui.separator();
                ui.add_space(5.);

                ui.horizontal(|ui| {
                    ui.add(egui::Label::new("Sentry gun: "));

                    let fire_rate = ui
                        .add(egui::Slider::new(
                            &mut settings.sentry_gun_fire_rate_value,
                            0..=5,
                        ))
                        .on_hover_text("Fire rate of the sentry guns. Shoots N bullets per second.");

                    if fire_rate.dragged() {
                        weapon_q
                            .iter_mut()
                            .filter(|w| w.name == "Sentry gun")
                            .for_each(|mut w| {
                                w.fire_rate = match settings.sentry_gun_fire_rate_value as f32 {
                                    0. => None,
                                    v => Some(Timer::from_seconds(1. / v, TimerMode::Repeating)),
                                };
                            });
                    }
                })
            });
        });
}

pub fn start_game_panel(mut contexts: EguiContexts, mut next_state: ResMut<NextState<AppState>>) {
    egui::Window::new("start game")
        .title_bar(false)
        .fixed_size((MAP_SIZE.x * 0.6, MAP_SIZE.y * 0.8))
        .fixed_pos((MAP_SIZE.x * 0.3, MAP_SIZE.y * 0.4))
        .show(contexts.ctx_mut(), |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(5.);
                ui.heading("Welcome to Fortress!");

                ui.add_space(15.);

                egui::ScrollArea::vertical()
                    .max_width(SIZE.x * 0.4)
                    .show(ui, |ui| {
                        ui.with_layout(
                            egui::Layout::top_down(egui::Align::LEFT),
                            |ui| {
                                ui.add_space(5.);
                                ui.label(
                                    "The world has been conquered by monsters. Together \
                                    with a handful of survivors, you have created a fortress \
                                    to defend yourself from their attacks.\n\nEvery night, a \
                                    swarm of monsters attacks your fortress. Try to shoot them \
                                    down before they reach the wall! When they do, they will \
                                    start to hit the wall, and if the wall is destroyed, the \
                                    monsters can freely enter the fortress and the game is \
                                    lost.\n\nDuring the day, you can upgrade your weapon \
                                    arsenal to prepare yourself for the following attack, but \
                                    manage your resources well, or you won't be able to stop \
                                    the monsters anymore...");
                                ui.add_space(5.);
                            },
                        );
                });

                ui.add_space(15.);

                if ui.add_sized([120., 40.], egui::Button::new("Start game")).clicked() {
                    next_state.set(AppState::Game);
                }
            });


            ui.add_space(5.);
        });
}
