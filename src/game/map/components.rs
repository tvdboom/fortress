use bevy::prelude::*;

pub struct Images {
    pub day_night: Handle<Image>,
    pub day: Handle<Image>,
    pub night: Handle<Image>,
    pub person: Handle<Image>,
    pub fortress: Handle<Image>,
    pub fence: Handle<Image>,
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
            day_night: asset_server.load("map/day-and-night.png"),
            day: asset_server.load("map/day.png"),
            night: asset_server.load("map/night.png"),
            person: asset_server.load("map/person.png"),
            fortress: asset_server.load("map/fortress.png"),
            fence: asset_server.load("map/fence.png"),
            weapon: asset_server.load("map/rifle.png"),
            bullets: asset_server.load("map/bullet.png"),
            gasoline: asset_server.load("map/gasoline.png"),
            materials: asset_server.load("map/brick.png"),
            hourglass: asset_server.load("map/hourglass.png"),
            clock: asset_server.load("map/clock.png"),
            spot: asset_server.load("map/spot.png"),
            game_over: asset_server.load("map/game-over.png"),
        }
    }
}

#[derive(Component)]
pub struct Map;

#[derive(Component)]
pub struct Wall;
