//! Stuff about the game map.

mod terrain;

use self::terrain::{generation::Config, Terrain, Terrains};
use bevy::{
    core_pipeline::ClearColor,
    math::Vec3,
    prelude::{
        App, AssetServer, Bundle, Color, Commands, Component, Entity, EventReader, EventWriter,
        Handle, Image, OrthographicCameraBundle, Plugin, Res, ResMut, Transform,
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
    const SIZE: f32 = 4.0;
    const SPACING: Spacing = Spacing::PointyTop(Self::SIZE / 2.0);

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
    fn new(coord: Coordinate, terrain: usize, terrains: &Terrains, texture: &TileTexture) -> Self {
        TileBundle {
            sprite: SpriteBundle {
                sprite: bevy::sprite::Sprite {
                    color: *terrains.of_id(terrain).color(),
                    ..Default::default()
                },
                transform: Transform {
                    translation: Location::to_vec3(coord),
                    scale: Vec3::new(
                        Location::SIZE / TileTexture::SIZE,
                        Location::SIZE / TileTexture::SIZE,
                        1.0,
                    ),
                    ..Default::default()
                },
                texture: texture.0.clone(),
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
        app.add_event::<MapGenEvent>();
        app.insert_resource(Terrains::default());
        app.insert_resource(Map::default());
        app.insert_resource(ClearColor(Color::rgb(0.94, 0.97, 1.0)));
        app.add_startup_system(create_camera);
        app.add_startup_system(load_tile_texture);
        app.add_startup_system(generate_initial_map);
        app.add_system(generate_map);
    }

    fn name(&self) -> &str {
        "Fndg::Map"
    }
}

fn create_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

struct TileTexture(Handle<Image>);

impl TileTexture {
    const SIZE: f32 = 256.0;
}

fn load_tile_texture(mut commands: Commands, server: Res<AssetServer>) {
    commands.insert_resource(TileTexture(server.load("tile.png")));
}

fn generate_initial_map(mut gen_event: EventWriter<MapGenEvent>) {
    gen_event.send(MapGenEvent);
}

pub(crate) struct MapGenEvent;

fn generate_map(
    mut commands: Commands,
    mut map: ResMut<Map>,
    mut gen_event: EventReader<MapGenEvent>,
    terrains: Res<Terrains>,
    texture: Res<TileTexture>,
) {
    if gen_event.iter().next().is_some() {
        for (_, tile) in map.tiles.iter() {
            commands.entity(*tile).despawn();
        }
        map.tiles.clear();
        let terrain_map = Config::default().create(&terrains).generate();
        for (coord, terrain) in terrain_map.into_iter() {
            let tile = commands
                .spawn_bundle(TileBundle::new(coord, terrain, &terrains, &texture))
                .id();
            map.tiles.insert(coord, tile);
        }
    }
}
