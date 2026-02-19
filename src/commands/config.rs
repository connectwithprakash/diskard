use anyhow::Result;
use console::style;
use diskard_core::config::Config;

pub fn show() -> Result<()> {
    let config = Config::load()?;
    let toml_str =
        toml::to_string_pretty(&config).map_err(|e| anyhow::anyhow!("Serialize error: {e}"))?;
    println!("{toml_str}");
    Ok(())
}

pub fn init() -> Result<()> {
    let path = Config::init()?;
    println!(
        "{}  Config initialized at {}",
        style("✓").green().bold(),
        style(path.display()).cyan(),
    );
    Ok(())
}

pub fn path() -> Result<()> {
    match Config::path() {
        Some(p) => println!("{}", p.display()),
        None => println!("{}", style("Cannot determine config directory").red()),
    }
    Ok(())
}
