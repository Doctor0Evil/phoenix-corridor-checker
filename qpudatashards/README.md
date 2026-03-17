# qpudatashards directory

This directory stores provisional WindHydro and windnet qpudatashards emitted by the Phoenix Corridor Checker CLI.[web:9]

Each file (for example, `WindHydroPhoenix2026v1.aln`) is a JSON-encoded shard containing:

- `meta`: shard identity and geostamp.[web:9]
- `structural`: geometric and material corridor coordinates plus `V_struct`.[web:9]
- `ecosafety`: ecosafety residual `V_t` as a scalar violation potential.[web:9]
- `k_e_r`: K/E/R metrics tied to `M_help` and `E_hybrid` for Karma mapping.[web:9]
- `geom_material`: audit-ready copy of input CAD/geometry/material parameters.[web:9]

These provisional shards are intended for ingestion by ALN-aware edge agents and validator pools that recompute structural/material invariants and enforce “no corridor, no build” contracts at CI and runtime.[web:9]
