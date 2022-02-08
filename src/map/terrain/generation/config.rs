use super::{Distribute, Noise, Select, Weight};
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
    pub(super) fn create(&self) -> Noise {
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
        Noise {
            noise,
            scale: self.scale,
        }
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

impl DistributeConfig {
    pub(super) fn create(&self, terrains: &Terrains) -> Distribute {
        let total_weight = self.total_weight();
        Distribute {
            distribution: self
                .0
                .iter()
                .map(|config| config.create(terrains, total_weight))
                .collect(),
        }
    }

    fn total_weight(&self) -> f32 {
        self.0.iter().map(|config| config.weight).sum()
    }
}

/// Noise to [Terrain] distributor configuration.
#[derive(Serialize, Deserialize)]
pub(super) struct SelectConfig {
    pub(super) noise: NoiseConfig,
    pub(super) distribute: DistributeConfig,
}

impl SelectConfig {
    pub(super) fn create(&self, terrains: &Terrains) -> Select {
        Select {
            noise: self.noise.create(),
            distribute: self.distribute.create(terrains),
        }
    }
}
