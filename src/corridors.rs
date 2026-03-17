use serde::{Deserialize, Serialize};
use crate::geom_material::GeomMaterialSpec;
use crate::lyapunov::{StructuralResidual, structural_residual};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeciesBands {
    pub child: GapBand,
    pub rat: GapBand,
    pub pigeon: GapBand,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GapBand {
    pub safe_max: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialBands {
    pub rat: MaterialBand,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialBand {
    pub safe_min: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuralBand {
    pub safe_min: f64,
    pub safe_max: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorridorBands {
    pub species: SpeciesBands,
    pub material: MaterialBands,
    pub structural: StructuralBand,
}

impl CorridorBands {
    pub fn phoenix_default() -> Self {
        Self {
            species: SpeciesBands {
                child: GapBand { safe_max: 0.5 },
                rat: GapBand { safe_max: 0.5 },
                pigeon: GapBand { safe_max: 0.5 },
            },
            material: MaterialBands {
                rat: MaterialBand { safe_min: 1.0 },
            },
            structural: StructuralBand {
                safe_min: 0.0,
                safe_max: 1.0,
            },
        }
    }

    pub fn from_file(path: &str) -> Result<Self, std::io::Error> {
        let content = std::fs::read_to_string(path)?;
        if path.ends_with(".yaml") || path.ends_with(".yml") {
            let bands: Self = serde_yaml::from_str(&content)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
            Ok(bands)
        } else {
            let bands: Self = serde_json::from_str(&content)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
            Ok(bands)
        }
    }
}

#[derive(Debug, Clone)]
pub struct CorridorEval {
    pub rgap_child: f64,
    pub rgap_rat: f64,
    pub rgap_pigeon: f64,
    pub rmat_rat: f64,
    pub v_struct: f64,
    pub accepted: bool,
}

#[derive(Debug, Clone)]
pub struct CorridorEvaluator {
    pub bands: CorridorBands,
}

impl CorridorEvaluator {
    pub fn new(bands: CorridorBands) -> Self {
        Self { bands }
    }

    fn compute_rgap(gap: f64, species_min: f64) -> f64 {
        if species_min <= 0.0 {
            return f64::INFINITY;
        }
        gap / species_min
    }

    fn compute_rmat(spec: &GeomMaterialSpec) -> f64 {
        let bite_ratio = if spec.rat_bite_cycles_required == 0 {
            0.0
        } else {
            spec.rat_bite_cycles_survived as f64 / spec.rat_bite_cycles_required as f64
        };
        let fatigue_ratio = if spec.fatigue_cycles_required == 0 {
            0.0
        } else {
            spec.fatigue_cycles_survived as f64 / spec.fatigue_cycles_required as f64
        };
        let corrosion_ratio = if spec.design_life_years <= 0.0 {
            0.0
        } else {
            spec.corrosion_margin_years / spec.design_life_years
        };
        let bite_clamped = bite_ratio.min(2.0);
        let fatigue_clamped = fatigue_ratio.min(2.0);
        let corrosion_clamped = corrosion_ratio.min(2.0);
        0.5 * bite_clamped + 0.3 * fatigue_clamped + 0.2 * corrosion_clamped
    }

    pub fn evaluate(&self, spec: &GeomMaterialSpec) -> CorridorEval {
        let rgap_child = Self::compute_rgap(spec.clear_gap_m, spec.species_min_dim_child_m);
        let rgap_rat = Self::compute_rgap(spec.clear_gap_m, spec.species_min_dim_rat_m);
        let rgap_pigeon = Self::compute_rgap(spec.clear_gap_m, spec.species_min_dim_pigeon_m);
        let rmat_rat = Self::compute_rmat(spec);

        let v_struct = structural_residual(
            rgap_child,
            rgap_rat,
            rgap_pigeon,
            rmat_rat,
            &self.bands,
        );

        let accepted = rgap_child < self.bands.species.child.safe_max
            && rgap_rat < self.bands.species.rat.safe_max
            && rgap_pigeon < self.bands.species.pigeon.safe_max
            && rmat_rat >= self.bands.material.rat.safe_min
            && v_struct >= self.bands.structural.safe_min
            && v_struct <= self.bands.structural.safe_max;

        CorridorEval {
            rgap_child,
            rgap_rat,
            rgap_pigeon,
            rmat_rat,
            v_struct,
            accepted,
        }
    }
}
