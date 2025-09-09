use std::{
    sync::{Arc, Mutex, mpsc::Receiver},
    thread,
};

pub(crate) type Job = Box<dyn FnOnce() + Send + 'static>;

pub(crate) struct Worker {
    id: usize,
    thread_handle: thread::JoinHandle<()>,
}

impl Worker {
    pub(crate) fn new(
        id: usize,
        receiver: Arc<Mutex<Receiver<Job>>>,
    ) -> Result<Worker, std::io::Error> {
        let builder = thread::Builder::new();

        let thread_handle = builder.spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();

                println!("Worker {id} got a job; executing.");

                job();
            }
        })?;

        Ok(Worker { id, thread_handle })
    }
}
