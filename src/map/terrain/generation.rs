mod config;

pub(crate) use self::config::GenerationConfig;

use self::config::{DistributorConfig, TerrainWeightConfig};
use super::Terrains;
use hex_grid::{Coordinate, Offset, CENTER};
use perlin2d::PerlinNoise2D;
use std::collections::HashMap;

/// [Terrain] and weight pair to distribute.
struct TerrainWeight {
    terrain: usize,
    weight: f64,
}

impl TerrainWeight {
    const BY_HUMIDITY: usize = usize::MAX;

    fn new(terrains: &Terrains, config: &TerrainWeightConfig) -> Self {
        if config.name == "BY_HUMIDITY" {
            TerrainWeight {
                terrain: Self::BY_HUMIDITY,
                weight: config.weight,
            }
        } else {
            TerrainWeight {
                terrain: terrains.of_name(&config.name).id(),
                weight: config.weight,
            }
        }
    }
}

struct Distributor {
    noise: PerlinNoise2D,
    distribution: Vec<TerrainWeight>,
}

impl Distributor {
    fn new(terrains: &Terrains, config: &DistributorConfig) -> Self {
        let mut weights = Vec::new();
        let mut amplitude = 0.0;
        for weight in config.distribution.iter() {
            weights.push(TerrainWeight::new(terrains, weight));
            amplitude += weight.weight;
        }
        Distributor {
            noise: config.noise.perlin(amplitude),
            distribution: weights,
        }
    }

    fn noise(&self, coord: Coordinate) -> f64 {
        self.noise.get_noise(coord.x as f64, coord.y as f64)
    }

    fn distribute(&self, mut value: f64) -> usize {
        for TerrainWeight { terrain, weight } in self.distribution.iter() {
            value -= weight;
            if value <= 0.0 {
                return *terrain;
            }
        }
        unreachable!("`value` is too big!");
    }
}

/// Generates; a [Map] from Perlin noise.
pub(crate) struct Generator {
    radius: u16,
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
        let coords = CENTER + Offset::fill_hex(self.radius);
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
