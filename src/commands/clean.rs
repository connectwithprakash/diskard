use anyhow::Result;
use console::style;
use diskard_core::cleaner::{self, DeleteMode};
use diskard_core::config::Config;
use diskard_core::recognizers::all_recognizers;
use diskard_core::scanner::{self, ScanOptions};
use diskard_core::size::format_bytes;

use crate::cli::{CategoryFilter, RiskFilter};
use crate::commands::scan::parse_duration;

pub fn run(
    dry_run: bool,
    permanent: bool,
    risk: RiskFilter,
    category: Option<CategoryFilter>,
    older_than: Option<String>,
    yes: bool,
) -> Result<()> {
    let config = Config::load()?;
    let recognizers = all_recognizers();

    let older_duration = match older_than {
        Some(s) => Some(parse_duration(&s)?),
        None => None,
    };

    let options = ScanOptions {
        max_risk: risk.to_risk_level(),
        min_size: config.defaults.min_size,
        category: category.map(|c| c.to_category()),
        older_than: older_duration,
        ..Default::default()
    };

    let result = scanner::scan(&recognizers, &config, &options);

    if result.findings.is_empty() {
        println!("{}", style("Nothing to clean.").dim());
        return Ok(());
    }

    // Show what will be cleaned
    println!("\n{}", style("Items to clean:").bold());
    for (i, finding) in result.findings.iter().enumerate() {
        println!(
            "  {}. {} {} — {}",
            i + 1,
            style(finding.size_human()).cyan(),
            finding.risk.emoji(),
            finding.description,
        );
        println!("     {}", style(finding.path.display()).dim());
    }

    println!(
        "\n{}  Total: {}",
        style("==>").green().bold(),
        style(format_bytes(result.total_reclaimable)).cyan().bold(),
    );

    let mode = if dry_run {
        DeleteMode::DryRun
    } else if permanent {
        DeleteMode::Permanent
    } else {
        DeleteMode::Trash
    };

    if mode == DeleteMode::DryRun {
        println!(
            "\n{}",
            style("Dry run — no files were deleted.").yellow().bold()
        );
        return Ok(());
    }

    // Confirmation
    if !yes {
        let mode_label = if permanent {
            "PERMANENTLY DELETE"
        } else {
            "move to Trash"
        };
        print!(
            "\n{} {} these items? [y/N] ",
            style("?").yellow().bold(),
            mode_label,
        );
        use std::io::{self, Write};
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if !input.trim().eq_ignore_ascii_case("y") {
            println!("{}", style("Cancelled.").dim());
            return Ok(());
        }
    }

    // Execute
    let clean_result = cleaner::clean(&result.findings, mode)?;

    println!(
        "\n{}  Cleaned {} items, freed {}",
        style("✓").green().bold(),
        clean_result.deleted_count,
        style(format_bytes(clean_result.freed_bytes)).cyan().bold(),
    );

    if !clean_result.errors.is_empty() {
        println!(
            "\n{}  {} errors:",
            style("⚠").yellow(),
            clean_result.errors.len(),
        );
        for (path, err) in &clean_result.errors {
            println!("    {} — {}", style(path).red(), err);
        }
    }

    Ok(())
}
