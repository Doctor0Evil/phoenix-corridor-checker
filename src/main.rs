use clap::{Parser, Subcommand};
use crate::io::{load_geom_spec, write_shard};
use crate::geom_material::GeomMaterialSpec;
use crate::corridors::{CorridorBands, SpeciesBands, MaterialBands, CorridorEvaluator};
use crate::shard::{WindHydroShard, ShardMeta};
use crate::lyapunov::StructuralResidual;
use chrono::Utc;
use rand::Rng;

mod corridors;
mod geom_material;
mod shard;
mod io;
mod lyapunov;

#[derive(Parser, Debug)]
#[command(name = "phoenix-corridor-checker")]
#[command(about = "Phoenix Hardware Corridor Checker for WindHydro / windnet nodes", long_about = None)]
struct Cli {
    /// Path to corridor bands config (YAML or JSON)
    #[arg(short = 'c', long = "corridors")]
    corridors_path: Option<String>,

    /// Output shard path
    #[arg(short = 'o', long = "out", default_value = "qpudatashards/WindHydroPhoenix2026v1.aln")]
    out_path: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Check a CAD-exported geometry/material spec and emit a shard
    Check {
        /// Path to CAD-derived geometry/material JSON/YAML
        #[arg(short = 'g', long = "geom")]
        geom_path: String,

        /// Node identifier (e.g., WindHydro-PHX-Node01)
        #[arg(short = 'n', long = "node-id")]
        node_id: String,

        /// Approximate latitude
        #[arg(long = "lat", default_value_t = 33.4484)]
        lat: f64,

        /// Approximate longitude
        #[arg(long = "lon", default_value_t = -112.0740)]
        lon: f64,
    },
}

fn main() {
    let cli = Cli::parse();

    let bands = match &cli.corridors_path {
        Some(path) => CorridorBands::from_file(path).unwrap_or_else(|e| {
            eprintln!("Failed to load corridor bands from {}: {}", path, e);
            std::process::exit(1);
        }),
        None => CorridorBands::phoenix_default(),
    };

    match cli.command {
        Commands::Check { geom_path, node_id, lat, lon } => {
            let spec: GeomMaterialSpec = match load_geom_spec(&geom_path) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Failed to load geometry/material spec: {}", e);
                    std::process::exit(1);
                }
            };

            let evaluator = CorridorEvaluator::new(bands.clone());
            let eval = evaluator.evaluate(&spec);

            if !eval.accepted {
                eprintln!("NO BUILD: one or more structural/material corridors violated");
                eprintln!("Details:");
                eprintln!("  rgap_child  = {:.4} (safe <= {:.4})", eval.rgap_child, evaluator.bands.species.child.safe_max);
                eprintln!("  rgap_rat    = {:.4} (safe <= {:.4})", eval.rgap_rat, evaluator.bands.species.rat.safe_max);
                eprintln!("  rgap_pigeon = {:.4} (safe <= {:.4})", eval.rgap_pigeon, evaluator.bands.species.pigeon.safe_max);
                eprintln!("  rmat_rat    = {:.4} (safe >= {:.4})", eval.rmat_rat, evaluator.bands.material.rat.safe_min);
                eprintln!("  V_struct    = {:.4} (safe within [{:.4}, {:.4}])",
                          eval.v_struct,
                          evaluator.bands.structural.safe_min,
                          evaluator.bands.structural.safe_max);
                std::process::exit(2);
            }

            let now = Utc::now();
            let mut rng = rand::thread_rng();
            let hex_suffix: u64 = rng.gen();

            let meta = ShardMeta {
                shard_id: format!("WindHydroPhoenix2026v1-{}", uuid::Uuid::new_v4()),
                node_id,
                timestamp_utc: now,
                lat,
                lon,
                hex_proof: format!("0x{:016x}", hex_suffix),
            };

            let shard = WindHydroShard::from_eval(meta, &spec, &eval);

            if let Err(e) = write_shard(&cli.out_path, &shard) {
                eprintln!("Failed to write shard: {}", e);
                std::process::exit(1);
            }

            println!("Shard written to {}", cli.out_path);
            println!("  shard_id    : {}", shard.meta.shard_id);
            println!("  node_id     : {}", shard.meta.node_id);
            println!("  rgap_child  : {:.4}", shard.structural.rgap_child);
            println!("  rgap_rat    : {:.4}", shard.structural.rgap_rat);
            println!("  rgap_pigeon : {:.4}", shard.structural.rgap_pigeon);
            println!("  rmat_rat    : {:.4}", shard.structural.rmat_rat);
            println!("  V_struct    : {:.4}", shard.structural.v_struct);
            println!("  V_t         : {:.4}", shard.ecosafety.v_t);
            println!("  M_help      : {:.4} kg/year", shard.k_e_r.m_help_kg_year);
            println!("  E_hybrid    : {:.4} MWh/year", shard.k_e_r.e_hybrid_mwh_year);
        }
    }
}
