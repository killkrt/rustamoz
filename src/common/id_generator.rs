use std::sync::atomic::AtomicUsize;

/// Type used for all kind of unique identifier.
pub type Id = usize;

static LAST_GENERATED_ID: AtomicUsize = AtomicUsize::new(0);

struct IdGenerator;

impl IdGenerator {
    /// Generate an unique ID
    pub fn generate_id() -> Id {
        // Return current value and add 1 to current ID
        LAST_GENERATED_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
    }
}

/// A generic element that can be identifiable in a unique way
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Identifiable {
    id: Id,
}

impl Identifiable {
    /// Create a new element with unique id
    pub fn new() -> Self {
        Identifiable {
            id: IdGenerator::generate_id(),
        }
    }

    /// Get unique id
    pub fn id(&self) -> Id {
        self.id
    }
}

#[cfg(test)]
mod test {
    use constants::NUMBER_OF_LOOPS_FOR_NORMAL_TEST;

    use super::Identifiable;
    use crate::test_utilities::*;
    use std::thread;

    #[test]
    fn test_unique_id() {
        let mut ids = vec![];

        // Create some id and check if they are unique
        for _ in 0..NUMBER_OF_LOOPS_FOR_NORMAL_TEST {
            let id = Identifiable::new();
            ids.push(id.id());
        }

        // Remove all duplicates
        let len = ids.len();
        ids.sort();
        ids.dedup();
        // No duplicates shall be removed after removing duplicates
        assert_eq!(ids.len(), len);
    }

    #[test]
    fn multithread_test_unique_id() {
        const NTHREADS: u32 = 16u32;
        let mut children = vec![];

        for _ in 0..NTHREADS {
            // Spin up another thread
            children.push(thread::spawn(move || {
                let mut ids = vec![];

                for _ in 0..NUMBER_OF_LOOPS_FOR_NORMAL_TEST {
                    let id = Identifiable::new();
                    ids.push(id.id());
                }
                // Remove all duplicates
                let len = ids.len();
                ids.sort();
                ids.dedup();
                // No duplicates shall be removed after removing duplicates
                assert_eq!(ids.len(), len);
            }));
        }

        for child in children {
            // Wait for the thread to finish.
            let _ = child.join();
        }
    }
}
