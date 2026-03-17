use serde::{Serialize, Deserialize};
use crate::corridors::CorridorBands;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuralResidual {
    pub v_struct: f64,
}

pub fn structural_residual(
    rgap_child: f64,
    rgap_rat: f64,
    rgap_pigeon: f64,
    rmat_rat: f64,
    bands: &CorridorBands,
) -> f64 {
    let r_child = (rgap_child / bands.species.child.safe_max).max(0.0);
    let r_rat = (rgap_rat / bands.species.rat.safe_max).max(0.0);
    let r_pigeon = (rgap_pigeon / bands.species.pigeon.safe_max).max(0.0);
    let r_mat = if rmat_rat <= 0.0 {
        10.0
    } else {
        bands.material.rat.safe_min / rmat_rat
    };
    let w_child = 0.35;
    let w_rat = 0.25;
    let w_pigeon = 0.15;
    let w_mat = 0.25;
    let mut v = w_child * r_child + w_rat * r_rat + w_pigeon * r_pigeon + w_mat * r_mat;
    if v < 0.0 {
        v = 0.0;
    }
    if v > 2.0 {
        v = 2.0;
    }
    v
}
