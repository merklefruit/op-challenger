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

pub fn init_prometheus_exporter() -> Result<()> {
    start("0.0.0.0:9201".parse().expect("Could not parse address"))?;
    Ok(())
}
