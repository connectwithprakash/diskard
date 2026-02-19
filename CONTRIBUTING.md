# Contributing to diskard

## Adding a Recognizer

1. Create a new file in `crates/diskard-core/src/recognizers/`
2. Implement the `Recognizer` trait:

```rust
use crate::error::Result;
use crate::finding::{Category, Finding, RiskLevel};
use crate::recognizer::Recognizer;
use crate::size::dir_size;

pub struct MyRecognizer;

impl Recognizer for MyRecognizer {
    fn name(&self) -> &'static str { "My Tool Cache" }
    fn id(&self) -> &'static str { "my-tool-cache" }
    fn category(&self) -> Category { Category::Generic }

    fn scan(&self) -> Result<Vec<Finding>> {
        // Check if path exists, calculate size, return findings
        Ok(vec![])
    }
}
```

3. Register it in `recognizers/mod.rs`
4. Add a test in the recognizer file
5. Update the README recognizer table

## Code Style

- Run `cargo fmt` before committing
- Run `cargo clippy` and fix all warnings
- Write tests for new functionality

## Pull Requests

- One feature or fix per PR
- Include tests
- Update documentation if needed
- Use conventional commit messages: `feat:`, `fix:`, `docs:`, `test:`, `chore:`
