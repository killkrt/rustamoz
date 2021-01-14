use std::sync::Arc;

pub trait Clonable {}

pub trait ArcClone<T> {
    fn arc_clone(&self) -> Arc<T>;
}

impl<T: Clone + Clonable> ArcClone<T> for T {
    fn arc_clone(&self) -> Arc<T> {
        Arc::new(self.clone())
    }
}
