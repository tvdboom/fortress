use bevy::prelude::*;

pub struct Images {
    pub health: Handle<Image>,
    pub fortress: Handle<Image>,
    pub bullets: Handle<Image>,
    pub gasoline: Handle<Image>,
    pub materials: Handle<Image>,
}

impl FromWorld for Images {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        Self {
            health: asset_server.load("map/heart.png"),
            fortress: asset_server.load("map/fortress.png"),
            bullets: asset_server.load("map/bullet.png"),
            gasoline: asset_server.load("map/gasoline.png"),
            materials: asset_server.load("map/brick.png"),
        }
    }
}

#[derive(Component)]
pub struct Map;

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct HealthText;

#[derive(Component)]
pub struct BulletsText;

#[derive(Component)]
pub struct GasolineText;

#[derive(Component)]
pub struct MaterialsText;
