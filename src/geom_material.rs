use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeomMaterialSpec {
    pub clear_gap_m: f64,
    pub bar_diameter_m: f64,
    pub bar_spacing_m: f64,
    pub species_min_dim_child_m: f64,
    pub species_min_dim_rat_m: f64,
    pub species_min_dim_pigeon_m: f64,
    pub alloy_name: String,
    pub mesh_gauge_mm: f64,
    pub rat_bite_cycles_survived: u32,
    pub rat_bite_cycles_required: u32,
    pub fatigue_cycles_survived: u64,
    pub fatigue_cycles_required: u64,
    pub corrosion_margin_years: f64,
    pub design_life_years: f64,
    pub projected_macro_litter_capture_kg_year: f64,
    pub projected_energy_hybrid_mwh_year: f64,
}
