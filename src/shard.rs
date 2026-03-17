use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use crate::geom_material::GeomMaterialSpec;
use crate::corridors::CorridorEval;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShardMeta {
    pub shard_id: String,
    pub node_id: String,
    pub timestamp_utc: DateTime<Utc>,
    pub lat: f64,
    pub lon: f64,
    pub hex_proof: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuralCoords {
    pub rgap_child: f64,
    pub rgap_rat: f64,
    pub rgap_pigeon: f64,
    pub rmat_rat: f64,
    pub v_struct: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosafetyCoords {
    pub v_t: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KE RMetrics {
    pub m_help_kg_year: f64,
    pub e_hybrid_mwh_year: f64,
    pub k_score: f64,
    pub e_score: f64,
    pub r_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindHydroShard {
    pub meta: ShardMeta,
    pub structural: StructuralCoords,
    pub ecosafety: EcosafetyCoords,
    pub k_e_r: KERMetrics,
    pub geom_material: GeomMaterialSpec,
}

impl WindHydroShard {
    pub fn from_eval(meta: ShardMeta, spec: &GeomMaterialSpec, eval: &CorridorEval) -> Self {
        let v_t = 0.6 * eval.v_struct + 0.4 * 0.5;
        let m_help = spec.projected_macro_litter_capture_kg_year;
        let e_hybrid = spec.projected_energy_hybrid_mwh_year;
        let k_score = (m_help / 150.0).min(1.0);
        let e_score = (e_hybrid / 1.0).min(1.0);
        let r_score = eval.v_struct;

        Self {
            meta,
            structural: StructuralCoords {
                rgap_child: eval.rgap_child,
                rgap_rat: eval.rgap_rat,
                rgap_pigeon: eval.rgap_pigeon,
                rmat_rat: eval.rmat_rat,
                v_struct: eval.v_struct,
            },
            ecosafety: EcosafetyCoords {
                v_t,
            },
            k_e_r: KERMetrics {
                m_help_kg_year: m_help,
                e_hybrid_mwh_year: e_hybrid,
                k_score,
                e_score,
                r_score,
            },
            geom_material: spec.clone(),
        }
    }
}
