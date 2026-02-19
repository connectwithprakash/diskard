use anyhow::Result;
use diskard_core::config::Config;
use diskard_core::recognizers::all_recognizers;
use diskard_core::scanner::{self, ScanOptions};

use crate::cli::{OutputFormat, RiskFilter};
use crate::output;

pub fn run(risk: RiskFilter, min_size: Option<String>, format: OutputFormat) -> Result<()> {
    let config = Config::load()?;
    let recognizers = all_recognizers();

    let min_size_bytes = match min_size {
        Some(s) => parse_size(&s)?,
        None => config.defaults.min_size,
    };

    let options = ScanOptions {
        max_risk: risk.to_risk_level(),
        min_size: min_size_bytes,
    };

    let result = scanner::scan(&recognizers, &config, &options);

    match format {
        OutputFormat::Table => output::print_table(&result),
        OutputFormat::Json => output::print_json(&result),
    }

    Ok(())
}

fn parse_size(s: &str) -> Result<u64> {
    let s = s.trim().to_uppercase();

    // Try to parse as plain number
    if let Ok(n) = s.parse::<u64>() {
        return Ok(n);
    }

    // Parse with suffix
    let (num_str, multiplier) = if let Some(n) = s.strip_suffix("GB") {
        (n.trim(), 1_073_741_824u64)
    } else if let Some(n) = s.strip_suffix("MB") {
        (n.trim(), 1_048_576u64)
    } else if let Some(n) = s.strip_suffix("KB") {
        (n.trim(), 1_024u64)
    } else if let Some(n) = s.strip_suffix('B') {
        (n.trim(), 1u64)
    } else {
        anyhow::bail!("Invalid size format: {s}. Use e.g. 10MB, 1GB");
    };

    let num: f64 = num_str
        .parse()
        .map_err(|_| anyhow::anyhow!("Invalid size number: {num_str}"))?;

    Ok((num * multiplier as f64) as u64)
}
