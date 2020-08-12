use std::collections::BinaryHeap;
use std::sync::{Condvar, Mutex, Arc};

use crate::{PriorityQueue, PQError};

/// A simple priority queue that locks a binary heap and uses a condition variable to block while
/// waiting for the next item on the queue
#[derive(Clone, Default)]
pub struct SimplePQ<T: Ord> {
    /// The queue behind a lock
    queue: Arc<Mutex<BinaryHeap<T>>>,
    /// Does the queue have additional items?
    cvar: Arc<Condvar>
}


impl <T: Ord>PriorityQueue<T> for SimplePQ<T> {
    type Error = PQError;

    fn enqueue(&mut self, item: T) -> Result<(), Self::Error> {
        let mut locked = self.queue.lock()?;
        locked.push(item);
        self.cvar.notify_one();
        Ok(())
    }

    fn dequeue(&mut self) -> Result<T, Self::Error> {
        let mut locked = self.cvar.wait_while(self.queue.lock()?, |q| q.is_empty())?;
        locked.pop().ok_or(Self::Error::LockError)
    }
}

#[cfg(test)]
mod tests {
    use crate::{SimplePQ, PriorityQueue};
    use std::{thread, time};
    #[test]
    fn single_thread_t1() {
        let mut q = SimplePQ::<usize>::default();
        q.enqueue(1).unwrap();
        q.enqueue(3).unwrap();
        q.enqueue(2).unwrap();
        assert_eq!(q.dequeue().unwrap(), 3);
        assert_eq!(q.dequeue().unwrap(), 2);
        assert_eq!(q.dequeue().unwrap(), 1);
    }

    #[test]
    fn single_thread_t2() {
        let mut q = SimplePQ::<usize>::default();
        q.enqueue(1).unwrap();
        q.enqueue(3).unwrap();
        assert_eq!(q.dequeue().unwrap(), 3);
        q.enqueue(2).unwrap();
        assert_eq!(q.dequeue().unwrap(), 2);
        assert_eq!(q.dequeue().unwrap(), 1);
    }

    #[test]
    fn multi_thread_1p1c() {
        let mut q = SimplePQ::<usize>::default();
        let mut producer = q.clone();
        let t1 = thread::spawn(move || {
            let wait_time = time::Duration::from_millis(100);
            producer.enqueue(1).unwrap();
            thread::sleep(wait_time.clone());
            producer.enqueue(3).unwrap();
            thread::sleep(wait_time.clone());
            producer.enqueue(2).unwrap();
        });


        let t2 = thread::spawn(move || {
            
            // wait until 1 and 3 are in queue
            let wait_time = time::Duration::from_millis(150);
            thread::sleep(wait_time);

            assert_eq!(q.dequeue().unwrap(), 3);
            assert_eq!(q.dequeue().unwrap(), 1);
            assert_eq!(q.dequeue().unwrap(), 2);
        });
        t1.join().unwrap();
        t2.join().unwrap();

    }
}
