const B: u64 = 1;
const KIB: u64 = B * 1024;
const MIB: u64 = KIB * 1024;
const GIB: u64 = MIB * 1024;
const TIB: u64 = GIB * 1024;

/// Formats a byte count into a human-readable string with an appropriate unit (B, KiB, MiB, GiB, TiB).
///
/// This function automatically selects the most suitable unit to display the file size,
/// providing a more readable output for various magnitudes of bytes.
///
/// # Arguments
///
/// * `bytes` - The number of bytes to format, as a `u64`.
///
/// # Returns
///
/// A `String` representing the formatted file size (e.g., "1.23 MiB", "456 B").
pub fn format_size_auto(bytes: u64) -> String {
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

#[allow(dead_code)]
/// Formats a byte count into a human-readable string with an appropriate unit (B, KiB, MiB, GiB, TiB).
///
/// This function automatically selects the most suitable unit to display the file size,
/// providing a more readable output for various magnitudes of bytes.
///
/// # Arguments
///
/// * `bytes` - The number of bytes to format, as a `u64`.
///
/// # Returns
///
/// A `String` representing the formatted file size (e.g., "1.23 MiB", "456 B").
pub fn format_size_with_unit(bytes: u64, unit: &str) -> Result<String, String> {
    match unit.to_lowercase().as_str() {
        "b" => Ok(format!("{} B", bytes as f64)),
        "kib" => Ok(format!("{:.2} KiB", bytes as f64 / KIB as f64)),
        "mib" => Ok(format!("{:.2} MiB", bytes as f64 / MIB as f64)),
        "gib" => Ok(format!("{:.2} GiB", bytes as f64 / GIB as f64)),
        "tib" => Ok(format!("{:.2} TiB", bytes as f64 / TIB as f64)),
        _ => Err(format!("Invalid unit: {}", unit)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_size_auto() {
        assert_eq!(format_size_auto(0), "0 B");
        assert_eq!(format_size_auto(1024), "1.00 KiB");
        assert_eq!(format_size_auto(1024 * 1024), "1.00 MiB");
        assert_eq!(format_size_auto(1024 * 1024 * 1024), "1.00 GiB");
        assert_eq!(format_size_auto(1024 * 1024 * 1024 * 1024), "1.00 TiB");
        assert_eq!(format_size_auto(256 * 1024 * 1024 * 1024), "256.00 GiB");
    }

    #[test]
    fn test_format_size_with_unit() {
        assert_eq!(format_size_with_unit(0, "b").unwrap(), "0 B");
        assert_eq!(format_size_with_unit(1024, "b").unwrap(), "1024 B");
        assert_eq!(
            format_size_with_unit(1024 * 1024, "kib").unwrap(),
            "1024.00 KiB"
        );
        assert_eq!(
            format_size_with_unit(1024 * 1024 * 1024, "mib").unwrap(),
            "1024.00 MiB"
        );
        assert_eq!(
            format_size_with_unit(1024 * 1024 * 1024 * 1024, "gib").unwrap(),
            "1024.00 GiB"
        );
        assert_eq!(
            format_size_with_unit(1024 * 1024 * 1024 * 1024 * 1024, "tib").unwrap(),
            "1024.00 TiB"
        );
    }
}
