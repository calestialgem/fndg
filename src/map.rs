//! Stuff about the game map.

mod terrain;

use self::terrain::{
    generation::{GenerationConfig, Generator},
    Terrain, Terrains,
};
use bevy::{
    math::Vec3,
    prelude::{
        App, Bundle, Commands, Component, Entity, OrthographicCameraBundle, Plugin, Res, ResMut,
        Transform,
    },
    sprite::SpriteBundle,
};
use hex2d::{Coordinate, Spacing};
use std::collections::HashMap;

/// Location in the [Map].
#[derive(Component)]
struct Location {
    coord: Coordinate,
}

impl Location {
    const SPACING: Spacing = Spacing::PointyTop(1.0);

    fn to_vec3(coord: Coordinate) -> Vec3 {
        let pixel = coord.to_pixel(Self::SPACING);
        Vec3::new(pixel.0, pixel.1, 0.0)
    }
}

/// Smallest piece of a [Map].
#[derive(Component)]
struct Tile {
    terrain: usize,
}

impl Tile {
    /// Returns this tile's terrain from the given terrains. This must be the
    /// same terrains that was used in the generation of the tile.
    fn terrain<'a>(&self, terrains: &'a Terrains) -> &'a Terrain {
        terrains.of_id(self.terrain)
    }
}

#[derive(Bundle)]
struct TileBundle {
    #[bundle]
    sprite: SpriteBundle,
    location: Location,
    tile: Tile,
}

impl TileBundle {
    fn new(coord: Coordinate, terrain: usize, terrains: &Terrains) -> Self {
        TileBundle {
            sprite: SpriteBundle {
                sprite: bevy::sprite::Sprite {
                    color: *terrains.of_id(terrain).color(),
                    ..Default::default()
                },
                transform: Transform {
                    translation: Location::to_vec3(coord),
                    scale: Vec3::new(1.0, 1.0, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            location: Location { coord },
            tile: Tile { terrain },
        }
    }
}

/// The game world.
#[derive(Default)]
struct Map {
    tiles: HashMap<Coordinate, Entity>,
}

pub(super) struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Terrains::new());
        app.insert_resource(Map::default());
        app.add_startup_system(create_camera);
        app.add_startup_system(generate_map);
    }

    fn name(&self) -> &str {
        "Fndg::Map"
    }
}

fn create_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn generate_map(mut map: ResMut<Map>, terrains: Res<Terrains>, mut commands: Commands) {
    let terrain_map = Generator::new(&terrains, &GenerationConfig::new()).generate();
    for (coord, terrain) in terrain_map.into_iter() {
        let tile = commands
            .spawn_bundle(TileBundle::new(coord, terrain, &terrains))
            .id();
        map.tiles.insert(coord, tile);
    }
}
