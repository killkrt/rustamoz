use std::sync::atomic::AtomicUsize;

/// Type used for all kind of unique identifier.
pub type Id = usize;

static LAST_GENERATED_ID: AtomicUsize = AtomicUsize::new(0);

/// Generate an unique ID
pub fn new_id() -> Id {
    // Return current value and add 1 to current ID
    LAST_GENERATED_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
}

#[cfg(test)]
mod test {
    use common::check_for_duplicate;
    use constants::*;

    use crate::test_utilities::*;
    use std::{sync::mpsc, thread};

    use super::new_id;

    #[test]
    fn test_unique_id() {
        let mut ids = vec![];

        // Create some id and check if they are unique
        for _ in 0..NUMBER_OF_LOOPS_FOR_BIG_TEST {
            let id = new_id();
            ids.push(id);
        }

        // Check for duplicates
        check_for_duplicate(&mut ids);
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
                        let id = new_id();
                        ids.push(id);
                        // Send ID value to main thread
                        thread_tx.send(id).unwrap();
                    }
                    // Check for duplicates
                    check_for_duplicate(&mut ids);
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

        // Check for duplicates
        check_for_duplicate(&mut ids);
        assert_eq!(ids.len(), NTHREADS * N_ID_PER_THREAD);
    }
}
