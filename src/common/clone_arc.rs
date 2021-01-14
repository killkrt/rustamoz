use std::sync::Arc;

/// Generic clonable object
pub trait Clonable {}

/// Fake trait
pub trait ArcClone<T> {
    fn arc_clone(&self) -> Arc<T>;
}

impl<T: Clone + Clonable> ArcClone<T> for T {
    /// Implementation for `ArcClone` trait for clonable object
    fn arc_clone(&self) -> Arc<T> {
        Arc::new(self.clone())
    }
}
