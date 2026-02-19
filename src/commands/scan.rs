use anyhow::Result;
use diskard_core::config::Config;
use diskard_core::recognizers::all_recognizers;
use diskard_core::scanner::{self, ScanOptions};

use crate::cli::{CategoryFilter, OutputFormat, RiskFilter, SortField};
use crate::output;

pub fn run(
    risk: RiskFilter,
    min_size: Option<String>,
    category: Option<CategoryFilter>,
    sort: SortField,
    older_than: Option<String>,
    format: OutputFormat,
) -> Result<()> {
    let config = Config::load()?;
    let recognizers = all_recognizers();

    let min_size_bytes = match min_size {
        Some(s) => parse_size(&s)?,
        None => config.defaults.min_size,
    };

    let older_duration = match older_than {
        Some(s) => Some(parse_duration(&s)?),
        None => None,
    };

    let options = ScanOptions {
        max_risk: risk.to_risk_level(),
        min_size: min_size_bytes,
        category: category.map(|c| c.to_category()),
        older_than: older_duration,
        sort: sort.to_sort_order(),
    };

    let result = scanner::scan(&recognizers, &config, &options);

    match format {
        OutputFormat::Table => output::print_table(&result),
        OutputFormat::Json => output::print_json(&result),
    }

    Ok(())
}

pub fn parse_size(s: &str) -> Result<u64> {
    let s = s.trim().to_uppercase();

    if let Ok(n) = s.parse::<u64>() {
        return Ok(n);
    }

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

pub fn parse_duration(s: &str) -> Result<std::time::Duration> {
    let s = s.trim().to_lowercase();

    let (num_str, multiplier) = if let Some(n) = s.strip_suffix('d') {
        (n, 86_400u64)
    } else if let Some(n) = s.strip_suffix('h') {
        (n, 3_600u64)
    } else if let Some(n) = s.strip_suffix('m') {
        (n, 60u64)
    } else if let Some(n) = s.strip_suffix('w') {
        (n, 604_800u64)
    } else {
        anyhow::bail!("Invalid duration: {s}. Use e.g. 7d, 30d, 1h, 2w");
    };

    let num: u64 = num_str
        .parse()
        .map_err(|_| anyhow::anyhow!("Invalid duration number: {num_str}"))?;

    Ok(std::time::Duration::from_secs(num * multiplier))
}
