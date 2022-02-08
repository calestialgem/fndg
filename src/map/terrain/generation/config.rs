use super::Weight;
use crate::map::terrain::Terrains;
use bracket_noise::prelude::{FastNoise, FractalType, NoiseType};
use serde::{Deserialize, Serialize};

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

/// [Terrain] name and non-normalized weight pair to distribute.
#[derive(Serialize, Deserialize)]
pub(super) struct WeightConfig {
    pub(super) name: String,
    pub(super) weight: f32,
}

impl WeightConfig {
    pub(super) fn create(&self, terrains: &Terrains, total_weight: f32) -> Weight {
        Weight::new(terrains, &self.name, self.weight / total_weight)
    }
}

/// Non-normalized distribution of weights.
#[derive(Serialize, Deserialize)]
pub(super) struct DistributeConfig(pub(super) Vec<WeightConfig>);

/// Noise to [Terrain] distributor configuration.
#[derive(Serialize, Deserialize)]
pub(super) struct SelectConfig {
    pub(super) noise: NoiseConfig,
    pub(super) distribution: DistributeConfig,
}
