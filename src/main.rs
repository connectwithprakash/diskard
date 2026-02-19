mod cli;
mod commands;
mod output;

use clap::{CommandFactory, Parser};
use cli::{Cli, Command, ConfigAction, ListCommand};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if cli.verbose {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    } else {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();
    }

    match cli.command {
        Command::Scan {
            risk,
            min_size,
            category,
            sort,
            older_than,
        } => {
            commands::scan::run(risk, min_size, category, sort, older_than, cli.format)?;
        }
        Command::Clean {
            dry_run,
            permanent,
            risk,
            category,
            older_than,
            yes,
            ..
        } => {
            commands::clean::run(dry_run, permanent, risk, category, older_than, yes)?;
        }
        Command::List { what } => match what {
            ListCommand::Targets => commands::list::targets()?,
        },
        Command::Config { action } => match action {
            ConfigAction::Show => commands::config::show()?,
            ConfigAction::Init => commands::config::init()?,
            ConfigAction::Path => commands::config::path()?,
        },
        #[cfg(feature = "tui")]
        Command::Interactive { risk, category } => {
            use diskard_core::config::Config;
            use diskard_core::recognizers::all_recognizers;
            use diskard_core::scanner::{self, ScanOptions};

            let config = Config::load()?;
            let recognizers = all_recognizers();
            let options = ScanOptions {
                max_risk: risk.to_risk_level(),
                category: category.map(|c| c.to_category()),
                ..Default::default()
            };

            println!("Scanning...");
            let result = scanner::scan(&recognizers, &config, &options);
            diskard_tui::run(result.findings)?;
        }
        Command::Completions { shell } => {
            let mut cmd = Cli::command();
            clap_complete::generate(shell, &mut cmd, "diskard", &mut std::io::stdout());
        }
    }

    Ok(())
}
