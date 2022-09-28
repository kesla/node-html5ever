use std::sync::atomic::{AtomicUsize, Ordering};

pub(crate) fn get_id() -> usize {
  static COUNTER: AtomicUsize = AtomicUsize::new(1);
  COUNTER.fetch_add(1, Ordering::Relaxed)
}
