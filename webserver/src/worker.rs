use std::thread;

pub struct Worker {
    id: usize,
    thread_handle: thread::JoinHandle<()>,
}

impl Worker {
    pub fn new(id: usize) -> Result<Worker, std::io::Error> {
        let builder = thread::Builder::new();

        let thread_handle = builder.spawn(|| {
            // thread code
        })?;

        Ok(Worker { id, thread_handle })
    }
}
