use std::sync::Arc;

/// Generic trait for arc-clonable objects
pub trait Clonable {}

/// Trait that provide a method to clone via `Arc`
pub trait CloneArc {
    /// Returns an Arc that contains the clone of this object
    fn clone_arc<'slf>(&'slf self) -> Arc<dyn Clonable + 'slf>;
}

impl<T: Clone + Clonable> CloneArc for T {
    /// Returns an Arc that contains the clone of this object
    fn clone_arc<'slf>(&'slf self) -> Arc<dyn Clonable + 'slf> {
        Arc::new(self.clone())
    }
}
