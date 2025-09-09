use std::{
    panic::{AssertUnwindSafe, catch_unwind},
    sync::{Arc, Mutex, mpsc::Receiver},
    thread,
    time::Instant,
};

use uuid::Uuid;

pub(crate) struct Job {
    pub(crate) id: Uuid,
    pub(crate) closure: Box<dyn FnOnce() + Send + 'static>,
}

pub(crate) struct Worker {
    pub(crate) id: usize,
    pub(crate) thread: thread::JoinHandle<()>,
}

impl Worker {
    pub(crate) fn new(
        id: usize,
        receiver: Arc<Mutex<Receiver<Job>>>,
    ) -> Result<Worker, std::io::Error> {
        let builder = thread::Builder::new();

        let thread = builder.spawn(move || {
            'outer: loop {
                let job = {
                    let job_receiver = receiver.lock().unwrap_or_else(|poisoned| {
                        // Log the issue and continue with potentially corrupted data
                        eprintln!("Mutex was poisoned, continuing anyway");
                        poisoned.into_inner()
                    });

                    let job = match job_receiver.recv() {
                        Ok(job) => job,
                        Err(_) => {
                            println!("Worker {id} disconnected, shutting down");
                            break 'outer;
                        }
                    };

                    job
                }; // drop mutext guard

                if let Err(_) = catch_unwind(AssertUnwindSafe(|| {
                    println!("Worker.{id}: {} Taken ", job.id);
                    let start = Instant::now();

                    (job.closure)();

                    let execution_time = start.elapsed();
                    let nanos = execution_time.as_nanos();
                    let formatted_time = match nanos {
                        0..=999 => format!("{}ns", nanos),
                        1_000..=999_999 => format!("{:.1}μs", nanos as f64 / 1_000.0),
                        1_000_000..=999_999_999 => format!("{:.1}ms", nanos as f64 / 1_000_000.0),
                        _ => format!("{:.2}s", execution_time.as_secs_f64()),
                    };

                    println!("Worker.{id}: {} Done in {}", job.id, formatted_time);
                })) {
                    eprintln!("Worker.{id}: Job panicked, but worker continues");
                }
            }
        })?;

        Ok(Worker { id, thread })
    }
}
