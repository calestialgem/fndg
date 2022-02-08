use bracket_noise::prelude::{FastNoise, FractalType, NoiseType};
use serde::{Deserialize, Serialize};

use crate::map::terrain::Terrains;

use super::TerrainWeight;

/// Noise configuration.
#[derive(Serialize, Deserialize)]
pub(super) struct NoiseConfig {
    pub(super) simplex: bool,
    pub(super) octaves: i32,
    pub(super) gain: f32,
    pub(super) lacunarity: f32,
    pub(super) frequency: f32,
    pub(super) scale: f32,
}

impl NoiseConfig {
    pub(super) fn noise(&self) -> FastNoise {
        let mut noise = FastNoise::seeded(rand::random());
        noise.set_noise_type(if self.simplex {
            NoiseType::SimplexFractal
        } else {
            NoiseType::PerlinFractal
        });
        noise.set_fractal_type(FractalType::FBM);
        noise.set_fractal_octaves(self.octaves);
        noise.set_fractal_gain(self.gain);
        noise.set_fractal_lacunarity(self.lacunarity);
        noise.set_frequency(self.frequency);
        noise
    }
}

/// [Terrain] name and weight pair to distribute.
#[derive(Serialize, Deserialize)]
pub(super) struct TerrainWeightConfig {
    pub(super) name: String,
    pub(super) weight: f32,
}

impl TerrainWeightConfig {
    pub(super) fn create(&self, terrains: &Terrains, total_weight: f32) -> TerrainWeight {
        TerrainWeight::new(terrains, &self.name, self.weight / total_weight)
    }
}

#[derive(Serialize, Deserialize)]
pub(super) struct Distribution(pub(super) Vec<TerrainWeightConfig>);

/// Noise to [Terrain] distributor configuration.
#[derive(Serialize, Deserialize)]
pub(super) struct DistributorConfig {
    pub(super) noise: NoiseConfig,
    pub(super) distribution: Distribution,
}
