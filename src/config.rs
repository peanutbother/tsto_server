use crate::app::models::direction::Direction;
use server::ServerOptions;
use std::sync::Mutex;

mod direction;
mod server;

lazy_static::lazy_static! {
    pub static ref DIRECTIONS: Direction = Direction::new();
    pub static ref OPTIONS: Mutex<ServerOptions> = Mutex::new(ServerOptions::new());
}

impl OPTIONS {
    /// locks and unwraps the mutex
    ///
    /// panics if mutex fails to lock
    ///
    /// If another user of this mutex panicked while holding the mutex, then this call will return an error once the mutex is acquired. The acquired mutex guard will be contained in the returned error.
    pub fn take(&self) -> std::sync::MutexGuard<'_, ServerOptions> {
        self.lock().unwrap()
    }
}
