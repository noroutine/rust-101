use std::{any::Any, error::Error, process::ExitCode, thread, time::Duration};

fn main() -> Result<ExitCode, Box<dyn Error>> {
    let thread = "main";

    let bg_thread_hdl = thread::spawn(|| {
        let thread = "bg  ";
        for i in 1..=20 {
            println!("[{thread}] {i}");
            // thread::sleep(Duration::from_millis(100));
        }

        panic!("Thread is panicking!");

        42
    });

    for i in 1..=100 {
        println!("[{thread}] {i}");
        // thread::sleep(Duration::from_millis(100));
    }

    let result  = match bg_thread_hdl.join() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Background thread panicked");
            // Try to extract the panic message
            if let Some(s) = e.downcast_ref::<&str>() {
                eprintln!("Panic message: {}", s);
            } else if let Some(s) = e.downcast_ref::<String>() {
                eprintln!("Panic message: {}", s);
            } else {
                eprintln!("Panic with non-string payload: {:?}", e);
            }

            0
        }
    };
    
    println!("Background thread returned: {}", result);

    Ok(ExitCode::SUCCESS)
}
