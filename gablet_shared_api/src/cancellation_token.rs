use std::sync::{Mutex, atomic::{AtomicBool, Ordering}, Arc};

pub struct CancellationSource {
    token: Arc<AtomicBool>
}

impl CancellationSource {
    pub fn new() -> Self {
        Self { token: Arc::new(AtomicBool::new(false)) }
    }

    pub fn request_cancellation(&mut self) {
        self.token.store(true, Ordering::Relaxed);
    }

    pub fn token(&self) -> CancellationToken {
        CancellationToken {
            token: Arc::clone(&self.token)
        }
    }
}

#[derive(Clone)]
pub struct CancellationToken {
    token: Arc<AtomicBool>
}

impl CancellationToken {
    pub fn is_cancellation_requested(&self) -> bool {
        self.token.load(Ordering::Relaxed)
    }
}