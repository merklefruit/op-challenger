use anyhow::Result;
use lazy_static::lazy_static;
use prometheus_exporter::{
    prometheus::{register_counter, Counter},
    start,
};

lazy_static! {
    pub static ref ROOT_MISMATCH: Counter =
        register_counter!("root_mismatch", "number of output root mismatches found").unwrap();
}

/// Initializes the prometheus exporter
///
/// # Returns
/// * `Result<()>` - Ok if successful, Err otherwise.
pub fn init_prometheus_exporter() -> Result<()> {
    start("0.0.0.0:9201".parse().expect("Could not parse address"))?;
    Ok(())
}

/// Increments the root mismatch count metric by 1.
pub fn increment_root_mismatch_count() {
    ROOT_MISMATCH.inc();
}
