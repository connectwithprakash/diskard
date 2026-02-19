use crossterm::event::{self, Event as CrosstermEvent, KeyEvent};
use std::time::Duration;

/// Application events.
pub enum Event {
    Key(KeyEvent),
    Tick,
}

/// Poll for terminal events with a tick rate.
pub fn poll(tick_rate: Duration) -> Option<Event> {
    if event::poll(tick_rate).ok()? {
        if let CrosstermEvent::Key(key) = event::read().ok()? {
            return Some(Event::Key(key));
        }
    }
    Some(Event::Tick)
}
