use std::sync::atomic::AtomicUsize;

/// Type used for all kind of unique identifier.
pub type Id = usize;

static LAST_GENERATED_ID: AtomicUsize = AtomicUsize::new(0);

/// Unique ID generator
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
    use constants::*;

    use super::Identifiable;
    use crate::test_utilities::*;
    use std::{sync::mpsc, thread};

    #[test]
    fn test_unique_id() {
        let mut ids = vec![];

        // Create some id and check if they are unique
        for _ in 0..NUMBER_OF_LOOPS_FOR_BIG_TEST {
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
        const NTHREADS: usize = 16usize;
        const N_ID_PER_THREAD: usize = NUMBER_OF_LOOPS_FOR_MID_TEST;
        // Channel to receive the ID
        let (tx, rx) = mpsc::channel::<usize>();

        //for _ in 0..NTHREADS {
        // Spawn a new thread
        let handles: Vec<_> = (0..NTHREADS)
            .map(|_| {
                // Clone the sender for a new thread
                let thread_tx = tx.clone();
                thread::spawn(move || {
                    let mut ids = vec![];

                    for _ in 0..N_ID_PER_THREAD {
                        let id = Identifiable::new();
                        ids.push(id.id());
                        // Send ID value to main thread
                        thread_tx.send(id.id()).unwrap();
                    }
                    // Remove all duplicates
                    let len = ids.len();
                    ids.sort();
                    ids.dedup();
                    // No duplicates shall be removed after removing duplicates
                    assert_eq!(ids.len(), len);
                })
            })
            .collect();

        // Wait for threads to complete
        for handle in handles {
            handle.join().unwrap();
        }

        let mut ids = vec![];
        for id in rx.try_iter() {
            // Sum all ID
            ids.push(id);
        }

        // Remove all duplicates
        let len = ids.len();
        ids.sort();
        ids.dedup();
        // No duplicates shall be removed after removing duplicates
        assert_eq!(ids.len(), len);
        assert_eq!(ids.len(), NTHREADS * N_ID_PER_THREAD);
    }
}
