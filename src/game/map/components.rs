use bevy::prelude::*;

pub struct Images {
    pub bug: Handle<Image>,
    pub day: Handle<Image>,
    pub night: Handle<Image>,
    pub person: Handle<Image>,
    pub wall: Handle<Image>,
    pub fence: Handle<Image>,
    pub lightning: Handle<Image>,
    pub weapon: Handle<Image>,
    pub bullets: Handle<Image>,
    pub gasoline: Handle<Image>,
    pub materials: Handle<Image>,
    pub spot: Handle<Image>,
    pub hourglass: Handle<Image>,
    pub clock: Handle<Image>,
    pub game_over: Handle<Image>,
}

impl FromWorld for Images {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        Self {
            bug: asset_server.load("icons/bug.png"),
            day: asset_server.load("icons/day.png"),
            night: asset_server.load("icons/night.png"),
            person: asset_server.load("icons/user.png"),
            wall: asset_server.load("icons/wall.png"),
            fence: asset_server.load("icons/fence.png"),
            lightning: asset_server.load("icons/lightning.png"),
            weapon: asset_server.load("icons/rifle.png"),
            bullets: asset_server.load("icons/bullet.png"),
            gasoline: asset_server.load("icons/gasoline.png"),
            materials: asset_server.load("icons/brick.png"),
            hourglass: asset_server.load("icons/hourglass.png"),
            clock: asset_server.load("icons/clock.png"),
            spot: asset_server.load("icons/spot.png"),
            game_over: asset_server.load("map/game-over.png"),
        }
    }
}

#[derive(Component)]
pub struct Map;

#[derive(Component)]
pub struct Fence;

#[derive(Component)]
pub struct Wall;
