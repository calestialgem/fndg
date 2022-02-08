mod config;

use self::config::{SelectConfig, WeightConfig};
use super::Terrains;
use bracket_noise::prelude::FastNoise;
use hex2d::Coordinate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

struct Noise {
    noise: FastNoise,
    scale: f32,
}

impl Noise {
    fn noise(&self, coord: Coordinate) -> f32 {
        self.noise
            .get_noise(coord.x as f32 / self.scale, coord.y as f32 / self.scale)
    }
}

/// [Terrain] and weight pair to distribute.
struct Weight {
    terrain: usize,
    weight: f32,
}

impl Weight {
    const BY_HUMIDITY: usize = usize::MAX;

    fn new(terrains: &Terrains, name: &String, weight: f32) -> Self {
        if name == "BY_HUMIDITY" {
            Weight {
                terrain: Self::BY_HUMIDITY,
                weight,
            }
        } else {
            Weight {
                terrain: terrains.of_name(name).id(),
                weight,
            }
        }
    }
}

struct Distribute {
    distribution: Vec<Weight>,
}

impl Distribute {
    fn distribute(&self, value: f32) -> usize {
        let mut remaining = value;
        for Weight { terrain, weight } in self.distribution.iter() {
            remaining -= weight;
            if remaining <= 0.0 {
                return *terrain;
            }
        }
        unreachable!("`value` {} is too big! Remaining: {}", value, remaining);
    }
}

struct Select {
    noise: Noise,
    distribute: Distribute,
}

impl Select {
    fn new(terrains: &Terrains, config: &SelectConfig) -> Self {
        let total_weight = {
            let mut total_weight = 0.0;
            for WeightConfig { name: _, weight } in config.distribution.0.iter() {
                total_weight += weight;
            }
            total_weight
        };
        let distribution = {
            let mut distribution = Vec::new();
            for weight in config.distribution.0.iter() {
                distribution.push(weight.create(terrains, total_weight));
            }
            distribution
        };
        Select {
            noise: Noise {
                noise: config.noise.noise(),
                scale: config.noise.scale,
            },
            distribute: Distribute { distribution },
        }
    }
}

/// Generates; a [Map] from Perlin noise.
pub(crate) struct Generate {
    radius: i32,
    height: Select,
    humidity: Select,
}

impl Generate {
    pub(crate) fn new(terrains: &Terrains, config: &Config) -> Self {
        Generate {
            radius: config.radius,
            height: Select::new(terrains, &config.height),
            humidity: Select::new(terrains, &config.humidity),
        }
    }

    pub(crate) fn generate(&self) -> HashMap<Coordinate, usize> {
        let coords = Coordinate::new(0, 0).range_iter(self.radius);
        let mut map = HashMap::new();
        for coord in coords {
            map.insert(coord, self.generate_tile(coord));
        }
        map
    }

    fn generate_tile(&self, coord: Coordinate) -> usize {
        let from_height = self
            .height
            .distribute
            .distribute(self.height.noise.noise(coord));
        if from_height == Weight::BY_HUMIDITY {
            self.humidity
                .distribute
                .distribute(self.humidity.noise.noise(coord))
        } else {
            from_height
        }
    }
}

/// Configuration of [Terrain] generation.
#[derive(Serialize, Deserialize)]
pub(crate) struct Config {
    radius: i32,
    height: SelectConfig,
    humidity: SelectConfig,
}

impl Config {
    const FILE: &'static str = "assets/terrain_generation.json";

    fn from_file(file: &str) -> Self {
        Self::from_json(&fs::read_to_string(file).unwrap())
    }

    fn from_json(json: &str) -> Self {
        serde_json::from_str(json).unwrap()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::from_file(Self::FILE)
    }
}
