pub fn format_size_auto(bytes: u64) -> String {
    const KIB: u64 = 1024;
    const MIB: u64 = 1024 * KIB;
    const GIB: u64 = 1024 * MIB;
    const TIB: u64 = 1024 * GIB;

    if bytes >= TIB {
        format!("{:.2} TiB", bytes as f64 / TIB as f64)
    } else if bytes >= GIB {
        format!("{:.2} GiB", bytes as f64 / GIB as f64)
    } else if bytes >= MIB {
        format!("{:.2} MiB", bytes as f64 / MIB as f64)
    } else if bytes >= KIB {
        format!("{:.2} KiB", bytes as f64 / KIB as f64)
    } else {
        format!("{} B", bytes)
    }
}

pub fn format_size_with_unit(bytes: u64, unit: &str) -> Result<String, String> {
    match unit.to_lowercase().as_str() {
        "b" => Ok(format!("{} B", bytes)),
        "kib" => Ok(format!("{:.2} KiB", bytes as f64 / 1024.0)),
        "mib" => Ok(format!("{:.2} MiB", bytes as f64 / (1024.0 * 1024.0))),
        "gib" => Ok(format!("{:.2} GiB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))),
        "tib" => Ok(format!("{:.2} TiB", bytes as f64 / (1024.0 * 1024.0 * 1024.0 * 1024.0))),
        _ => Err(format!("Invalid unit: {}", unit)),
    }
}
