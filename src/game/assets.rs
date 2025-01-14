use bevy::asset::{AssetServer, Handle};
use bevy::image::Image;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Clone)]
pub struct AtlasInfo {
    pub image: Handle<Image>,
    pub texture: TextureAtlas,
    pub last_index: usize,
}

pub struct WorldAssets {
    pub images: HashMap<&'static str, Handle<Image>>,
    pub atlas: HashMap<&'static str, AtlasInfo>,
}

impl WorldAssets {
    pub fn get_image(&self, name: &str) -> Handle<Image> {
        self.images[name].clone_weak()
    }

    pub fn get_atlas(&self, name: &str) -> AtlasInfo {
        self.atlas[name].clone()
    }
}

impl FromWorld for WorldAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.get_resource::<AssetServer>().unwrap();
        let images = HashMap::from([
            // Icons
            ("day", assets.load("icons/day.png")),
            ("night", assets.load("icons/night.png")),
            ("person", assets.load("icons/user.png")),
            ("wall", assets.load("icons/wall.png")),
            ("fence", assets.load("icons/fence.png")),
            ("lightning", assets.load("icons/lightning.png")),
            ("mine", assets.load("icons/mine.png")),
            ("bomb", assets.load("icons/bomb.png")),
            ("nuke", assets.load("icons/nuke.png")),
            ("spotlight", assets.load("icons/spotlight.png")),
            ("bulb", assets.load("icons/bulb.png")),
            ("weapon", assets.load("icons/rifle.png")),
            ("bullets", assets.load("icons/bullet.png")),
            ("gasoline", assets.load("icons/gasoline.png")),
            ("materials", assets.load("icons/brick.png")),
            ("spot", assets.load("icons/spot.png")),
            ("hourglass", assets.load("icons/hourglass.png")),
            ("clock", assets.load("icons/clock.png")),
            ("game_over", assets.load("map/game-over.png")),
            // Enemies
            ("Dartling", assets.load("enemy/dartling.png")),
            ("Skitterling", assets.load("enemy/skitterling.png")),
            ("Shellback", assets.load("enemy/shellback.png")),
            ("Quickstrike", assets.load("enemy/quickstrike.png")),
            ("Chiton", assets.load("enemy/chiton.png")),
            ("Thornbiter", assets.load("enemy/thornbiter.png")),
            ("Needler", assets.load("enemy/needler.png")),
            ("Blightcraw", assets.load("enemy/blightcraw.png")),
            ("Shellfist", assets.load("enemy/shellfist.png")),
            ("Shellwarden", assets.load("enemy/shellwarden.png")),
            ("Hiveborn", assets.load("enemy/hiveborn.png")),
            ("Crawler", assets.load("enemy/crawler.png")),
            ("Carapacebreaker", assets.load("enemy/carapacebreaker.png")),
            ("Dartmite", assets.load("enemy/dartmite.png")),
            ("Nestling", assets.load("enemy/nestling.png")),
            ("Gargantula", assets.load("enemy/gargantula.png")),
            ("Ironclaw", assets.load("enemy/ironclaw.png")),
            ("Ironcarapace", assets.load("enemy/ironcarapace.png")),
            // Sprite sheets
            ("flashes", assets.load("weapon/flashes.png")),
            ("flame", assets.load("weapon/flame.png")),
            ("explosion1", assets.load("weapon/explosion1.png")),
            ("explosion2", assets.load("weapon/explosion2.png")),
        ]);

        let mut texture = world
            .get_resource_mut::<Assets<TextureAtlasLayout>>()
            .unwrap();

        let single_flash = TextureAtlasLayout::from_grid(UVec2::new(88, 100), 5, 6, None, None);
        let triple_flash = TextureAtlasLayout::from_grid(UVec2::new(100, 117), 4, 5, None, None);
        let cone_flash = TextureAtlasLayout::from_grid(UVec2::new(107, 105), 4, 6, None, None);
        let wide_flash = TextureAtlasLayout::from_grid(UVec2::new(97, 150), 4, 4, None, None);
        let flame = TextureAtlasLayout::from_grid(UVec2::new(124, 50), 1, 12, None, None);
        let explosion1 = TextureAtlasLayout::from_grid(UVec2::new(256, 256), 8, 6, None, None);
        let explosion2 = TextureAtlasLayout::from_grid(UVec2::new(257, 252), 8, 6, None, None);

        let atlas = HashMap::from([
            (
                "single-flash",
                AtlasInfo {
                    image: images["flashes"].clone_weak(),
                    texture: TextureAtlas {
                        layout: texture.add(single_flash),
                        index: 25,
                    },
                    last_index: 30,
                },
            ),
            (
                "cone-flash",
                AtlasInfo {
                    image: images["flashes"].clone_weak(),
                    texture: TextureAtlas {
                        layout: texture.add(cone_flash),
                        index: 17,
                    },
                    last_index: 20,
                },
            ),
            (
                "triple-flash",
                AtlasInfo {
                    image: images["flashes"].clone_weak(),
                    texture: TextureAtlas {
                        layout: texture.add(triple_flash),
                        index: 9,
                    },
                    last_index: 12,
                },
            ),
            (
                "wide-flash",
                AtlasInfo {
                    image: images["flashes"].clone_weak(),
                    texture: TextureAtlas {
                        layout: texture.add(wide_flash),
                        index: 1,
                    },
                    last_index: 4,
                },
            ),
            (
                "flame",
                AtlasInfo {
                    image: images["flame"].clone_weak(),
                    texture: TextureAtlas {
                        layout: texture.add(flame),
                        index: 1,
                    },
                    last_index: 12,
                },
            ),
            (
                "explosion1",
                AtlasInfo {
                    image: images["explosion1"].clone_weak(),
                    texture: TextureAtlas {
                        layout: texture.add(explosion1),
                        index: 1,
                    },
                    last_index: 48,
                },
            ),
            (
                "explosion2",
                AtlasInfo {
                    image: images["explosion2"].clone_weak(),
                    texture: TextureAtlas {
                        layout: texture.add(explosion2),
                        index: 1,
                    },
                    last_index: 32,
                },
            ),
        ]);

        Self { images, atlas }
    }
}
