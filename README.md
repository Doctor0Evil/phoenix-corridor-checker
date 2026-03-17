# Phoenix Corridor Checker

`phoenix-corridor-checker` is a Rust CLI that implements a Phoenix-specific **Hardware Corridor Checker** for WindHydro and windnet nodes, emitting qpudatashards with structural, material, and ecosafety coordinates wired to a Lyapunov-style residual spine.[web:9]

The tool ingests CAD-exported geometry/material specs, computes normalized structural/material invariants (`r_gap`, `r_mat`, `V_struct`), and writes provisional ALN-compatible shard files (e.g., `qpudatashards/WindHydroPhoenix2026v1.aln`) that can be consumed by EcoNet validators and `econet-stake-terminal`-style edge agents.[web:9]

## Features

- Species-specific geometric corridors for children, rats, and pigeons using normalized gap ratios `r_gap,child`, `r_gap,rat`, `r_gap,pigeon` as shard invariants with “no corridor, no build” semantics.[web:9]
- Material corridor axis `r_mat,rat` that encodes rat-bite surrogate tests, cyclic loading endurance, and corrosion margins, constrained to proven worst-case survival envelopes.[web:9]
- Structural residual `V_struct` implemented as a Lyapunov-style function over normalized coordinates, clamped into shard-level safe bands for CI and runtime safestep gating.[web:9]
- Approximate ecosafety residual proxy `V_t` compatible with K/E/R-style ecosafety grammars and safestep predicates (`V_{t+1} ≤ V_t`).[web:9]
- K/E/R metrics aligned with existing Karma mappings where avoided mass (`M_help`) is mapped linearly, making WindHydro corridor-compliant nodes compatible with CEIM Karma namespaces.[web:9]

## Quick start

```bash
git clone <your-repo-url> phoenix-corridor-checker
cd phoenix-corridor-checker
cargo build --release
```

Run a check with a CAD export:

```bash
cargo run -- \
  --out qpudatashards/WindHydroPhoenix2026v1.aln \
  check \
  --geom examples/cad_exports/node01_geom.json \
  --node-id WindHydro-PHX-Node01 \
  --lat 33.4484 \
  --lon -112.0740
```

If all corridor conditions are satisfied, the tool emits a JSON-encoded shard file at the requested path.[web:9]

## CAD spec format

The CLI expects a JSON or YAML document with geometry and material fields:

```json
{
  "clear_gap_m": 0.015,
  "bar_diameter_m": 0.010,
  "bar_spacing_m": 0.030,
  "species_min_dim_child_m": 0.089,
  "species_min_dim_rat_m": 0.050,
  "species_min_dim_pigeon_m": 0.040,
  "alloy_name": "316L",
  "mesh_gauge_mm": 1.5,
  "rat_bite_cycles_survived": 15000,
  "rat_bite_cycles_required": 10000,
  "fatigue_cycles_survived": 2000000,
  "fatigue_cycles_required": 1000000,
  "corrosion_margin_years": 60.0,
  "design_life_years": 30.0,
  "projected_macro_litter_capture_kg_year": 140.0,
  "projected_energy_hybrid_mwh_year": 0.8
}
```

Species dimensions and mesh geometries can be calibrated from Phoenix-relevant anthropometry and nuisance-species data, then mapped into conservative `d_species,min` tables for corridor bands.[web:9]

## Corridor bands configuration

Default corridor bands are compiled in for Phoenix, but a custom JSON/YAML config can be supplied via `--corridors`:

```yaml
species:
  child:
    safe_max: 0.50
  rat:
    safe_max: 0.45
  pigeon:
    safe_max: 0.40
material:
  rat:
    safe_min: 1.10
structural:
  safe_min: 0.00
  safe_max: 0.90
```

These bands approximate a “well below 1 is safe, ≥ 1 is forbidden” policy for species gaps, together with a `r_mat,rat` threshold reflecting rat-bite and fatigue survival.[web:9]

Place this file at `config/phoenix_corridors.yaml` and run:

```bash
cargo run -- \
  --corridors config/phoenix_corridors.yaml \
  --out qpudatashards/WindHydroPhoenix2026v1.aln \
  check \
  --geom examples/cad_exports/node01_geom.json \
  --node-id WindHydro-PHX-Node01
```

## ALN shard integration

The emitted shard is structured for ALN qpudatashards:

- `meta`: shard ID, node ID, geostamp near Phoenix (≈33.4484 N, −112.0740 W), and a hex proof string.[web:9]
- `structural`: `r_gap,*`, `r_mat,rat`, and `V_struct` suitable for inclusion in ecosafety grammars.[web:9]
- `ecosafety`: `V_t` as a scalar violation potential combining structural and other corridor components.[web:9]
- `k_e_r`: `M_help`, `E_hybrid`, and K/E/R scores aligned with a shared 2026 ecosafety spine.[web:9]
- `geom_material`: echo of the input geometry/material spec for traceability and audit.[web:9]

Validators can recompute all invariants deterministically from the shard’s `geom_material` payload and reject any deployment whose stored coordinates deviate from recomputed values or fall outside configured safe bands.[web:9]

## License

MIT or Apache-2.0, as you prefer for the target GitHub organization.[web:9]
```

***

## File: `config/phoenix_corridors.yaml`

**Destination:** `config/phoenix_corridors.yaml`

```yaml
species:
  child:
    # child head/torso clearance corridor (normalized gap ratio)
    safe_max: 0.50
  rat:
    # rat body clearance corridor (normalized gap ratio)
    safe_max: 0.50
  pigeon:
    # pigeon body/wing corridor (normalized gap ratio)
    safe_max: 0.50

material:
  rat:
    # composite rat bite, fatigue, corrosion corridor (>= 1.0 is acceptable)
    safe_min: 1.00

structural:
  # Lyapunov-style structural residual safe band
  safe_min: 0.00
  safe_max: 1.00
```

These default Phoenix bands instantiate “no corridor, no build” for gaps where normalized ratios approach or exceed 1, and they require material performance at least equal to the reference corridor. [eartharxiv](https://eartharxiv.org/repository/object/8351/download/15657/)

***
