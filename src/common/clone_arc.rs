use std::{fmt::Debug, sync::Arc};

/// Generic trait for arc-clonable objects
pub trait Clonable: Debug {}

/// Trait that provide a method to clone via `Arc`
pub trait CloneArc {
    /// Returns an Arc that contains the clone of this object
    fn clone_arc<'slf>(self: &'_ Self) -> Arc<dyn Clonable + 'slf>
    where
        Self: 'slf;
}

impl<T: Clone + Clonable> CloneArc for T {
    /// Returns an Arc that contains the clone of this object
    fn clone_arc<'slf>(self: &'_ Self) -> Arc<dyn Clonable + 'slf>
    where
        Self: 'slf,
    {
        Arc::new(self.clone())
    }
}
