use std::sync::atomic::{AtomicUsize, Ordering};
use once_cell::sync::OnceCell;

/// Auto-incrementing request ID generator
pub struct RequestIdGenerator {
    prefix: String,
    counter: AtomicUsize,
}

impl RequestIdGenerator {
    pub fn new(prefix: impl Into<String>) -> Self {
        Self {
            prefix: prefix.into(),
            counter: AtomicUsize::new(1),
        }
    }

    pub fn next(&self) -> String {
        let count = self.counter.fetch_add(1, Ordering::SeqCst);
        format!("{}-{:06}", self.prefix, count)
    }
}

/// Global generator
static REQUEST_ID_GENERATOR: OnceCell<RequestIdGenerator> = OnceCell::new();
const DEFAULT_PREFIX: &str = "dareq";

/// Generate request ID
pub fn generate_request_id() -> String {
    let generator = REQUEST_ID_GENERATOR
        .get_or_init(|| RequestIdGenerator::new(DEFAULT_PREFIX));
    generator.next()
}

/// **Public function** to set a custom prefix
pub fn daconfig_set_request_id_prefix(prefix: &str) {
    // Try to set a new generator; if already initialized, do nothing
    let _ = REQUEST_ID_GENERATOR.set(RequestIdGenerator::new(prefix));
}
