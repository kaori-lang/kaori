use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_ID: AtomicUsize = AtomicUsize::new(1);

pub fn generate_id() -> usize {
    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}
