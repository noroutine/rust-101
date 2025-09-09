mod worker;

use std::sync::{Arc, Mutex, mpsc};

use uuid::Uuid;

use crate::worker::{Job, Worker};

#[derive(Debug)]
pub enum PoolCreationError {
    EmptyPool,
    WorkerSpawnFailed,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
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

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            // create some workers
            if let Ok(worker) = Worker::new(id, Arc::clone(&receiver)) {
                workers.push(worker);
            } else {
                return Err(PoolCreationError::WorkerSpawnFailed);
            }
        }

        Ok(ThreadPool {
            workers,
            sender: Some(sender),
        })
    }

    pub fn execute<F>(&self, id: Uuid, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Job {
            id,
            closure: Box::new(f),
        };

        self.sender
            .as_ref()
            .unwrap()
            .send(job)
            .expect("Failed to send the job to worker");
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers.drain(..) {
            println!("Shutting down worker {}", worker.id);

            worker.thread.join().unwrap();
        }
    }
}
