mod config;

pub(crate) use self::config::GenerationConfig;

use self::config::{DistributorConfig, TerrainWeightConfig};
use super::Terrains;
use bracket_noise::prelude::FastNoise;
use hex2d::Coordinate;
use std::collections::HashMap;

/// [Terrain] and weight pair to distribute.
struct TerrainWeight {
    terrain: usize,
    weight: f32,
}

impl TerrainWeight {
    const BY_HUMIDITY: usize = usize::MAX;

    fn new(terrains: &Terrains, config: &TerrainWeightConfig, total_weight: f32) -> Self {
        let weight = config.weight / total_weight;
        if config.name == "BY_HUMIDITY" {
            TerrainWeight {
                terrain: Self::BY_HUMIDITY,
                weight,
            }
        } else {
            TerrainWeight {
                terrain: terrains.of_name(&config.name).id(),
                weight,
            }
        }
    }
}

struct Distributor {
    noise: FastNoise,
    scale: f32,
    distribution: Vec<TerrainWeight>,
}

impl Distributor {
    fn new(terrains: &Terrains, config: &DistributorConfig) -> Self {
        let total_weight = {
            let mut total_weight = 0.0;
            for TerrainWeightConfig { name: _, weight } in config.distribution.iter() {
                total_weight += weight;
            }
            total_weight
        };
        let distribution = {
            let mut distribution = Vec::new();
            for weight in config.distribution.iter() {
                distribution.push(TerrainWeight::new(terrains, weight, total_weight));
            }
            distribution
        };
        let noise = config.noise.noise();
        Distributor {
            noise: noise.0,
            scale: noise.1,
            distribution,
        }
    }

    fn noise(&self, coord: Coordinate) -> f32 {
        self.noise
            .get_noise(coord.x as f32 / self.scale, coord.y as f32 / self.scale)
    }

    fn distribute(&self, value: f32) -> usize {
        let mut remaining = value;
        for TerrainWeight { terrain, weight } in self.distribution.iter() {
            remaining -= weight;
            if remaining <= 0.0 {
                return *terrain;
            }
        }
        remaining = value;
        for TerrainWeight { terrain: _, weight } in self.distribution.iter() {
            println!("Remaining: {}", remaining);
            remaining -= weight;
        }
        unreachable!("`value` {} is too big! Remaining: {}", value, remaining);
    }
}

/// Generates; a [Map] from Perlin noise.
pub(crate) struct Generator {
    radius: i32,
    height: Distributor,
    humidity: Distributor,
}

impl Generator {
    pub(crate) fn new(terrains: &Terrains, config: &GenerationConfig) -> Self {
        Generator {
            radius: config.radius,
            height: Distributor::new(terrains, &config.height),
            humidity: Distributor::new(terrains, &config.humidity),
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
        let from_height = self.height.distribute(self.height.noise(coord));
        if from_height == TerrainWeight::BY_HUMIDITY {
            self.humidity.distribute(self.humidity.noise(coord))
        } else {
            from_height
        }
    }
}
