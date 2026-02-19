pub mod app;
pub mod components;
pub mod event;
pub mod tui;

use app::{App, AppMode};
use crossterm::event::{KeyCode, KeyEventKind, KeyModifiers};
use diskard_core::cleaner::{self, DeleteMode};
use diskard_core::finding::Finding;
use diskard_core::size::format_bytes;
use ratatui::layout::{Constraint, Layout};
use std::io;
use std::time::Duration;

/// Run the interactive TUI with the given findings.
pub fn run(findings: Vec<Finding>) -> io::Result<()> {
    if findings.is_empty() {
        println!("No reclaimable space found.");
        return Ok(());
    }

    let mut terminal = tui::init()?;
    let mut app = App::new(findings);

    loop {
        // Draw
        terminal.draw(|frame| {
            let area = frame.area();

            // Main layout: header + results + status bar
            let chunks = Layout::vertical([
                Constraint::Length(5),
                Constraint::Min(5),
                Constraint::Length(1),
            ])
            .split(area);

            // Disk summary header
            components::header::render(frame, chunks[0], &app);

            // Main content area
            if app.mode == AppMode::DrillDown {
                if let Some(ref state) = app.drill_down {
                    components::drilldown::render(frame, chunks[1], state);
                }
            } else {
                components::results::render(frame, chunks[1], &app);
            }

            // Status bar
            let status = if let Some(ref msg) = app.status_message {
                msg.clone()
            } else if app.mode == AppMode::DrillDown || app.mode == AppMode::ConfirmDrillDown {
                " ↑↓/jk: navigate | Space: toggle | a: all | d: delete | l/→/Enter: open | h/←: back | Esc/q: exit".to_string()
            } else {
                " ↑↓/jk: navigate | Space: toggle | a: all | l/→: inspect | Enter: delete | ?: help | q: quit"
                    .to_string()
            };
            let status_bar = ratatui::widgets::Paragraph::new(status)
                .style(ratatui::style::Style::default().bg(ratatui::style::Color::DarkGray));
            frame.render_widget(status_bar, chunks[2]);

            // Overlays
            if app.mode == AppMode::Confirm || app.mode == AppMode::ConfirmDrillDown {
                components::confirm::render(frame, area, &app);
            }
            if app.show_help {
                components::help::render(frame, area);
            }
        })?;

        if app.should_quit {
            break;
        }

        // Handle events
        if let Some(evt) = event::poll(Duration::from_millis(100)) {
            match evt {
                event::Event::Key(key) if key.kind == KeyEventKind::Press => {
                    app.status_message = None;

                    // Ctrl+C always quits
                    if key.modifiers.contains(KeyModifiers::CONTROL)
                        && key.code == KeyCode::Char('c')
                    {
                        app.should_quit = true;
                        continue;
                    }

                    match app.mode {
                        AppMode::Browse => match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
                            KeyCode::Char('j') | KeyCode::Down => app.move_down(),
                            KeyCode::Char('k') | KeyCode::Up => app.move_up(),
                            KeyCode::Char(' ') => app.toggle_selected(),
                            KeyCode::Char('a') => app.select_all(),
                            KeyCode::Char('?') => app.show_help = !app.show_help,
                            KeyCode::Char('l') | KeyCode::Right => app.enter_drill_down(),
                            KeyCode::Enter => {
                                if app.checked_count() > 0 {
                                    app.mode = AppMode::Confirm;
                                } else {
                                    app.status_message =
                                        Some(" No items selected. Use Space to select.".into());
                                }
                            }
                            _ => {}
                        },
                        AppMode::DrillDown => match key.code {
                            KeyCode::Char('j') | KeyCode::Down => {
                                if let Some(ref mut state) = app.drill_down {
                                    state.move_down();
                                }
                            }
                            KeyCode::Char('k') | KeyCode::Up => {
                                if let Some(ref mut state) = app.drill_down {
                                    state.move_up();
                                }
                            }
                            KeyCode::Char(' ') => {
                                if let Some(ref mut state) = app.drill_down {
                                    state.toggle_selected();
                                }
                            }
                            KeyCode::Char('a') => {
                                if let Some(ref mut state) = app.drill_down {
                                    state.select_all();
                                }
                            }
                            KeyCode::Char('d') => {
                                if let Some(ref state) = app.drill_down {
                                    if state.checked_count() > 0 {
                                        app.mode = AppMode::ConfirmDrillDown;
                                    } else {
                                        app.status_message =
                                            Some(" No items selected. Use Space to select.".into());
                                    }
                                }
                            }
                            KeyCode::Char('l') | KeyCode::Right | KeyCode::Enter => {
                                if let Some(ref mut state) = app.drill_down {
                                    if !state.drill_into() {
                                        let is_file = state
                                            .entries
                                            .get(state.selected)
                                            .is_some_and(|e| !e.is_dir);
                                        app.status_message = Some(if is_file {
                                            " Not a directory.".into()
                                        } else {
                                            " Cannot read directory.".into()
                                        });
                                    }
                                }
                            }
                            KeyCode::Char('h') | KeyCode::Left => {
                                if let Some(ref mut state) = app.drill_down {
                                    if !state.go_back() {
                                        app.exit_drill_down();
                                    }
                                }
                            }
                            KeyCode::Esc | KeyCode::Char('q') => {
                                app.exit_drill_down();
                            }
                            _ => {}
                        },
                        AppMode::ConfirmDrillDown => match key.code {
                            KeyCode::Char('y') | KeyCode::Enter => {
                                if let Some(ref mut state) = app.drill_down {
                                    let to_delete = state.checked_paths();
                                    let mut deleted = 0u64;
                                    let mut count = 0usize;
                                    let mut err_count = 0usize;
                                    for (path, size) in &to_delete {
                                        match cleaner::delete_path(path, DeleteMode::Trash) {
                                            Ok(()) => {
                                                deleted += size;
                                                count += 1;
                                            }
                                            Err(_) => err_count += 1,
                                        }
                                    }
                                    state.remove_checked();
                                    let msg = if err_count > 0 {
                                        format!(
                                            " Trashed {count} items ({}), {err_count} failed",
                                            format_bytes(deleted),
                                        )
                                    } else {
                                        format!(
                                            " Moved {count} items to Trash, freed {}",
                                            format_bytes(deleted),
                                        )
                                    };
                                    app.status_message = Some(msg);
                                }
                                app.mode = AppMode::DrillDown;
                            }
                            KeyCode::Char('n') | KeyCode::Esc => {
                                app.mode = AppMode::DrillDown;
                            }
                            _ => {}
                        },
                        AppMode::Confirm => match key.code {
                            KeyCode::Char('y') | KeyCode::Enter => {
                                let to_delete = app.checked_findings();
                                let count = to_delete.len();
                                match cleaner::clean(&to_delete, DeleteMode::Trash) {
                                    Ok(result) => {
                                        app.remove_checked();
                                        app.status_message = Some(format!(
                                            " Moved {} items to Trash, freed {}",
                                            result.deleted_count,
                                            format_bytes(result.freed_bytes),
                                        ));
                                    }
                                    Err(e) => {
                                        app.status_message =
                                            Some(format!(" Error cleaning {count} items: {e}"));
                                    }
                                }
                                app.mode = AppMode::Browse;
                                if app.findings.is_empty() {
                                    app.should_quit = true;
                                }
                            }
                            KeyCode::Char('n') | KeyCode::Esc => {
                                app.mode = AppMode::Browse;
                            }
                            _ => {}
                        },
                    }
                }
                _ => {}
            }
        }
    }

    tui::restore()?;
    Ok(())
}
