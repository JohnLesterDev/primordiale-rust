pub mod event_queue;
pub mod dispatch;

pub use event_queue::{GameEvent, EventQueue};
pub use dispatch::dispatch_events;