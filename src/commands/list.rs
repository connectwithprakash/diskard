use anyhow::Result;
use console::style;
use diskard_core::config::Config;
use diskard_core::recognizers::all_recognizers;

pub fn targets() -> Result<()> {
    let config = Config::load()?;
    let recognizers = all_recognizers();

    println!("\n{}", style("Available recognizers:").bold());
    println!(
        "  {:>3}  {:<25}  {:<12}  {}",
        style("#").dim(),
        style("ID").bold().underlined(),
        style("CATEGORY").bold().underlined(),
        style("NAME").bold().underlined(),
    );

    for (i, r) in recognizers.iter().enumerate() {
        let enabled = config.is_recognizer_enabled(r.id());
        let status = if enabled {
            style("●").green()
        } else {
            style("○").red()
        };

        println!(
            "  {:>3}  {:<25}  {:<12}  {} {}",
            i + 1,
            r.id(),
            r.category(),
            r.name(),
            status,
        );
    }

    println!(
        "\n  {} enabled  {} disabled",
        style("●").green(),
        style("○").red(),
    );
    println!(
        "  Disable recognizers in {}",
        style("~/.config/diskard/config.toml").dim(),
    );

    Ok(())
}
