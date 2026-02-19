use console::style;
use diskard_core::finding::RiskLevel;
use diskard_core::scanner::ScanResult;
use diskard_core::size::format_bytes;

/// Print scan results as a formatted table.
pub fn print_table(result: &ScanResult) {
    if result.findings.is_empty() {
        println!("{}", style("No reclaimable space found.").dim());
        return;
    }

    // Header
    println!(
        "\n{:>10}  {:>8}  {:<20}  {}",
        style("SIZE").bold().underlined(),
        style("RISK").bold().underlined(),
        style("CATEGORY").bold().underlined(),
        style("DESCRIPTION").bold().underlined(),
    );

    // Rows
    for finding in &result.findings {
        let risk_colored = match finding.risk {
            RiskLevel::Safe => style(format!("{:>8}", finding.risk)).green(),
            RiskLevel::Moderate => style(format!("{:>8}", finding.risk)).yellow(),
            RiskLevel::Risky => style(format!("{:>8}", finding.risk)).red(),
        };

        println!(
            "{:>10}  {}  {:<20}  {}",
            style(finding.size_human()).cyan(),
            risk_colored,
            finding.category,
            finding.description,
        );
        println!(
            "{:>10}  {:>8}  {:<20}  {}",
            "",
            "",
            "",
            style(finding.path.display()).dim(),
        );
    }

    // Summary
    println!();
    println!(
        "{}  Total reclaimable: {}",
        style("==>").green().bold(),
        style(format_bytes(result.total_reclaimable)).cyan().bold(),
    );
    println!(
        "{}  Scanned in {:.1}s",
        style("==>").green().bold(),
        result.scan_duration.as_secs_f64(),
    );

    if !result.errors.is_empty() {
        println!(
            "\n{}  {} errors during scan:",
            style("⚠").yellow(),
            result.errors.len(),
        );
        for err in &result.errors {
            println!("    {}", style(err).red());
        }
    }
}

/// Print scan results as JSON.
pub fn print_json(result: &ScanResult) {
    let output = serde_json::json!({
        "findings": result.findings,
        "total_reclaimable_bytes": result.total_reclaimable,
        "total_reclaimable_human": format_bytes(result.total_reclaimable),
        "scan_duration_ms": result.scan_duration.as_millis(),
        "errors": result.errors,
    });
    println!("{}", serde_json::to_string_pretty(&output).unwrap());
}
