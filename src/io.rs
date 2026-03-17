use crate::geom_material::GeomMaterialSpec;
use crate::shard::WindHydroShard;
use std::fs::{self, File};
use std::io::{Read, Write};

pub fn load_geom_spec(path: &str) -> Result<GeomMaterialSpec, std::io::Error> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    if path.ends_with(".yaml") || path.ends_with(".yml") {
        let spec: GeomMaterialSpec = serde_yaml::from_str(&s)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        Ok(spec)
    } else {
        let spec: GeomMaterialSpec = serde_json::from_str(&s)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        Ok(spec)
    }
}

pub fn write_shard(path: &str, shard: &WindHydroShard) -> Result<(), std::io::Error> {
    if let Some(parent) = std::path::Path::new(path).parent() {
        fs::create_dir_all(parent)?;
    }
    let mut f = File::create(path)?;
    let encoded = serde_json::to_string_pretty(shard)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    f.write_all(encoded.as_bytes())?;
    Ok(())
}
