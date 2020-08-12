//! This crate provides the definition of multiple different types of MPMC priority queues


/// This is the main trait provided by this crate and defines the functionality a MPMC Priority
/// Queue must provide
pub trait PriorityQueue<T>
    where T: Ord
{
    /// Error type to be returned by priority queues
    /// default implementation use the PQError type
    type Error;
    /// Add a new element to the priority queue
    fn enqueue(&mut self, item: T) -> Result<(), Self::Error>;
    /// Get the next element with the highest priority
    fn dequeue(&mut self) -> Result<T, Self::Error>;
}

mod simple_pq;
mod error;

pub use error::PQError;
pub use simple_pq::SimplePQ;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
