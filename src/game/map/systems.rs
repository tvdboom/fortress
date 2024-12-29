use super::components::*;
use super::constants::*;
use crate::game::components::*;
use crate::game::enemy::components::Enemy;
use crate::game::resources::Player;
use crate::game::systems::{pause_game, resume_game};
use crate::game::weapon::components::{Bullet, Weapon, WeaponSettings};
use crate::game::{AppState, GameState};
use bevy::color::palettes::basic::WHITE;
use bevy::prelude::*;
use bevy_egui::egui::{epaint, Response, RichText, Style, TextStyle, TextureId, Ui, WidgetText};
use bevy_egui::{egui, EguiContexts};
use catppuccin_egui;

trait CustomUi {
    fn add_button(&mut self, text: impl Into<WidgetText>) -> Response;
    fn add_image(&mut self, id: impl Into<TextureId>, size: impl Into<epaint::Vec2>) -> Response;
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
}

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
    vis_q: Query<&mut Visibility, With<PauseWrapper>>,
    app_state: Res<State<AppState>>,
    game_state: Res<State<GameState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    next_game_state: ResMut<NextState<GameState>>,
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
                                GameState::Running => pause_game(vis_q, next_game_state),
                                GameState::Paused => resume_game(vis_q, next_game_state),
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

pub fn resources_panel(mut contexts: EguiContexts, player: Res<Player>, images: Local<Images>) {
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

                ui.add_image(day_texture, [20., 20.]);
                ui.add(egui::Label::new(player.day.to_string()));

                ui.add_space(5.);
                ui.separator();
                ui.add_space(5.);

                ui.add_image(fortress_texture, [20., 20.]);
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

                ui.add_image(bullets_texture, [20., 20.]);
                ui.add(egui::Label::new(player.resources.bullets.to_string()));

                ui.add_space(15.);

                ui.add_image(gasoline_texture, [20., 20.]);
                ui.add(egui::Label::new(player.resources.gasoline.to_string()));

                ui.add_space(15.);

                ui.add_image(materials_texture, [20., 20.]);
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
                        ui.add_space(55.);
                        ui.add_image(weapon_texture, [30., 30.]);
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
                        .on_hover_text(
                            "Fire rate of the sentry guns. Shoots N bullets per second.",
                        );

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

                        ui.heading(format!("You survived {} days!", player.day));

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
                                            ui.label(k);
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
    bullet_q: Query<Entity, With<Bullet>>,
    enemy_q: Query<Entity, With<Enemy>>,
) {
    enemy_q
        .iter()
        .for_each(|e| commands.entity(e).despawn_recursive());
    bullet_q.iter().for_each(|b| commands.entity(b).despawn());
}
