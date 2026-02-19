use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(
    name = "diskard",
    about = "Developer-aware disk cleanup CLI",
    version,
    long_about = "Scans your machine for reclaimable disk space from developer tools, \
                  build caches, AI models, and more."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    /// Output format
    #[arg(long, global = true, default_value = "table")]
    pub format: OutputFormat,

    /// Verbose logging
    #[arg(short, long, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Command {
    /// Scan for reclaimable disk space
    Scan {
        /// Maximum risk level to show
        #[arg(long, short, default_value = "moderate")]
        risk: RiskFilter,

        /// Minimum size to report (e.g., "10MB", "1GB")
        #[arg(long)]
        min_size: Option<String>,

        /// Filter by category (e.g., "xcode", "node", "rust")
        #[arg(long, short)]
        category: Option<CategoryFilter>,

        /// Sort results by field
        #[arg(long, short, default_value = "size")]
        sort: SortField,

        /// Only show items older than duration (e.g., "7d", "30d", "1h")
        #[arg(long)]
        older_than: Option<String>,
    },

    /// Delete selected findings
    Clean {
        /// Only show what would be deleted
        #[arg(long)]
        dry_run: bool,

        /// Move to trash instead of permanent delete (default)
        #[arg(long, conflicts_with = "permanent")]
        trash: bool,

        /// Permanently delete files (irreversible)
        #[arg(long, conflicts_with = "trash")]
        permanent: bool,

        /// Maximum risk level to clean
        #[arg(long, short, default_value = "safe")]
        risk: RiskFilter,

        /// Filter by category
        #[arg(long, short)]
        category: Option<CategoryFilter>,

        /// Only clean items older than duration (e.g., "7d", "30d")
        #[arg(long)]
        older_than: Option<String>,

        /// Skip confirmation prompt
        #[arg(short = 'y', long)]
        yes: bool,
    },

    /// List available targets or configuration
    List {
        #[command(subcommand)]
        what: ListCommand,
    },

    /// Manage configuration
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },

    /// Interactive TUI mode — scan, select, and clean
    #[cfg(feature = "tui")]
    Interactive {
        /// Maximum risk level to show
        #[arg(long, short, default_value = "moderate")]
        risk: RiskFilter,

        /// Filter by category
        #[arg(long, short)]
        category: Option<CategoryFilter>,
    },

    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        shell: clap_complete::Shell,
    },
}

#[derive(Subcommand)]
pub enum ListCommand {
    /// List all available recognizers/targets
    Targets,
}

#[derive(Subcommand)]
pub enum ConfigAction {
    /// Show current configuration
    Show,
    /// Initialize default config file
    Init,
    /// Print config file path
    Path,
}

#[derive(Clone, Copy, ValueEnum)]
pub enum RiskFilter {
    Safe,
    Moderate,
    Risky,
}

impl RiskFilter {
    pub fn to_risk_level(self) -> diskard_core::finding::RiskLevel {
        match self {
            Self::Safe => diskard_core::finding::RiskLevel::Safe,
            Self::Moderate => diskard_core::finding::RiskLevel::Moderate,
            Self::Risky => diskard_core::finding::RiskLevel::Risky,
        }
    }
}

#[derive(Clone, Copy, ValueEnum)]
pub enum OutputFormat {
    Table,
    Json,
}

#[derive(Clone, Copy, ValueEnum)]
pub enum CategoryFilter {
    Xcode,
    Node,
    Homebrew,
    Python,
    Rust,
    Docker,
    Ollama,
    Huggingface,
    Claude,
    Vscode,
    Gradle,
    Cocoapods,
    Generic,
}

impl CategoryFilter {
    pub fn to_category(self) -> diskard_core::finding::Category {
        use diskard_core::finding::Category;
        match self {
            Self::Xcode => Category::Xcode,
            Self::Node => Category::Node,
            Self::Homebrew => Category::Homebrew,
            Self::Python => Category::Python,
            Self::Rust => Category::Rust,
            Self::Docker => Category::Docker,
            Self::Ollama => Category::Ollama,
            Self::Huggingface => Category::HuggingFace,
            Self::Claude => Category::Claude,
            Self::Vscode => Category::VSCode,
            Self::Gradle => Category::Gradle,
            Self::Cocoapods => Category::CocoaPods,
            Self::Generic => Category::Generic,
        }
    }
}

#[derive(Clone, Copy, ValueEnum)]
pub enum SortField {
    Size,
    Risk,
    Category,
}

impl SortField {
    pub fn to_sort_order(self) -> diskard_core::scanner::SortOrder {
        use diskard_core::scanner::SortOrder;
        match self {
            Self::Size => SortOrder::Size,
            Self::Risk => SortOrder::Risk,
            Self::Category => SortOrder::Category,
        }
    }
}
