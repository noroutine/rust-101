mod worker;
use worker::Worker;

#[derive(Debug)]
pub enum PoolCreationError {
    EmptyPool,
    WorkerSpawnFailed
}

pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size == 0 {
            return Err(PoolCreationError::EmptyPool);
        }

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            // create some workers
            if let Ok(worker) = Worker::new(id) {
                workers.push(worker);
            } else {
                return Err(PoolCreationError::WorkerSpawnFailed);
            }
        }

        Ok(ThreadPool{ workers })
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static
    {
        f()
    }
}
