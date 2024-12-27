use super::components::*;
use super::constants::*;
use crate::game::components::*;
use crate::game::enemy::components::{Enemy, EnemyHealth};
use crate::game::weapon::components::{Weapon, WeaponSettings};
use crate::resources::Player;
use bevy::color::palettes::basic::WHITE;
use bevy::prelude::*;
use bevy_egui::egui::{Color32, CursorIcon, Style, TextStyle};
use bevy_egui::{egui, EguiContexts};

pub fn set_style(mut contexts: EguiContexts) {
    let context = contexts.ctx_mut();

    let mut style = Style::default();
    style.text_styles.get_mut(&TextStyle::Small).unwrap().size = 24.;
    style.text_styles.get_mut(&TextStyle::Heading).unwrap().size = 60.;
    context.set_style(style);
}

pub fn setup_map(mut commands: Commands, player: Res<Player>, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite {
            image: asset_server.load("map/map.png"),
            custom_size: Some(MAP_SIZE),
            ..default()
        },
        Transform::from_xyz(
            -WEAPONS_PANEL_SIZE.x * 0.5,
            (WALL_SIZE.y + RESOURCES_PANEL_SIZE.y) * 0.5,
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
            (-MAP_SIZE.y + WALL_SIZE.y) * 0.5,
            0.1,
        ),
        Wall,
    ));

    // Spawn hidden pause banner
    commands
        .spawn((
            Sprite {
                color: Color::srgba(0., 0., 0., 0.3),
                custom_size: Some(Vec2::new(SIZE.x * 0.1, SIZE.y * 0.1)),
                ..default()
            },
            Transform::from_xyz(
                -WEAPONS_PANEL_SIZE.x * 0.5,
                RESOURCES_PANEL_SIZE.y * 0.5,
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

    // Spawn weapons ============================================== >>

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
            Transform::from_xyz(pos, (-MAP_SIZE.y + WALL_SIZE.y) * 0.5, 2.0),
            weapon.clone(),
        ));
    }
}

pub fn resources_panel(
    mut contexts: EguiContexts,
    mut player: ResMut<Player>,
    images: Local<Images>,
) {
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
                    fortress_texture,
                    [20., 20.],
                )));
                ui.add(
                    egui::ProgressBar::new(
                        player.wall.health as f32 / player.wall.max_health as f32,
                    )
                    .desired_width(200.)
                    .desired_height(20.)
                    .text(format!("{} / {}", player.wall.health, player.wall.max_health))
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
) {
    egui::SidePanel::right("Weapons panel")
        .exact_width(WEAPONS_PANEL_SIZE.x)
        .resizable(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.add_space(5.);
            ui.vertical_centered(|ui| {
                ui.heading("Weapon settings");
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
}

pub fn map_update(
    enemy_q: Query<(&Enemy, Entity)>,
    children_q: Query<&Children>,
    mut health_q: Query<(&mut Transform, &mut Sprite), With<EnemyHealth>>,
) {
    // Update enemy health bars
    for (enemy, entity) in enemy_q.iter() {
        if enemy.health < enemy.max_health {
            for child in children_q.iter_descendants(entity) {
                if let Ok((mut transform, mut sprite)) = health_q.get_mut(child) {
                    if let Some(size) = sprite.custom_size.as_mut() {
                        let full_size = enemy.size.x * 0.8 - 2.0;
                        size.x = full_size * enemy.health as f32 / enemy.max_health as f32;
                        transform.translation.x = (size.x - full_size) * 0.5;
                    }
                }
            }
        }
    }
}
