# diskard

Developer-aware disk cleanup CLI. Scans your machine for reclaimable space from build caches, AI models, package managers, and IDE artifacts.

## Why

Developers accumulate tens of GBs of reclaimable space — Xcode DerivedData, npm cache, Ollama models, HuggingFace cache, Docker images, Cargo target dirs, and more. diskard finds them all and lets you clean up safely.

## Install

```bash
cargo install diskard
```

## Usage

```bash
# Scan for reclaimable space
diskard scan

# Scan with JSON output
diskard scan --format json

# Scan only safe-to-delete items
diskard scan --risk safe

# Scan with minimum size filter
diskard scan --min-size 100MB

# Clean (move to Trash by default)
diskard clean --risk safe

# Clean with dry-run
diskard clean --dry-run

# Permanently delete (no Trash)
diskard clean --permanent --risk safe

# List available recognizers
diskard list targets

# Manage config
diskard config init     # Create default config
diskard config show     # Show current config
diskard config path     # Print config file path
```

## Recognizers

| Recognizer | Category | Path | Risk |
|---|---|---|---|
| Xcode DerivedData | Xcode | `~/Library/Developer/Xcode/DerivedData` | Safe |
| Xcode DeviceSupport | Xcode | `~/Library/Developer/Xcode/iOS DeviceSupport` | Moderate |
| Xcode Simulators | Xcode | `~/Library/Developer/CoreSimulator/Devices` | Risky |
| Xcode Previews | Xcode | `~/Library/Developer/Xcode/UserData/Previews` | Safe |
| npm cache | Node.js | `~/.npm` | Safe |
| Homebrew cache | Homebrew | `~/Library/Caches/Homebrew` | Safe |
| pip cache | Python | `~/Library/Caches/pip` | Safe |
| Cargo target dirs | Rust | `**/target/` (with Cargo.toml) | Moderate |
| Docker data | Docker | `~/Library/Containers/com.docker.docker/Data` | Risky |
| Ollama models | Ollama | `~/.ollama/models` | Moderate |
| HuggingFace cache | HuggingFace | `~/.cache/huggingface` | Moderate |
| Claude Code data | Claude | `~/.claude/projects/`, `~/.claude/debug/` | Moderate |
| VS Code extensions | VS Code | `~/.vscode/extensions` (old versions) | Moderate |
| .DS_Store files | Generic | `**/.DS_Store` | Safe |

## Configuration

Config file location: `~/.config/diskard/config.toml`

```toml
[defaults]
risk_tolerance = "moderate"
delete_mode = "trash"
min_size = 0

[ignore]
paths = []

[recognizers]
disabled = []
```

## Risk Levels

- **Safe** — Caches and build artifacts that regenerate automatically
- **Moderate** — Can be regenerated but may require downloads or time
- **Risky** — May contain user data or require manual reconfiguration

## License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE).
